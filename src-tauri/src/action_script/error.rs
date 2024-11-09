use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid action type: {0}")]
    InvalidActionType(String),
    #[error("Invalid simulation type: {0}")]
    InvalidSimulationType(String),
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
    #[error("Invalid parameter format")]
    InvalidParameterFormat,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
