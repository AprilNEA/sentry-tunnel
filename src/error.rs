use thiserror::Error;

#[cfg(feature = "extension")]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
#[cfg(feature = "extension")]
use serde_json::json;

#[derive(Error, Debug)]
pub enum SentryTunnelError {
    #[error("Invalid envelope encoding")]
    InvalidEncoding,

    #[error("Empty envelope")]
    EmptyEnvelope,

    #[error("Invalid envelope header: {0}")]
    InvalidHeader(String),

    #[error("Missing DSN in envelope")]
    MissingDsn,

    #[error("Invalid DSN URL: {0}")]
    InvalidDsnUrl(String),

    #[error("Invalid sentry hostname: {0}")]
    InvalidHostname(String),

    #[error("Invalid sentry project id: {0}")]
    InvalidProjectId(String),

    #[error("Error tunneling to sentry: {0}")]
    TunnelError(String),
}

#[cfg(feature = "extension")]
impl IntoResponse for SentryTunnelError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::InvalidEncoding
            | Self::EmptyEnvelope
            | Self::InvalidHeader(_)
            | Self::MissingDsn
            | Self::InvalidDsnUrl(_)
            | Self::InvalidHostname(_)
            | Self::InvalidProjectId(_) => (StatusCode::BAD_REQUEST, self.to_string()),

            Self::TunnelError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
