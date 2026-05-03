// Implemented in TASK-007
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Json,
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

    // Persist user message
    let _user_msg =
        crate::repository::create_message(&state.db, &conversation_id, "user", &body.content)
            .await?;

    // Get conversation history
    let history = crate::repository::list_messages(&state.db, &conversation_id).await?;
    let messages: Vec<crate::openrouter::ChatMessage> = history
        .iter()
        .map(|m| crate::openrouter::ChatMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    let db = state.db.clone();
    let conv_id = conversation_id.clone();

    let stream = state
        .openrouter
        .stream_chat(&state.config.openrouter_model, messages)
        .await
        .map_err(|e| AppError::OpenRouter(e.to_string()))?;

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
                format!(
                    "data: {{\"type\":\"error\",\"message\":{}}}\n\n",
                    serde_json::json!(e.to_string())
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
        while let Some(item) = s.next().await {
            let is_done = item
                .as_ref()
                .map(|s: &String| s.contains("\"done\""))
                .unwrap_or(false);
            let _ = tx.send(item).await;
            if is_done {
                break;
            }
        }
        // Persist assistant message
        let content = buf_final.lock().unwrap().clone();
        if !content.is_empty() {
            let _ = crate::repository::create_message(&db2, &conv_id2, "assistant", &content).await;
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
