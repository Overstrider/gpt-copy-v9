use std::{collections::HashSet, sync::Arc};

use sqlx::SqlitePool;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::openrouter::OpenRouterClient;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub openrouter: Arc<dyn OpenRouterClient + Send + Sync>,
    pub stream_registry: StreamRegistry,
}

#[derive(Clone, Default)]
pub struct StreamRegistry {
    in_flight: Arc<Mutex<HashSet<String>>>,
}

impl StreamRegistry {
    pub async fn try_acquire(&self, conversation_id: &str) -> bool {
        self.in_flight
            .lock()
            .await
            .insert(conversation_id.to_string())
    }

    pub async fn release(&self, conversation_id: &str) {
        self.in_flight.lock().await.remove(conversation_id);
    }
}
