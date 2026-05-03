// Implemented in TASK-007
use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::validation::validate_message_content;

#[derive(Deserialize)]
pub struct StreamRequest {
    pub content: String,
}

const MAX_CONTEXT_MESSAGES: usize = 40;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum SsePayload<'a> {
    Delta { content: &'a str },
    Done,
    Error { message: &'a str },
}

pub async fn stream_chat(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Json(body): Json<StreamRequest>,
) -> AppResult<Response> {
    validate_message_content(&body.content)?;
    // Ensure conversation exists
    crate::repository::get_conversation(&state.db, &conversation_id).await?;

    if !state.stream_registry.try_acquire(&conversation_id).await {
        return Err(AppError::Conflict(
            "A stream is already in progress for this conversation".to_string(),
        ));
    }
    let stream_registry = state.stream_registry.clone();
    let locked_conversation_id = conversation_id.clone();

    // Get conversation history
    let history = match crate::repository::list_messages(&state.db, &conversation_id).await {
        Ok(history) => history,
        Err(e) => {
            stream_registry.release(&locked_conversation_id).await;
            return Err(e);
        }
    };
    let history_start = history.len().saturating_sub(MAX_CONTEXT_MESSAGES);
    let mut messages: Vec<crate::openrouter::ChatMessage> = history[history_start..]
        .iter()
        .map(|m| crate::openrouter::ChatMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();
    messages.push(crate::openrouter::ChatMessage {
        role: "user".to_string(),
        content: body.content.clone(),
    });

    let db = state.db.clone();
    let conv_id = conversation_id.clone();

    let stream = match state
        .openrouter
        .stream_chat(&state.config.openrouter_model, messages)
        .await
    {
        Ok(stream) => stream,
        Err(e) => {
            stream_registry.release(&locked_conversation_id).await;
            return Err(AppError::OpenRouter(e.to_string()));
        }
    };

    use futures_util::StreamExt;
    use std::sync::{Arc, Mutex};

    let buffer = Arc::new(Mutex::new(String::new()));
    let buffer_clone = buffer.clone();

    let sse_stream = stream.map(move |event| {
        let chunk = match &event {
            Ok(crate::openrouter::StreamEvent::Delta(delta)) => {
                buffer_clone.lock().unwrap().push_str(delta);
                sse_frame(&SsePayload::Delta { content: delta })
            }
            Ok(crate::openrouter::StreamEvent::Done) => sse_frame(&SsePayload::Done),
            Err(e) => {
                tracing::error!("Streaming chat error: {e}");
                sse_frame(&SsePayload::Error {
                    message: stream_error_message(e),
                })
            }
        };
        Ok::<_, std::convert::Infallible>(chunk)
    });

    // Collect and persist after stream
    let (tx, rx) = tokio::sync::mpsc::channel::<Result<String, std::convert::Infallible>>(64);

    let db2 = db.clone();
    let conv_id2 = conv_id.clone();
    let buf_final = buffer.clone();
    let user_content = body.content.clone();
    let release_registry = stream_registry.clone();
    let release_conversation_id = locked_conversation_id.clone();

    tokio::spawn(async move {
        use futures_util::StreamExt;
        let mut s = Box::pin(sse_stream);
        let mut completed = false;
        let mut receiver_open = true;
        let mut stream_failed = false;
        while let Some(item) = s.next().await {
            let (is_done, is_error) = item
                .as_ref()
                .map(|sse| {
                    (
                        sse_type_matches(sse, "done"),
                        sse_type_matches(sse, "error"),
                    )
                })
                .unwrap_or((false, false));
            if is_done {
                completed = true;
                break;
            }
            if tx.send(item).await.is_err() {
                receiver_open = false;
                break;
            }
            if is_error {
                stream_failed = true;
                break;
            }
        }
        let content = buf_final.lock().unwrap().clone();
        if !completed && receiver_open && !stream_failed && !content.is_empty() {
            tracing::warn!("Streaming chat ended without a done event; skipping persistence");
        }
        if completed {
            let terminal = match crate::repository::create_user_assistant_message_pair(
                &db2,
                &conv_id2,
                &user_content,
                &content,
            )
            .await
            {
                Ok(()) => sse_frame(&SsePayload::Done),
                Err(e) => {
                    tracing::error!("Failed to persist streamed message pair: {e}");
                    sse_frame(&SsePayload::Error {
                        message: "Failed to save streamed response",
                    })
                }
            };
            let _ = tx.send(Ok(terminal)).await;
        }
        release_registry.release(&release_conversation_id).await;
    });

    let body_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let body = Body::from_stream(body_stream);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(response)
}

fn stream_error_message(error: &AppError) -> &'static str {
    match error {
        AppError::OpenRouter(_) => "Upstream model service error",
        AppError::Database(_) => "A database error occurred",
        AppError::Internal(_) => "An internal server error occurred",
        AppError::NotFound(_) | AppError::BadRequest(_) | AppError::Conflict(_) => "Stream error",
    }
}

fn sse_frame(payload: &SsePayload<'_>) -> String {
    let json = serde_json::to_string(payload).expect("serialize SSE payload");
    format!("data: {json}\n\n")
}

fn sse_type_matches(sse: &str, expected: &str) -> bool {
    sse.strip_prefix("data: ")
        .or_else(|| sse.lines().find_map(|line| line.strip_prefix("data: ")))
        .and_then(|data| serde_json::from_str::<serde_json::Value>(data).ok())
        .and_then(|value| {
            value
                .get("type")
                .and_then(|event_type| event_type.as_str())
                .map(|event_type| event_type == expected)
        })
        .unwrap_or(false)
}
