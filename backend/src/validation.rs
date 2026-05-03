use crate::error::{AppError, AppResult};

// Keep request payloads bounded for context-window safety and abuse prevention.
const MAX_MESSAGE_BYTES: usize = 32_768;

pub fn validate_conversation_title(title: &Option<String>) -> AppResult<()> {
    if let Some(t) = title {
        if t.trim().is_empty() {
            return Err(AppError::BadRequest(
                "Conversation title cannot be blank".to_string(),
            ));
        }
        if t.len() > 256 {
            return Err(AppError::BadRequest(
                "Conversation title must be 256 characters or fewer".to_string(),
            ));
        }
    }
    Ok(())
}

pub fn validate_message_content(content: &str) -> AppResult<()> {
    if content.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Message content cannot be empty".to_string(),
        ));
    }
    if content.len() > MAX_MESSAGE_BYTES {
        return Err(AppError::BadRequest(
            "Message content exceeds maximum length".to_string(),
        ));
    }
    Ok(())
}
