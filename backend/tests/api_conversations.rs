mod support;
use axum_test::TestServer;
use support::test_app;

#[tokio::test]
async fn test_list_conversations_empty() {
    let app = test_app(vec![]).await;
    let server = TestServer::new(app).unwrap();
    let resp = server.get("/api/conversations").await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_create_conversation() {
    let app = test_app(vec![]).await;
    let server = TestServer::new(app).unwrap();
    let resp = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "My Chat" }))
        .await;
    resp.assert_status(axum::http::StatusCode::CREATED);
    let body: serde_json::Value = resp.json();
    assert_eq!(body["title"], "My Chat");
    assert!(body["id"].is_string());
}

#[tokio::test]
async fn test_get_conversation_not_found() {
    let app = test_app(vec![]).await;
    let server = TestServer::new(app).unwrap();
    let resp = server.get("/api/conversations/nonexistent").await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
    let body: serde_json::Value = resp.json();
    assert_eq!(body["code"], "NOT_FOUND");
}

#[tokio::test]
async fn test_update_conversation() {
    let app = test_app(vec![]).await;
    let server = TestServer::new(app).unwrap();
    let create = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "Original" }))
        .await;
    let created: serde_json::Value = create.json();
    let id = created["id"].as_str().unwrap();

    let resp = server
        .patch(&format!("/api/conversations/{id}"))
        .json(&serde_json::json!({ "title": "Updated" }))
        .await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert_eq!(body["title"], "Updated");
}

#[tokio::test]
async fn test_delete_conversation() {
    let app = test_app(vec![]).await;
    let server = TestServer::new(app).unwrap();
    let create = server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "To Delete" }))
        .await;
    let created: serde_json::Value = create.json();
    let id = created["id"].as_str().unwrap();

    let resp = server.delete(&format!("/api/conversations/{id}")).await;
    resp.assert_status(axum::http::StatusCode::NO_CONTENT);

    let get_resp = server.get(&format!("/api/conversations/{id}")).await;
    get_resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_create_conversation_default_title() {
    let app = test_app(vec![]).await;
    let server = TestServer::new(app).unwrap();
    let resp = server
        .post("/api/conversations")
        .json(&serde_json::json!({}))
        .await;
    resp.assert_status(axum::http::StatusCode::CREATED);
    let body: serde_json::Value = resp.json();
    assert_eq!(body["title"], "New Chat");
}

#[tokio::test]
async fn test_list_conversations_ordered() {
    let app = test_app(vec![]).await;
    let server = TestServer::new(app).unwrap();
    server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "First" }))
        .await;
    server
        .post("/api/conversations")
        .json(&serde_json::json!({ "title": "Second" }))
        .await;
    let resp = server.get("/api/conversations").await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    let arr = body.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["title"], "Second");
    assert_eq!(arr[1]["title"], "First");
}
