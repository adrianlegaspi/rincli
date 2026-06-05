use thiserror::Error;

#[derive(Error, Debug)]
pub enum RincliError {
    #[error("Not implemented")]
    NotImplemented,
}
