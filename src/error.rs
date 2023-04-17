#[derive(Debug, thiserror::Error)]
pub enum NodelessError {
    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Invalid Response")]
    InvalidResponse,
}
