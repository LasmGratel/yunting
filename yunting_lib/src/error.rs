use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Server returned error code {0}: {1}")]
    ServerError(i32, String),
}