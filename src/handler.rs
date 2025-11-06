use crate::{config::SentryTunnelConfig, error::SentryTunnelError};
use std::sync::Arc;

#[cfg(feature = "extension")]
use axum::{body::Bytes, http::StatusCode};

#[cfg(feature = "extension")]
/// Handle Sentry tunnel requests
pub async fn handle_sentry_tunnel(
    config: Arc<SentryTunnelConfig>,
    envelope_bytes: Bytes,
) -> Result<StatusCode, SentryTunnelError> {
    handle_sentry_tunnel_inner(config, &envelope_bytes).await.map(|_| StatusCode::OK)
}

/// Core handler logic independent of web framework
pub async fn handle_sentry_tunnel_inner(
    config: Arc<SentryTunnelConfig>,
    envelope_bytes: &[u8],
) -> Result<(), SentryTunnelError> {
    // Decode envelope
    let envelope = std::str::from_utf8(envelope_bytes)
        .map_err(|_| SentryTunnelError::InvalidEncoding)?;

    // Extract first line (header)
    let header_line = envelope
        .split('\n')
        .next()
        .ok_or(SentryTunnelError::EmptyEnvelope)?;

    // Parse header as JSON
    let header: serde_json::Value = serde_json::from_str(header_line)
        .map_err(|e| SentryTunnelError::InvalidHeader(e.to_string()))?;

    // Extract and validate DSN
    let dsn = header["dsn"]
        .as_str()
        .ok_or(SentryTunnelError::MissingDsn)?;

    let dsn_url = url::Url::parse(dsn)
        .map_err(|e| SentryTunnelError::InvalidDsnUrl(e.to_string()))?;

    // Validate hostname
    let hostname = dsn_url.host_str().unwrap_or("");
    if hostname != config.sentry_host {
        return Err(SentryTunnelError::InvalidHostname(hostname.to_string()));
    }

    // Validate project ID
    let project_id = dsn_url.path().trim_start_matches('/');
    if project_id.is_empty() || !config.allowed_project_ids.iter().any(|id| id == project_id) {
        return Err(SentryTunnelError::InvalidProjectId(project_id.to_string()));
    }

    // Build upstream Sentry URL
    let upstream_sentry_url = format!(
        "https://{}/api/{}/envelope/",
        config.sentry_host,
        project_id
    );

    // Forward request to Sentry
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout_secs))
        .build()
        .map_err(|e| SentryTunnelError::TunnelError(e.to_string()))?;

    let response = client
        .post(&upstream_sentry_url)
        .body(envelope_bytes.to_vec())
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Error tunneling to sentry: {}", e);
            SentryTunnelError::TunnelError(e.to_string())
        })?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status_code = response.status();
        tracing::error!("Sentry upstream returned error: {}", status_code);
        Err(SentryTunnelError::TunnelError(format!(
            "Upstream returned {}",
            status_code
        )))
    }
}
