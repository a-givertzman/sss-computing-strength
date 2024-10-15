//! Общая структура ошибки. 
use thiserror::Error as ThisError;

/// Общая структура ошибки. 
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("FromString: {0}")]
    FromString(String),
    #[error("Serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Parameter: {0}")]
    Parameter(String),
    #[error("ApiRequest: {0}")]
    ApiRequest(String),
    #[error("Calculate: {0}")]
    Calculate(String),
    #[error(transparent)]
    Other(#[from] std::io::Error),
}

impl From<std::string::String> for Error {
    fn from(value: std::string::String) -> Self {
        Self::FromString(value)
    }
}