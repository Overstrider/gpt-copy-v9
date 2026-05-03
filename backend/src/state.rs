use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use sqlx::SqlitePool;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::config::Config;
use crate::openrouter::OpenRouterClient;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub openrouter: Arc<dyn OpenRouterClient + Send + Sync>,
    pub stream_registry: StreamRegistry,
}

const DEFAULT_MAX_CONCURRENT_STREAMS: usize = 8;

#[derive(Clone)]
pub struct StreamRegistry {
    in_flight: Arc<Mutex<HashSet<String>>>,
    slots: Arc<Semaphore>,
}

impl Default for StreamRegistry {
    fn default() -> Self {
        Self::with_max_concurrent(DEFAULT_MAX_CONCURRENT_STREAMS)
    }
}

impl StreamRegistry {
    pub fn with_max_concurrent(max_concurrent: usize) -> Self {
        Self {
            in_flight: Arc::new(Mutex::new(HashSet::new())),
            slots: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub fn try_acquire(&self, conversation_id: &str) -> Option<StreamLease> {
        let permit = self.slots.clone().try_acquire_owned().ok()?;
        let mut in_flight = self
            .in_flight
            .lock()
            .expect("stream registry mutex poisoned");
        if !in_flight.insert(conversation_id.to_string()) {
            return None;
        }

        Some(StreamLease {
            registry: self.clone(),
            conversation_id: conversation_id.to_string(),
            _permit: permit,
        })
    }

    fn release(&self, conversation_id: &str) {
        self.in_flight
            .lock()
            .expect("stream registry mutex poisoned")
            .remove(conversation_id);
    }
}

pub struct StreamLease {
    registry: StreamRegistry,
    conversation_id: String,
    _permit: OwnedSemaphorePermit,
}

impl Drop for StreamLease {
    fn drop(&mut self) {
        self.registry.release(&self.conversation_id);
    }
}
