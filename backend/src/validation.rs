use crate::error::{AppError, AppResult};

pub fn validate_create_conversation(title: &Option<String>) -> AppResult<()> {
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
    if content.len() > 32_768 {
        return Err(AppError::BadRequest(
            "Message content exceeds maximum length".to_string(),
        ));
    }
    Ok(())
}
