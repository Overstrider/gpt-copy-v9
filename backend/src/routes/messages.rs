// Implemented in TASK-005
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::error::AppResult;
use crate::models::Message;
use crate::state::AppState;
use crate::validation::validate_message_content;

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
}

pub async fn list_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> AppResult<Json<Vec<Message>>> {
    // Ensure conversation exists
    crate::repository::get_conversation(&state.db, &conversation_id).await?;
    let msgs = crate::repository::list_messages(&state.db, &conversation_id).await?;
    Ok(Json(msgs))
}

pub async fn create_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Json(body): Json<CreateMessageRequest>,
) -> AppResult<(StatusCode, Json<Message>)> {
    validate_message_content(&body.content)?;
    crate::repository::get_conversation(&state.db, &conversation_id).await?;
    let msg = crate::repository::create_message(&state.db, &conversation_id, "user", &body.content)
        .await?;
    Ok((StatusCode::CREATED, Json(msg)))
}

pub async fn delete_message(
    State(state): State<AppState>,
    Path((conversation_id, message_id)): Path<(String, String)>,
) -> AppResult<StatusCode> {
    crate::repository::get_conversation(&state.db, &conversation_id).await?;
    crate::repository::delete_message(&state.db, &conversation_id, &message_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
