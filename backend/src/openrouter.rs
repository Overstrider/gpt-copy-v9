use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug)]
pub enum StreamEvent {
    Delta(String),
    Done,
}

#[async_trait]
pub trait OpenRouterClient {
    async fn stream_chat(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AppError>>, AppError>;
}

// ── Real HTTP client ──────────────────────────────────────────────────────────

pub struct HttpOpenRouterClient {
    client: Client,
    api_key: String,
}

impl HttpOpenRouterClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
        }
    }
}

#[derive(Serialize)]
struct OpenRouterRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    stream: bool,
}

#[async_trait]
impl OpenRouterClient for HttpOpenRouterClient {
    async fn stream_chat(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AppError>>, AppError> {
        use futures_util::StreamExt;

        let req_body = OpenRouterRequest {
            model,
            messages: &messages,
            stream: true,
        };

        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&req_body)
            .send()
            .await
            .map_err(|e| AppError::OpenRouter(format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(AppError::OpenRouter(format!(
                "OpenRouter returned status {}",
                response.status()
            )));
        }

        let stream = response.bytes_stream().map(move |chunk_res| {
            let chunk = chunk_res.map_err(|e| AppError::OpenRouter(e.to_string()))?;
            let text = String::from_utf8_lossy(&chunk);
            for line in text.lines() {
                let line = line.trim();
                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        return Ok(StreamEvent::Done);
                    }
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(delta) = val
                            .pointer("/choices/0/delta/content")
                            .and_then(|v| v.as_str())
                        {
                            return Ok(StreamEvent::Delta(delta.to_string()));
                        }
                    }
                }
            }
            Ok(StreamEvent::Done)
        });

        Ok(Box::pin(stream))
    }
}

// ── Fake client for tests ─────────────────────────────────────────────────────

pub struct FakeOpenRouterClient {
    pub responses: Vec<String>,
}

#[async_trait]
impl OpenRouterClient for FakeOpenRouterClient {
    async fn stream_chat(
        &self,
        _model: &str,
        _messages: Vec<ChatMessage>,
    ) -> Result<BoxStream<'static, Result<StreamEvent, AppError>>, AppError> {
        use futures_util::stream;
        let mut events: Vec<Result<StreamEvent, AppError>> = self
            .responses
            .iter()
            .map(|s| Ok(StreamEvent::Delta(s.clone())))
            .collect();
        events.push(Ok(StreamEvent::Done));
        Ok(Box::pin(stream::iter(events)))
    }
}
