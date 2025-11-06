use serde::{Deserialize, Serialize};

/// Sentry tunnel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentryTunnelConfig {
    /// Sentry host address (without https://)
    pub sentry_host: String,

    /// List of allowed project IDs
    pub allowed_project_ids: Vec<String>,

    /// Custom routing path, defaults to "/tunnel"
    #[serde(default = "default_path")]
    pub path: String,

    /// Request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_path() -> String {
    "/tunnel".to_string()
}

fn default_timeout() -> u64 {
    30
}

impl SentryTunnelConfig {
    /// Create a new configuration
    pub fn new(sentry_host: impl Into<String>, allowed_project_ids: Vec<String>) -> Self {
        Self {
            sentry_host: sentry_host.into(),
            allowed_project_ids,
            path: default_path(),
            timeout_secs: default_timeout(),
        }
    }

    /// Set custom path
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Set timeout duration
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }
}
