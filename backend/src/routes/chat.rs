// Implemented in TASK-007
use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use serde::Deserialize;

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::validation::validate_message_content;

#[derive(Deserialize)]
pub struct StreamRequest {
    pub content: String,
}

pub async fn stream_chat(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Json(body): Json<StreamRequest>,
) -> AppResult<Response> {
    validate_message_content(&body.content)?;
    // Ensure conversation exists
    crate::repository::get_conversation(&state.db, &conversation_id).await?;

    // Get conversation history
    let history = crate::repository::list_messages(&state.db, &conversation_id).await?;
    let mut messages: Vec<crate::openrouter::ChatMessage> = history
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

    let stream = state
        .openrouter
        .stream_chat(&state.config.openrouter_model, messages)
        .await
        .map_err(|e| AppError::OpenRouter(e.to_string()))?;

    // Persist only after the upstream stream is confirmed open.
    let _user_msg =
        crate::repository::create_message(&state.db, &conversation_id, "user", &body.content)
            .await?;

    use futures_util::StreamExt;
    use std::sync::{Arc, Mutex};

    let buffer = Arc::new(Mutex::new(String::new()));
    let buffer_clone = buffer.clone();

    let sse_stream = stream.map(move |event| {
        let chunk = match &event {
            Ok(crate::openrouter::StreamEvent::Delta(delta)) => {
                buffer_clone.lock().unwrap().push_str(delta);
                format!(
                    "data: {{\"type\":\"delta\",\"content\":{}}}\n\n",
                    serde_json::json!(delta)
                )
            }
            Ok(crate::openrouter::StreamEvent::Done) => "data: {\"type\":\"done\"}\n\n".to_string(),
            Err(e) => {
                tracing::error!("Streaming chat error: {e}");
                format!(
                    "data: {{\"type\":\"error\",\"message\":{}}}\n\n",
                    serde_json::json!(stream_error_message(e))
                )
            }
        };
        Ok::<_, std::convert::Infallible>(chunk)
    });

    // Collect and persist after stream
    let (tx, rx) = tokio::sync::mpsc::channel::<Result<String, std::convert::Infallible>>(64);

    let db2 = db.clone();
    let conv_id2 = conv_id.clone();
    let buf_final = buffer.clone();

    tokio::spawn(async move {
        use futures_util::StreamExt;
        let mut s = Box::pin(sse_stream);
        let mut completed = false;
        let mut client_connected = true;
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
            if tx.send(item).await.is_err() {
                client_connected = false;
                break;
            }
            if is_done {
                completed = true;
                break;
            }
            if is_error {
                break;
            }
        }
        if completed && client_connected {
            let content = buf_final.lock().unwrap().clone();
            if !content.is_empty() {
                let _ =
                    crate::repository::create_message(&db2, &conv_id2, "assistant", &content).await;
            }
        }
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
        AppError::NotFound(_) | AppError::BadRequest(_) => "Stream error",
    }
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
