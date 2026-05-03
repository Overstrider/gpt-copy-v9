use std::env;
use std::fmt;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub openrouter_api_key: String,
    pub openrouter_model: String,
    pub host: String,
    pub port: u16,
    pub frontend_origin: String,
    pub api_auth_token: String,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &self.database_url)
            .field("openrouter_api_key", &"[REDACTED]")
            .field("openrouter_model", &self.openrouter_model)
            .field("host", &self.host)
            .field("port", &self.port)
            .field("frontend_origin", &self.frontend_origin)
            .field("api_auth_token", &"[REDACTED]")
            .finish()
    }
}

impl Config {
    pub fn from_env() -> Self {
        let openrouter_api_key = env::var("OPENROUTER_API_KEY")
            .expect("OPENROUTER_API_KEY must be set (set it in .env or the environment)");
        if openrouter_api_key.trim().is_empty() {
            panic!("OPENROUTER_API_KEY must not be empty");
        }
        let openrouter_model = env::var("OPENROUTER_MODEL")
            .unwrap_or_else(|_| "nvidia/nemotron-3-super-120b-a12b:free".to_string());
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./gpt-copy-v9.db".to_string());
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .unwrap_or(3001);
        let frontend_origin =
            env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let api_auth_token = env::var("API_AUTH_TOKEN")
            .expect("API_AUTH_TOKEN must be set (set it in .env or the environment)");
        if api_auth_token.trim().is_empty() {
            panic!("API_AUTH_TOKEN must not be empty");
        }

        Config {
            database_url,
            openrouter_api_key,
            openrouter_model,
            host,
            port,
            frontend_origin,
            api_auth_token,
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
            frontend_origin: "http://localhost:3000".to_string(),
            api_auth_token: "test-token".to_string(),
        }
    }
}
