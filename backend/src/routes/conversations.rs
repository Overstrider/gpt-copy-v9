// Implemented in TASK-004
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::error::AppResult;
use crate::models::Conversation;
use crate::state::AppState;
use crate::validation::validate_create_conversation;

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub title: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateConversationRequest {
    pub title: Option<String>,
}

pub async fn list_conversations(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Conversation>>> {
    let convs = crate::repository::list_conversations(&state.db).await?;
    Ok(Json(convs))
}

pub async fn create_conversation(
    State(state): State<AppState>,
    Json(body): Json<CreateConversationRequest>,
) -> AppResult<(StatusCode, Json<Conversation>)> {
    validate_create_conversation(&body.title)?;
    let title = body.title.unwrap_or_else(|| "New Chat".to_string());
    let conv = crate::repository::create_conversation(&state.db, &title).await?;
    Ok((StatusCode::CREATED, Json(conv)))
}

pub async fn get_conversation(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<Conversation>> {
    let conv = crate::repository::get_conversation(&state.db, &id).await?;
    Ok(Json(conv))
}

pub async fn update_conversation(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateConversationRequest>,
) -> AppResult<Json<Conversation>> {
    validate_create_conversation(&body.title)?;
    let conv =
        crate::repository::update_conversation(&state.db, &id, body.title.as_deref()).await?;
    Ok(Json(conv))
}

pub async fn delete_conversation(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    crate::repository::delete_conversation(&state.db, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
