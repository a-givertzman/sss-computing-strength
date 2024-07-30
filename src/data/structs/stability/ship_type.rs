//! Типы судов
use serde::{Deserialize, Serialize};

use crate::Error;

/// Типы судов
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize,)]
pub enum ShipType {
    /// Навалочное судно
    #[serde(alias="bulk carrier")]
    BulkCarrier,       
    /// Контейнеровоз 
    #[serde(alias="container ship")]
    ContainerShip,    
    /// Суда, предназначенные для перевозки сухих генеральных грузов
    #[serde(alias="bulk carrier")]
    GeneralDryCargoShip, 
    /// Лесовоз
    #[serde(alias="general dry cargo ship")]
    TimberCarrier,  
    /// Наливное судно    
    #[serde(alias="timber carrier")]
    Tanker,
    /// Ролкер
    #[serde(alias="ro-ro ship")]
    RoRo,
    /// Все остальные типы судов  
    #[serde(alias="other")]
    Other,
}
///
impl ShipType {
    ///
    pub fn from_str(src: &str) -> Result<Self, Error> {
        Ok(match src.trim().to_lowercase().as_str() {
            "bulk carrier" => ShipType::BulkCarrier,
            "container ship" => ShipType::ContainerShip,
            "general dry cargo ship" => ShipType::GeneralDryCargoShip,
            "timber carrier" => ShipType::TimberCarrier,
            "tanker" => ShipType::Tanker,
            "ro-ro ship" => ShipType::RoRo,
            "other" => ShipType::Other,
            src @ _ => return Err(Error::FromString(format!("ShipType from_str error: no type {src}"))),
        })
    }
}
///
impl std::fmt::Display for ShipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShipType::BulkCarrier => "BulkCarrier", 
                ShipType::ContainerShip => "ContainerShip", 
                ShipType::GeneralDryCargoShip => "GeneralDryCargoShip", 
                ShipType::TimberCarrier => "TimberCarrier",
                ShipType::Tanker => "Tanker", 
                ShipType::RoRo => "RoRo", 
                ShipType::Other => "Other", 
            },
        )
    }
}