mod support;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum_test::TestServer;
use futures_util::stream::{self, BoxStream};
use gpt_copy_v9::error::AppError;
use gpt_copy_v9::openrouter::{ChatMessage, OpenRouterClient, StreamEvent};
use support::test_app;

struct FailingOpenRouterClient;

#[async_trait]
impl OpenRouterClient for FailingOpenRouterClient {
    async fn stream_chat(
        &self,
        _model: &str,
        _messages: Vec<ChatMessage>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AppError>>, AppError> {
        Err(AppError::OpenRouter(
            "secret upstream open failure".to_string(),
        ))
    }
}

struct ErroringStreamClient;

#[async_trait]
impl OpenRouterClient for ErroringStreamClient {
    async fn stream_chat(
        &self,
        _model: &str,
        _messages: Vec<ChatMessage>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AppError>>, AppError> {
        Ok(Box::pin(stream::iter([
            Ok(StreamEvent::Delta("partial".to_string())),
            Err(AppError::OpenRouter(
                "secret upstream stream failure".to_string(),
            )),
        ])))
    }
}

struct NoDoneStreamClient;

#[async_trait]
impl OpenRouterClient for NoDoneStreamClient {
    async fn stream_chat(
        &self,
        _model: &str,
        _messages: Vec<ChatMessage>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AppError>>, AppError> {
        Ok(Box::pin(stream::iter([Ok(StreamEvent::Delta(
            "clean eof".to_string(),
        ))])))
    }
}

struct CapturingOpenRouterClient {
    calls: Arc<Mutex<Vec<Vec<ChatMessage>>>>,
}

#[async_trait]
impl OpenRouterClient for CapturingOpenRouterClient {
    async fn stream_chat(
        &self,
        _model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AppError>>, AppError> {
        self.calls.lock().unwrap().push(messages);
        Ok(Box::pin(stream::iter([Ok(StreamEvent::Done)])))
    }
}

async fn test_app_with_client(
    openrouter: Arc<dyn OpenRouterClient + Send + Sync>,
) -> (axum::Router, sqlx::SqlitePool) {
    let pool = support::test_pool().await;
    let config = gpt_copy_v9::config::Config::for_test();
    let app = gpt_copy_v9::build_app_with_client(config, pool.clone(), openrouter).await;
    (app, pool)
}

#[tokio::test]
async fn test_stream_chat_validates_empty_content() {
    let app = test_app(vec!["Hello".to_string()]).await;
    let server = TestServer::new(app).unwrap();

    // Create a conversation first
    let create = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "Stream Test" }))
        .await;
    let conv: serde_json::Value = create.json();
    let id = conv["id"].as_str().unwrap();

    // Empty content should fail
    let resp = server
        .post(&format!("/api/conversations/{id}/stream"))
        .json(&serde_json::json!({ "content": "" }))
        .await;
    resp.assert_status(axum::http::StatusCode::BAD_REQUEST);
    let body: serde_json::Value = resp.json();
    assert_eq!(body["code"], "BAD_REQUEST");
}

#[tokio::test]
async fn test_stream_chat_unknown_conversation() {
    let app = test_app(vec!["Hello".to_string()]).await;
    let server = TestServer::new(app).unwrap();

    let resp = server
        .post("/api/conversations/nonexistent/stream")
        .json(&serde_json::json!({ "content": "Hi" }))
        .await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_stream_persists_user_message() {
    let app = test_app(vec!["World".to_string()]).await;
    let server = TestServer::new(app).unwrap();

    let create = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "Stream Test" }))
        .await;
    let conv: serde_json::Value = create.json();
    let id = conv["id"].as_str().unwrap();

    // Do the stream request
    let stream_resp = server
        .post(&format!("/api/conversations/{id}/stream"))
        .json(&serde_json::json!({ "content": "Hello" }))
        .await;
    stream_resp.assert_status_ok();
    let stream_body = stream_resp.text();
    assert!(stream_body.contains("\"type\":\"done\""));

    // Check messages were persisted
    let msgs_resp = server
        .get(&format!("/api/conversations/{id}/messages"))
        .await;
    msgs_resp.assert_status_ok();
    let msgs: serde_json::Value = msgs_resp.json();
    let arr = msgs.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["role"], "user");
    assert_eq!(arr[0]["content"], "Hello");
    assert_eq!(arr[1]["role"], "assistant");
    assert_eq!(arr[1]["content"], "World");
}

#[tokio::test]
async fn test_stream_open_failure_does_not_persist_user_message() {
    let (app, pool) = test_app_with_client(Arc::new(FailingOpenRouterClient)).await;
    let server = TestServer::new(app).unwrap();

    let create = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "Stream Test" }))
        .await;
    let conv: serde_json::Value = create.json();
    let id = conv["id"].as_str().unwrap();

    let resp = server
        .post(&format!("/api/conversations/{id}/stream"))
        .json(&serde_json::json!({ "content": "Hello" }))
        .await;
    resp.assert_status(axum::http::StatusCode::BAD_GATEWAY);

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages WHERE conversation_id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 0);
}

#[tokio::test]
async fn test_stream_error_is_sanitized_and_does_not_persist_partial_assistant() {
    let (app, _pool) = test_app_with_client(Arc::new(ErroringStreamClient)).await;
    let server = TestServer::new(app).unwrap();

    let create = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "Stream Test" }))
        .await;
    let conv: serde_json::Value = create.json();
    let id = conv["id"].as_str().unwrap();

    let stream_resp = server
        .post(&format!("/api/conversations/{id}/stream"))
        .json(&serde_json::json!({ "content": "Hello" }))
        .await;
    stream_resp.assert_status_ok();
    let stream_body = stream_resp.text();
    assert!(stream_body.contains("Upstream model service error"));
    assert!(!stream_body.contains("secret upstream stream failure"));

    let msgs_resp = server
        .get(&format!("/api/conversations/{id}/messages"))
        .await;
    msgs_resp.assert_status_ok();
    let msgs: serde_json::Value = msgs_resp.json();
    let arr = msgs.as_array().unwrap();
    assert_eq!(arr.len(), 0);
}

#[tokio::test]
async fn test_stream_clean_eof_persists_message_pair() {
    let (app, _pool) = test_app_with_client(Arc::new(NoDoneStreamClient)).await;
    let server = TestServer::new(app).unwrap();

    let create = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "Stream Test" }))
        .await;
    let conv: serde_json::Value = create.json();
    let id = conv["id"].as_str().unwrap();

    let stream_resp = server
        .post(&format!("/api/conversations/{id}/stream"))
        .json(&serde_json::json!({ "content": "Hello" }))
        .await;
    stream_resp.assert_status_ok();
    let stream_body = stream_resp.text();
    assert!(stream_body.contains("clean eof"));

    let msgs_resp = server
        .get(&format!("/api/conversations/{id}/messages"))
        .await;
    msgs_resp.assert_status_ok();
    let msgs: serde_json::Value = msgs_resp.json();
    let arr = msgs.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["role"], "user");
    assert_eq!(arr[0]["content"], "Hello");
    assert_eq!(arr[1]["role"], "assistant");
    assert_eq!(arr[1]["content"], "clean eof");
}

#[tokio::test]
async fn test_stream_sends_only_recent_history_to_openrouter() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(CapturingOpenRouterClient {
        calls: calls.clone(),
    });
    let (app, pool) = test_app_with_client(client).await;
    let server = TestServer::new(app).unwrap();

    let create = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "Stream Test" }))
        .await;
    let conv: serde_json::Value = create.json();
    let id = conv["id"].as_str().unwrap();

    for i in 0..45 {
        gpt_copy_v9::repository::create_message(&pool, id, "user", &format!("old-{i}"))
            .await
            .unwrap();
    }

    let stream_resp = server
        .post(&format!("/api/conversations/{id}/stream"))
        .json(&serde_json::json!({ "content": "current" }))
        .await;
    stream_resp.assert_status_ok();
    let _ = stream_resp.text();

    let calls = calls.lock().unwrap();
    let messages = calls.first().unwrap();
    assert_eq!(messages.len(), 41);
    assert_eq!(messages.first().unwrap().content, "old-5");
    assert_eq!(messages.last().unwrap().content, "current");
}
