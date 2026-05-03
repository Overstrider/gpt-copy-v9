use gpt_copy_v9::state::StreamRegistry;

#[tokio::test]
async fn stream_registry_rejects_duplicate_conversation_until_released() {
    let registry = StreamRegistry::default();

    let lease = registry.try_acquire("conversation-1");
    assert!(lease.is_some());
    assert!(registry.try_acquire("conversation-1").is_none());
    assert!(registry.try_acquire("conversation-2").is_some());

    drop(lease);

    assert!(registry.try_acquire("conversation-1").is_some());
}

#[test]
fn stream_registry_limits_total_concurrent_streams() {
    let registry = StreamRegistry::with_max_concurrent(1);

    let lease = registry.try_acquire("conversation-1");
    assert!(lease.is_some());
    assert!(registry.try_acquire("conversation-2").is_none());

    drop(lease);

    assert!(registry.try_acquire("conversation-2").is_some());
}
