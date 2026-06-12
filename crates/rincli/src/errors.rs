use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum RincliError {
    #[error("Not implemented")]
    NotImplemented,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Registry not found")]
    RegistryNotFound,
    #[error("General error: {0}")]
    General(String),
    #[error("HTTP error: {0}")]
    Http(String),
}

impl From<ureq::Error> for RincliError {
    fn from(err: ureq::Error) -> Self {
        RincliError::Http(err.to_string())
    }
}
