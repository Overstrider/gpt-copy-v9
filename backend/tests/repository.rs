mod support;
use support::test_pool;

use gpt_copy_v9::repository;

#[tokio::test]
async fn test_create_and_list_conversations() {
    let pool = test_pool().await;
    let conv = repository::create_conversation(&pool, "Test Chat")
        .await
        .unwrap();
    assert_eq!(conv.title, "Test Chat");

    let convs = repository::list_conversations(&pool).await.unwrap();
    assert_eq!(convs.len(), 1);
    assert_eq!(convs[0].id, conv.id);
}

#[tokio::test]
async fn test_get_conversation_not_found() {
    let pool = test_pool().await;
    let result = repository::get_conversation(&pool, "nonexistent-id").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_conversation() {
    let pool = test_pool().await;
    let conv = repository::create_conversation(&pool, "Original")
        .await
        .unwrap();
    let updated = repository::update_conversation(&pool, &conv.id, Some("Updated"))
        .await
        .unwrap();
    assert_eq!(updated.title, "Updated");
}

#[tokio::test]
async fn test_delete_conversation() {
    let pool = test_pool().await;
    let conv = repository::create_conversation(&pool, "To Delete")
        .await
        .unwrap();
    repository::delete_conversation(&pool, &conv.id)
        .await
        .unwrap();
    let result = repository::get_conversation(&pool, &conv.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_and_list_messages() {
    let pool = test_pool().await;
    let conv = repository::create_conversation(&pool, "Chat")
        .await
        .unwrap();
    let msg = repository::create_message(&pool, &conv.id, "user", "Hello")
        .await
        .unwrap();
    assert_eq!(msg.role, "user");
    assert_eq!(msg.content, "Hello");

    let msgs = repository::list_messages(&pool, &conv.id).await.unwrap();
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].id, msg.id);
}

#[tokio::test]
async fn test_delete_message() {
    let pool = test_pool().await;
    let conv = repository::create_conversation(&pool, "Chat")
        .await
        .unwrap();
    let msg = repository::create_message(&pool, &conv.id, "user", "Hello")
        .await
        .unwrap();
    repository::delete_message(&pool, &conv.id, &msg.id)
        .await
        .unwrap();
    let msgs = repository::list_messages(&pool, &conv.id).await.unwrap();
    assert!(msgs.is_empty());
}

#[tokio::test]
async fn test_messages_ordered_chronologically() {
    let pool = test_pool().await;
    let conv = repository::create_conversation(&pool, "Chat")
        .await
        .unwrap();
    repository::create_message(&pool, &conv.id, "user", "First")
        .await
        .unwrap();
    repository::create_message(&pool, &conv.id, "assistant", "Second")
        .await
        .unwrap();
    repository::create_message(&pool, &conv.id, "user", "Third")
        .await
        .unwrap();

    let msgs = repository::list_messages(&pool, &conv.id).await.unwrap();
    assert_eq!(msgs.len(), 3);
    assert_eq!(msgs[0].content, "First");
    assert_eq!(msgs[2].content, "Third");
}

#[tokio::test]
async fn test_delete_conversation_cascades_messages() {
    let pool = test_pool().await;
    let conv = repository::create_conversation(&pool, "Chat")
        .await
        .unwrap();
    repository::create_message(&pool, &conv.id, "user", "msg1")
        .await
        .unwrap();
    repository::delete_conversation(&pool, &conv.id)
        .await
        .unwrap();
    // No panic means cascade worked; verifying via direct SQL
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages WHERE conversation_id = ?")
        .bind(&conv.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 0);
}
