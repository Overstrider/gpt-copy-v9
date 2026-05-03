use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{Conversation, Message};

// ── Conversations ─────────────────────────────────────────────────────────────

pub async fn list_conversations(pool: &SqlitePool, owner_id: &str) -> AppResult<Vec<Conversation>> {
    let rows = sqlx::query_as::<_, Conversation>(
        "SELECT id, title, created_at, updated_at FROM conversations WHERE owner_id = ? ORDER BY updated_at DESC",
    )
    .bind(owner_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_conversation(
    pool: &SqlitePool,
    owner_id: &str,
    title: &str,
) -> AppResult<Conversation> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO conversations (id, owner_id, title, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(owner_id)
    .bind(title)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    get_conversation(pool, &id, owner_id).await
}

pub async fn get_conversation(
    pool: &SqlitePool,
    id: &str,
    owner_id: &str,
) -> AppResult<Conversation> {
    sqlx::query_as::<_, Conversation>(
        "SELECT id, title, created_at, updated_at FROM conversations WHERE id = ? AND owner_id = ?",
    )
    .bind(id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Conversation {id} not found")))
}

pub async fn update_conversation(
    pool: &SqlitePool,
    id: &str,
    owner_id: &str,
    title: &str,
) -> AppResult<Conversation> {
    // Ensure exists
    get_conversation(pool, id, owner_id).await?;
    let now = Utc::now();
    sqlx::query("UPDATE conversations SET title = ?, updated_at = ? WHERE id = ? AND owner_id = ?")
        .bind(title)
        .bind(now)
        .bind(id)
        .bind(owner_id)
        .execute(pool)
        .await?;
    get_conversation(pool, id, owner_id).await
}

pub async fn delete_conversation(pool: &SqlitePool, id: &str, owner_id: &str) -> AppResult<()> {
    get_conversation(pool, id, owner_id).await?;
    sqlx::query("DELETE FROM conversations WHERE id = ? AND owner_id = ?")
        .bind(id)
        .bind(owner_id)
        .execute(pool)
        .await?;
    Ok(())
}

// ── Messages ──────────────────────────────────────────────────────────────────

pub async fn list_messages(
    pool: &SqlitePool,
    conversation_id: &str,
    owner_id: &str,
) -> AppResult<Vec<Message>> {
    let rows = sqlx::query_as::<_, Message>(
        "SELECT m.id, m.conversation_id, m.role, m.content, m.created_at FROM messages m JOIN conversations c ON c.id = m.conversation_id WHERE m.conversation_id = ? AND c.owner_id = ? ORDER BY m.created_at ASC, m.rowid ASC",
    )
    .bind(conversation_id)
    .bind(owner_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_message(
    pool: &SqlitePool,
    conversation_id: &str,
    owner_id: &str,
    role: &str,
    content: &str,
) -> AppResult<Message> {
    get_conversation(pool, conversation_id, owner_id).await?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO messages (id, conversation_id, role, content, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(conversation_id)
    .bind(role)
    .bind(content)
    .bind(now)
    .execute(pool)
    .await?;
    // Touch conversation updated_at
    sqlx::query("UPDATE conversations SET updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(conversation_id)
        .execute(pool)
        .await?;
    get_message(pool, conversation_id, owner_id, &id).await
}

pub async fn create_user_assistant_message_pair(
    pool: &SqlitePool,
    conversation_id: &str,
    owner_id: &str,
    user_content: &str,
    assistant_content: &str,
) -> AppResult<()> {
    get_conversation(pool, conversation_id, owner_id).await?;
    let user_id = Uuid::new_v4().to_string();
    let assistant_id = Uuid::new_v4().to_string();
    let user_created_at = Utc::now();
    let assistant_created_at = user_created_at + chrono::Duration::microseconds(1);

    let mut tx = pool.begin().await?;
    sqlx::query(
        "INSERT INTO messages (id, conversation_id, role, content, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&user_id)
    .bind(conversation_id)
    .bind("user")
    .bind(user_content)
    .bind(user_created_at)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "INSERT INTO messages (id, conversation_id, role, content, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&assistant_id)
    .bind(conversation_id)
    .bind("assistant")
    .bind(assistant_content)
    .bind(assistant_created_at)
    .execute(&mut *tx)
    .await?;
    sqlx::query("UPDATE conversations SET updated_at = ? WHERE id = ?")
        .bind(assistant_created_at)
        .bind(conversation_id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    Ok(())
}

pub async fn get_message(
    pool: &SqlitePool,
    conversation_id: &str,
    owner_id: &str,
    message_id: &str,
) -> AppResult<Message> {
    sqlx::query_as::<_, Message>(
        "SELECT m.id, m.conversation_id, m.role, m.content, m.created_at FROM messages m JOIN conversations c ON c.id = m.conversation_id WHERE m.id = ? AND m.conversation_id = ? AND c.owner_id = ?",
    )
    .bind(message_id)
    .bind(conversation_id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Message {message_id} not found")))
}

pub async fn delete_message(
    pool: &SqlitePool,
    conversation_id: &str,
    owner_id: &str,
    message_id: &str,
) -> AppResult<()> {
    get_message(pool, conversation_id, owner_id, message_id).await?;
    sqlx::query("DELETE FROM messages WHERE id = ? AND conversation_id = ?")
        .bind(message_id)
        .bind(conversation_id)
        .execute(pool)
        .await?;
    Ok(())
}
