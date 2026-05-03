use axum::{
    Extension, Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use crate::state::{AppState, AuthenticatedUser};
use crate::validation::validate_message_content;

#[derive(Deserialize)]
pub struct StreamRequest {
    pub content: String,
}

// 40 messages is roughly 20 turns, keeping routine chats within context and payload limits.
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
    Extension(user): Extension<AuthenticatedUser>,
    Path(conversation_id): Path<String>,
    Json(body): Json<StreamRequest>,
) -> AppResult<Response> {
    validate_message_content(&body.content)?;
    // Ensure conversation exists
    crate::repository::get_conversation(&state.db, &conversation_id, &user.id).await?;

    let stream_guard = state
        .stream_registry
        .try_acquire(&conversation_id)
        .ok_or_else(|| {
            AppError::Conflict(
                "A stream is already in progress or stream capacity is full".to_string(),
            )
        })?;

    // Get conversation history
    let history =
        match crate::repository::list_messages(&state.db, &conversation_id, &user.id).await {
            Ok(history) => history,
            Err(e) => {
                drop(stream_guard);
                return Err(e);
            }
        };
    let history_start = history.len().saturating_sub(MAX_CONTEXT_MESSAGES);
    let mut messages: Vec<crate::openrouter::ChatMessage> = history[history_start..]
        .iter()
        .filter_map(|m| match m.role.as_str() {
            "user" | "assistant" => Some(crate::openrouter::ChatMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            }),
            other => {
                tracing::warn!(
                    message_id = %m.id,
                    role = %other,
                    "Skipping message with unsupported role in OpenRouter history"
                );
                None
            }
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
            drop(stream_guard);
            return Err(AppError::OpenRouter(e.to_string()));
        }
    };

    // Collect and persist after stream
    let (tx, rx) = tokio::sync::mpsc::channel::<Result<String, std::convert::Infallible>>(64);

    let db2 = db.clone();
    let conv_id2 = conv_id.clone();
    let owner_id = user.id.clone();
    let user_content = body.content.clone();

    tokio::spawn(async move {
        use futures_util::StreamExt;
        let _stream_guard = stream_guard;
        let mut s = Box::pin(stream);
        let mut content = String::new();

        while let Some(event) = s.next().await {
            match event {
                Ok(crate::openrouter::StreamEvent::Delta(delta)) => {
                    content.push_str(&delta);
                    if tx
                        .send(Ok(sse_frame(&SsePayload::Delta { content: &delta })))
                        .await
                        .is_err()
                    {
                        return;
                    }
                }
                Ok(crate::openrouter::StreamEvent::Done) => {
                    let terminal = if content.is_empty() {
                        sse_frame(&SsePayload::Error {
                            message: "Upstream model returned an empty response",
                        })
                    } else {
                        match crate::repository::create_user_assistant_message_pair(
                            &db2,
                            &conv_id2,
                            &owner_id,
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
                        }
                    };
                    let _ = tx.send(Ok(terminal)).await;
                    return;
                }
                Err(e) => {
                    tracing::error!("Streaming chat error: {e}");
                    let _ = tx
                        .send(Ok(sse_frame(&SsePayload::Error {
                            message: stream_error_message(&e),
                        })))
                        .await;
                    return;
                }
            }
        }

        if !content.is_empty() {
            tracing::warn!("Streaming chat ended without a done event; skipping persistence");
        }
    });

    let body_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let body = Body::from_stream(body_stream);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("X-Accel-Buffering", "no")
        .body(body)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(response)
}

fn stream_error_message(error: &AppError) -> &'static str {
    match error {
        AppError::OpenRouter(_) => "Upstream model service error",
        AppError::Database(_) => "A database error occurred",
        AppError::Internal(_) => "An internal server error occurred",
        AppError::Unauthorized(_)
        | AppError::NotFound(_)
        | AppError::BadRequest(_)
        | AppError::Conflict(_) => "Stream error",
    }
}

fn sse_frame(payload: &SsePayload<'_>) -> String {
    let json = serde_json::to_string(payload).expect("serialize SSE payload");
    format!("data: {json}\n\n")
}
