#[derive(Debug, thiserror::Error)]
pub enum DiscogsError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error (status {status}): {body}")]
    Api { status: u16, body: String },

    #[error("rate limited — retry after backoff")]
    RateLimited,

    #[error("configuration error: {0}")]
    Configuration(String),

    #[error("authentication required for this endpoint")]
    AuthRequired,

    #[error("JSON deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, DiscogsError>;
