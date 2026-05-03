mod support;

use gpt_copy_v9::openrouter::{ChatMessage, FakeOpenRouterClient, OpenRouterClient};

#[tokio::test]
async fn test_fake_openrouter_emits_deltas() {
    let client = FakeOpenRouterClient {
        responses: vec!["Hello".to_string(), " world".to_string()],
    };
    let msgs = vec![ChatMessage {
        role: "user".to_string(),
        content: "Hi".to_string(),
    }];
    let mut stream = client.stream_chat("test-model", msgs).await.unwrap();

    use futures_util::StreamExt;
    let mut collected = String::new();
    while let Some(event) = stream.next().await {
        match event.unwrap() {
            gpt_copy_v9::openrouter::StreamEvent::Delta(d) => collected.push_str(&d),
            gpt_copy_v9::openrouter::StreamEvent::Done => break,
        }
    }
    assert_eq!(collected, "Hello world");
}

#[tokio::test]
async fn test_fake_openrouter_empty_responses() {
    let client = FakeOpenRouterClient { responses: vec![] };
    let msgs = vec![ChatMessage {
        role: "user".to_string(),
        content: "Hi".to_string(),
    }];
    let mut stream = client.stream_chat("test-model", msgs).await.unwrap();

    use futures_util::StreamExt;
    let mut got_done = false;
    while let Some(event) = stream.next().await {
        if matches!(event.unwrap(), gpt_copy_v9::openrouter::StreamEvent::Done) {
            got_done = true;
            break;
        }
    }
    assert!(got_done);
}
