#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid action type: {0}")]
    InvalidActionType(String),
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
    #[error("Invalid parameter format")]
    InvalidParameterFormat,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl serde::Serialize for ParseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
