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
    extract::{Request, State},
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post},
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use config::Config;
use error::{AppError, AppResult};
use openrouter::HttpOpenRouterClient;
use state::{AppState, StreamRegistry};

const MAX_CONCURRENT_STREAMS: usize = 8;

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
    let frontend_origin = HeaderValue::from_str(&config.frontend_origin)
        .expect("FRONTEND_ORIGIN must be a valid HTTP header value");

    let state = AppState {
        db,
        config,
        openrouter,
        stream_registry: StreamRegistry::with_max_concurrent(MAX_CONCURRENT_STREAMS),
    };

    let cors = CorsLayer::new()
        .allow_origin(frontend_origin)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    let api_routes = Router::new()
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
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_api_auth,
        ));

    Router::new()
        .route("/health", get(routes::health::health_handler))
        .merge(api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn require_api_auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> AppResult<Response> {
    let Some(expected_token) = state.config.api_auth_token.as_deref() else {
        return Ok(next.run(req).await);
    };

    let provided_token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    if provided_token != Some(expected_token) {
        return Err(AppError::Unauthorized(
            "Invalid or missing API token".to_string(),
        ));
    }

    Ok(next.run(req).await)
}
