use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("json parse failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("regex failed: {0}")]
    Regex(#[from] regex::Error),

    #[error("API not initialized. Call init() first")]
    NotInitialized,

    #[error("missing config key: {0}")]
    MissingConfig(&'static str),

    #[error("invalid videoId")]
    InvalidVideoId,

    #[error("invalid response structure")]
    InvalidResponse,
}
