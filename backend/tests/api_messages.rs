mod support;
use axum_test::TestServer;
use support::{test_app, test_server};

async fn create_conv(server: &TestServer, title: &str) -> String {
    let resp = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": title }))
        .await;
    let body: serde_json::Value = resp.json();
    body["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_list_messages_empty() {
    let app = test_app(vec![]).await;
    let server = test_server(app);
    let id = create_conv(&server, "Chat").await;
    let resp = server
        .get(&format!("/api/conversations/{id}/messages"))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_create_message() {
    let app = test_app(vec![]).await;
    let server = test_server(app);
    let id = create_conv(&server, "Chat").await;
    let resp = server
        .post(&format!("/api/conversations/{id}/messages"))
        .json(&serde_json::json!({ "content": "Hello" }))
        .await;
    resp.assert_status(axum::http::StatusCode::CREATED);
    let body: serde_json::Value = resp.json();
    assert_eq!(body["role"], "user");
    assert_eq!(body["content"], "Hello");
}

#[tokio::test]
async fn test_create_message_empty_content() {
    let app = test_app(vec![]).await;
    let server = test_server(app);
    let id = create_conv(&server, "Chat").await;
    let resp = server
        .post(&format!("/api/conversations/{id}/messages"))
        .json(&serde_json::json!({ "content": "   " }))
        .await;
    resp.assert_status(axum::http::StatusCode::BAD_REQUEST);
    let body: serde_json::Value = resp.json();
    assert_eq!(body["code"], "BAD_REQUEST");
}

#[tokio::test]
async fn test_list_messages_for_unknown_conversation() {
    let app = test_app(vec![]).await;
    let server = test_server(app);
    let resp = server.get("/api/conversations/nonexistent/messages").await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_message() {
    let app = test_app(vec![]).await;
    let server = test_server(app);
    let id = create_conv(&server, "Chat").await;
    let msg_resp = server
        .post(&format!("/api/conversations/{id}/messages"))
        .json(&serde_json::json!({ "content": "Hello" }))
        .await;
    let msg: serde_json::Value = msg_resp.json();
    let msg_id = msg["id"].as_str().unwrap();

    let del = server
        .delete(&format!("/api/conversations/{id}/messages/{msg_id}"))
        .await;
    del.assert_status(axum::http::StatusCode::NO_CONTENT);

    let list = server
        .get(&format!("/api/conversations/{id}/messages"))
        .await;
    let msgs: serde_json::Value = list.json();
    assert!(msgs.as_array().unwrap().is_empty());
}
