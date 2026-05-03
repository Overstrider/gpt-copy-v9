use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::error::AppResult;
use crate::models::Conversation;
use crate::state::{AppState, AuthenticatedUser};
use crate::validation::validate_conversation_title;

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
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<Conversation>>> {
    let convs = crate::repository::list_conversations(&state.db, &user.id).await?;
    Ok(Json(convs))
}

pub async fn create_conversation(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(body): Json<CreateConversationRequest>,
) -> AppResult<(StatusCode, Json<Conversation>)> {
    validate_conversation_title(&body.title)?;
    let title = body.title.unwrap_or_else(|| "New Chat".to_string());
    let conv = crate::repository::create_conversation(&state.db, &user.id, &title).await?;
    Ok((StatusCode::CREATED, Json(conv)))
}

pub async fn get_conversation(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<String>,
) -> AppResult<Json<Conversation>> {
    let conv = crate::repository::get_conversation(&state.db, &id, &user.id).await?;
    Ok(Json(conv))
}

pub async fn update_conversation(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<String>,
    Json(body): Json<UpdateConversationRequest>,
) -> AppResult<Json<Conversation>> {
    validate_conversation_title(&body.title)?;
    let Some(title) = body.title.as_deref() else {
        return Err(crate::error::AppError::BadRequest(
            "Conversation title is required".to_string(),
        ));
    };
    let conv = crate::repository::update_conversation(&state.db, &id, &user.id, title).await?;
    Ok(Json(conv))
}

pub async fn delete_conversation(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    crate::repository::delete_conversation(&state.db, &id, &user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
