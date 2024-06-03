//! Общая структура ошибки. 
use thiserror::Error as ThisError;

/// Общая структура ошибки. 
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("FromUtf8Error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("ParseIntError")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("ParseFloatError")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Error")]
    FromString(String),
    #[error("Serde error")]
    Serde(#[from] serde_json::Error),
    #[error("Parameter error")]
    Parameter(String),
    #[error("ApiRequest error")]
    ApiRequest(String),
    #[error("Calculate error")]
    Calculate(String),
    #[error(transparent)]
    Other(#[from] std::io::Error),
}

impl From<std::string::String> for Error {
    fn from(value: std::string::String) -> Self {
        Self::FromString(value)
    }
}