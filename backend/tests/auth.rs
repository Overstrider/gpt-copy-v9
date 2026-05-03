mod support;

use std::sync::Arc;

use axum::http::{
    HeaderValue, StatusCode,
    header::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, AUTHORIZATION, ORIGIN, VARY,
    },
};
use axum_test::TestServer;
use gpt_copy_v9::openrouter::FakeOpenRouterClient;

#[tokio::test]
async fn api_routes_require_bearer_token_when_configured() {
    let pool = support::test_pool().await;
    let mut config = gpt_copy_v9::config::Config::for_test();
    config.api_auth_token = "test-token".to_string();
    let openrouter = Arc::new(FakeOpenRouterClient { responses: vec![] });
    let app = gpt_copy_v9::build_app_with_client(config, pool, openrouter).await;
    let mut server = TestServer::new(app).unwrap();

    let health = server.get("/health").await;
    health.assert_status_ok();

    let missing_auth = server.get("/api/conversations").await;
    missing_auth.assert_status(StatusCode::UNAUTHORIZED);
    let body: serde_json::Value = missing_auth.json();
    assert_eq!(body["message"], "Unauthorized");

    server.add_header(AUTHORIZATION, HeaderValue::from_static("Bearer test-token"));
    let authorized = server.get("/api/conversations").await;
    authorized.assert_status_ok();
}

#[tokio::test]
async fn cors_preflight_allows_json_and_authorization_headers() {
    let pool = support::test_pool().await;
    let mut config = gpt_copy_v9::config::Config::for_test();
    config.api_auth_token = "test-token".to_string();
    let openrouter = Arc::new(FakeOpenRouterClient { responses: vec![] });
    let app = gpt_copy_v9::build_app_with_client(config, pool, openrouter).await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .method(axum::http::Method::OPTIONS, "/api/conversations")
        .add_header(ORIGIN, HeaderValue::from_static("http://localhost:3000"))
        .add_header(
            axum::http::header::ACCESS_CONTROL_REQUEST_METHOD,
            HeaderValue::from_static("POST"),
        )
        .add_header(
            axum::http::header::ACCESS_CONTROL_REQUEST_HEADERS,
            HeaderValue::from_static("content-type, authorization"),
        )
        .await;

    response.assert_status_ok();
    let headers = response.headers();
    let allow_headers = headers
        .get(ACCESS_CONTROL_ALLOW_HEADERS)
        .expect("access-control-allow-headers")
        .to_str()
        .unwrap()
        .to_ascii_lowercase();
    assert!(allow_headers.contains("content-type"));
    assert!(allow_headers.contains("authorization"));

    if let Some(credentials) = headers.get(ACCESS_CONTROL_ALLOW_CREDENTIALS) {
        assert_eq!(credentials.to_str().unwrap(), "false");
    }

    let vary = headers
        .get(VARY)
        .expect("vary")
        .to_str()
        .unwrap()
        .to_ascii_lowercase();
    assert!(vary.split(',').any(|part| part.trim() == "origin"));
}
