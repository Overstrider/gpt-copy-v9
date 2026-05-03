mod support;
use axum_test::TestServer;
use support::test_app;

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
    let _stream_resp = server
        .post(&format!("/api/conversations/{id}/stream"))
        .json(&serde_json::json!({ "content": "Hello" }))
        .await;

    // Check messages were persisted
    let msgs_resp = server
        .get(&format!("/api/conversations/{id}/messages"))
        .await;
    msgs_resp.assert_status_ok();
    let msgs: serde_json::Value = msgs_resp.json();
    let arr = msgs.as_array().unwrap();
    // At minimum the user message should exist
    assert!(!arr.is_empty());
    assert_eq!(arr[0]["role"], "user");
    assert_eq!(arr[0]["content"], "Hello");
}
