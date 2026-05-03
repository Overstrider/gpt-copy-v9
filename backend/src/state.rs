use std::sync::Arc;

use sqlx::SqlitePool;

use crate::config::Config;
use crate::openrouter::OpenRouterClient;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub openrouter: Arc<dyn OpenRouterClient + Send + Sync>,
}
