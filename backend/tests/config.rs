use gpt_copy_v9::config::Config;

#[test]
fn debug_output_redacts_openrouter_api_key() {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        openrouter_api_key: "sk-secret-test-key".to_string(),
        openrouter_model: "test-model".to_string(),
        host: "127.0.0.1".to_string(),
        port: 3001,
        frontend_origin: "http://localhost:3000".to_string(),
        api_auth_token: Some("secret-api-token".to_string()),
    };

    let debug = format!("{config:?}");

    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("sk-secret-test-key"));
    assert!(!debug.contains("secret-api-token"));
}
