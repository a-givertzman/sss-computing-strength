//! Интерфес для парсинга json от  АПИ сервера
use crate::error::Error;

/// Интерфес для парсинга json от  АПИ сервера,  
/// проверяет наличие строки с ошибкой
pub trait IFromJson {
    /// Строка с ошибкой если она пришла с сервера
    fn error(&self) -> Option<&String>;
    /// Парсинг данных из json строки
    fn parse<'a>(src: &'a [u8]) -> Result<Self, Error> where Self: Sized + serde::Deserialize<'a> {
        let res: Self = serde_json::from_slice(src)?;

        if let Some(error) = res.error() {
            if !error.is_empty() {
                return Err(Error::ApiRequest(error.to_string()));
            }
        }

        Ok(res)
    }
}
