#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid action type: {0}")]
    InvalidActionType(String),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Invalid parameter value: {parameter} - {reason}")]
    InvalidParameterValue { parameter: String, reason: String },

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("[Line {line}] {source}")]
    WithLineInfo {
        line: usize,
        source: Box<ParseError>,
    },
}

impl ParseError {
    pub fn with_line_info(line: usize, error: ParseError) -> Self {
        ParseError::WithLineInfo {
            line,
            source: Box::new(error),
        }
    }
}

impl From<ParseError> for String {
    fn from(error: ParseError) -> String {
        error.to_string()
    }
}
