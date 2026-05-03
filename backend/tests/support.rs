use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use std::sync::Arc;

use gpt_copy_v9::config::Config;
use gpt_copy_v9::openrouter::FakeOpenRouterClient;
use gpt_copy_v9::state::{AppState, StreamRegistry};

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
