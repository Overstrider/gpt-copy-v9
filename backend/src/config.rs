use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub openrouter_api_key: String,
    pub openrouter_model: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let openrouter_api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| String::new());
        let openrouter_model = env::var("OPENROUTER_MODEL")
            .unwrap_or_else(|_| "nvidia/nemotron-3-super-120b-a12b:free".to_string());
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./gpt-copy-v9.db".to_string());
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .unwrap_or(3001);

        Config {
            database_url,
            openrouter_api_key,
            openrouter_model,
            host,
            port,
        }
    }

    /// Create config for testing — no real key required.
    pub fn for_test() -> Self {
        Config {
            database_url: "sqlite::memory:".to_string(),
            openrouter_api_key: "test-key".to_string(),
            openrouter_model: "nvidia/nemotron-3-super-120b-a12b:free".to_string(),
            host: "127.0.0.1".to_string(),
            port: 3001,
        }
    }
}
