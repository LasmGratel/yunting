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

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    Serialize(#[from] toml::ser::Error),
}
