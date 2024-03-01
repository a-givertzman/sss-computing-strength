//! Ошибка
use tokio_postgres::Error as TokioError;
use thiserror::Error as ThisError;

/// Общая структура ошибки. 
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Tokio error")]
    Tokio(#[from] TokioError),
    #[error("Serde error")]
    Serde(String),
    #[error("Parameter error")]
    Parameter(String),
    #[error(transparent)]
    Other(#[from] std::io::Error),
}