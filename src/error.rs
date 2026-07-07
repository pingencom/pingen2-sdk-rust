use thiserror::Error;

#[derive(Debug, Error)]
pub enum PingenError {
    #[error("HTTP error {status}: {body}")]
    Api { status: u16, body: String },

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Webhook signature error: {0}")]
    WebhookSignature(String),
}

pub type Result<T> = std::result::Result<T, PingenError>;
