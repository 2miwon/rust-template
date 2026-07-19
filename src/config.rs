use crate::error::Result;

/// Application configuration, loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Human-readable app name, used in logs. Defaults to `"rust-template"`.
    pub app_name: String,
    /// `tracing`/`RUST_LOG`-style log level. Defaults to `"info"`.
    pub log_level: String,
}

impl Config {
    /// Reads `APP_NAME` and `LOG_LEVEL` from the environment, falling back
    /// to sane defaults when unset.
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            app_name: std::env::var("APP_NAME").unwrap_or_else(|_| "rust-template".to_string()),
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        })
    }
}
