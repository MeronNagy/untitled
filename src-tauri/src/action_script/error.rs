#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid action type: {0}")]
    InvalidActionType(String),
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
    #[error("Invalid parameter format")]
    InvalidParameterFormat,
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    #[error("Invalid parameter value: {parameter} - {reason}")]
    InvalidParameterValue {
        parameter: String,
        reason: String,
    },
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl From<ParseError> for String {
    fn from(error: ParseError) -> String {
        error.to_string()
    }
}