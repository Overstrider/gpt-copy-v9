use gpt_copy_v9::state::StreamRegistry;

#[tokio::test]
async fn stream_registry_rejects_duplicate_conversation_until_released() {
    let registry = StreamRegistry::default();

    assert!(registry.try_acquire("conversation-1").await);
    assert!(!registry.try_acquire("conversation-1").await);
    assert!(registry.try_acquire("conversation-2").await);

    registry.release("conversation-1").await;

    assert!(registry.try_acquire("conversation-1").await);
}
