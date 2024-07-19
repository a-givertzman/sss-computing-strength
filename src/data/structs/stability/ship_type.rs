//! Типы судов
use serde::{Deserialize, Serialize};

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