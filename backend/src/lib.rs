pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod openrouter;
pub mod repository;
pub mod routes;
pub mod state;
pub mod validation;

use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use config::Config;
use openrouter::HttpOpenRouterClient;
use state::AppState;

pub async fn build_app(config: Config, db: sqlx::SqlitePool) -> Router {
    let openrouter: Arc<dyn openrouter::OpenRouterClient + Send + Sync> =
        Arc::new(HttpOpenRouterClient::new(config.openrouter_api_key.clone()));

    build_app_with_client(config, db, openrouter).await
}

pub async fn build_app_with_client(
    config: Config,
    db: sqlx::SqlitePool,
    openrouter: Arc<dyn openrouter::OpenRouterClient + Send + Sync>,
) -> Router {
    let state = AppState {
        db,
        config,
        openrouter,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(routes::health::health_handler))
        .route(
            "/api/conversations",
            get(routes::conversations::list_conversations)
                .post(routes::conversations::create_conversation),
        )
        .route(
            "/api/conversations/:id",
            get(routes::conversations::get_conversation)
                .patch(routes::conversations::update_conversation)
                .delete(routes::conversations::delete_conversation),
        )
        .route(
            "/api/conversations/:conversation_id/messages",
            get(routes::messages::list_messages).post(routes::messages::create_message),
        )
        .route(
            "/api/conversations/:conversation_id/messages/:message_id",
            delete(routes::messages::delete_message),
        )
        .route(
            "/api/conversations/:conversation_id/stream",
            post(routes::chat::stream_chat),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
