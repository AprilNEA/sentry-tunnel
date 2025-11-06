mod config;
mod error;
mod handler;

pub use config::SentryTunnelConfig;
pub use error::SentryTunnelError;
pub use handler::handle_sentry_tunnel_inner;

#[cfg(feature = "extension")]
use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    routing::post,
    Router,
};
#[cfg(feature = "extension")]
use std::sync::Arc;

#[cfg(feature = "extension")]
#[cfg_attr(feature = "utoipa", utoipa::path(
    post,
    path = "/tunnel",
    request_body = Vec<u8>,
    responses(
        (status = 200, description = "Successfully tunneled to Sentry"),
        (status = 400, description = "Bad request - invalid envelope or DSN"),
        (status = 500, description = "Internal server error - failed to tunnel")
    ),
    tag = "sentry"
))]
/// Create Sentry tunnel route handler
pub async fn sentry_tunnel_handler(
    State(config): State<Arc<SentryTunnelConfig>>,
    body: Bytes,
) -> Result<StatusCode, SentryTunnelError> {
    handler::handle_sentry_tunnel(config, body).await
}

#[cfg(feature = "extension")]
/// Extension methods for Router
pub trait SentryTunnelExt {
    /// Add Sentry tunnel route to Router
    fn sentry_tunnel(self, config: SentryTunnelConfig) -> Self;
}

#[cfg(feature = "extension")]
impl SentryTunnelExt for Router {
    fn sentry_tunnel(self, config: SentryTunnelConfig) -> Self {
        let path = config.path.clone();
        let config = Arc::new(config);

        self.route(
            &path,
            post(sentry_tunnel_handler).with_state(config),
        )
    }
}

#[cfg(feature = "standalone")]
/// Create a standalone Sentry tunnel service
pub fn create_sentry_tunnel_service(config: SentryTunnelConfig) -> Router {
    Router::new().sentry_tunnel(config)
}

#[cfg(feature = "utoipa")]
/// Implementation of SentryTunnelExt for utoipa-axum OpenApiRouter
impl<S> SentryTunnelExt for utoipa_axum::router::OpenApiRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn sentry_tunnel(self, config: SentryTunnelConfig) -> Self {
        let path = config.path.clone();
        let config_arc = Arc::new(config);

        // Add the route using the standard router method
        // The utoipa::path annotation is already applied to the handler
        self.route(&path, post(sentry_tunnel_handler).with_state(config_arc))
    }
}

/// Builder pattern for creating configuration
pub struct SentryTunnelBuilder {
    config: SentryTunnelConfig,
}

impl SentryTunnelBuilder {
    /// Create a new builder
    pub fn new(sentry_host: impl Into<String>) -> Self {
        Self {
            config: SentryTunnelConfig::new(sentry_host, vec![]),
        }
    }

    /// Add an allowed project ID
    pub fn allow_project_id(mut self, project_id: impl Into<String>) -> Self {
        self.config.allowed_project_ids.push(project_id.into());
        self
    }

    /// Add multiple allowed project IDs
    pub fn allow_project_ids<I, S>(mut self, project_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.config.allowed_project_ids.extend(
            project_ids.into_iter().map(|s| s.into())
        );
        self
    }

    /// Set the path
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.config.path = path.into();
        self
    }

    /// Set the timeout
    pub fn timeout_secs(mut self, timeout: u64) -> Self {
        self.config.timeout_secs = timeout;
        self
    }

    /// Build the configuration
    pub fn build(self) -> SentryTunnelConfig {
        self.config
    }
}
