use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::error::AppError;

const MAX_SSE_LINE_BYTES: usize = 1_048_576;
const MAX_OPENROUTER_OUTPUT_TOKENS: u32 = 4_096;

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
            client: Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(120))
                .build()
                .expect("reqwest client"),
            api_key: api_key.into(),
        }
    }
}

#[derive(Serialize)]
struct OpenRouterRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    stream: bool,
    max_tokens: u32,
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
            max_tokens: MAX_OPENROUTER_OUTPUT_TOKENS,
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

        // Use a stateful line-buffer so a single HTTP chunk can yield multiple
        // StreamEvents and we never emit a spurious Done for a non-terminal chunk.
        use futures_util::stream;
        let byte_stream = response.bytes_stream();

        let sse_stream = stream::unfold(
            (byte_stream, String::new(), false),
            |(mut byte_stream, mut buf, terminal_emitted)| async move {
                loop {
                    // Drain any complete lines already in the buffer.
                    while let Some(pos) = buf.find('\n') {
                        let line: String = buf.drain(..=pos).collect();
                        let line = line.trim();
                        if let Some(data) = line.strip_prefix("data: ") {
                            if data == "[DONE]" {
                                return Some((
                                    vec![Ok(StreamEvent::Done)],
                                    (byte_stream, buf, true),
                                ));
                            }
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
                                if let Some(delta) = val
                                    .pointer("/choices/0/delta/content")
                                    .and_then(|v| v.as_str())
                                {
                                    let delta = delta.to_string();
                                    return Some((
                                        vec![Ok(StreamEvent::Delta(delta))],
                                        (byte_stream, buf, terminal_emitted),
                                    ));
                                }
                            }
                        }
                    }

                    // Need more bytes.
                    use futures_util::StreamExt;
                    match byte_stream.next().await {
                        Some(Ok(chunk)) => {
                            if buf.len() + chunk.len() > MAX_SSE_LINE_BYTES {
                                return Some((
                                    vec![Err(AppError::OpenRouter(
                                        "SSE event exceeded maximum line length".to_string(),
                                    ))],
                                    (byte_stream, String::new(), true),
                                ));
                            }
                            buf.push_str(&String::from_utf8_lossy(&chunk));
                        }
                        Some(Err(e)) => {
                            return Some((
                                vec![Err(AppError::OpenRouter(e.to_string()))],
                                (byte_stream, buf, true),
                            ));
                        }
                        None => {
                            if !terminal_emitted {
                                return Some((
                                    vec![Ok(StreamEvent::Done)],
                                    (byte_stream, buf, true),
                                ));
                            }
                            return None;
                        }
                    }
                }
            },
        )
        .flat_map(|events| stream::iter(events));

        Ok(Box::pin(sse_stream))
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
