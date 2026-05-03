#![allow(dead_code)]

use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use std::sync::Arc;

use axum::http::{HeaderValue, header::AUTHORIZATION};
use axum_test::TestServer;
use gpt_copy_v9::config::Config;
use gpt_copy_v9::openrouter::FakeOpenRouterClient;
use gpt_copy_v9::state::{AppState, StreamRegistry};

pub const TEST_API_AUTH_TOKEN: &str = "test-token";
pub const TEST_OWNER_ID: &str = "4c5dc9b7708905f77f5e5d16316b5dfb425e68cb326dcd55a860e90a7707031e";

/// Create a fresh in-memory SQLite pool with migrations applied.
pub async fn test_pool() -> SqlitePool {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")
        .expect("sqlite options")
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .expect("in-memory pool");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations");
    pool
}

/// Build a test AppState with fake OpenRouter client.
pub async fn test_state(fake_responses: Vec<String>) -> AppState {
    let pool = test_pool().await;
    let config = Config::for_test();
    let openrouter = Arc::new(FakeOpenRouterClient {
        responses: fake_responses,
    });
    AppState {
        db: pool,
        config,
        openrouter,
        stream_registry: StreamRegistry::default(),
    }
}

/// Build the full Axum app for integration tests.
pub async fn test_app(fake_responses: Vec<String>) -> axum::Router {
    let state = test_state(fake_responses).await;
    gpt_copy_v9::build_app_with_client(
        state.config.clone(),
        state.db.clone(),
        state.openrouter.clone(),
    )
    .await
}

pub fn test_server(app: axum::Router) -> TestServer {
    let mut server = TestServer::new(app).unwrap();
    server.add_header(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {TEST_API_AUTH_TOKEN}")).unwrap(),
    );
    server
}
