//! Типы судов
use serde::{Deserialize, Serialize};

use crate::Error;

/// Типы судов
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ShipType {
    /// Судно, предназначенное для перевозки сухих генеральных грузов
    #[serde(alias = "general dry cargo ship")]
    GeneralDryCargoShip,
    /// Навалочное судно
    #[serde(alias = "bulk carrier")]
    BulkCarrier,
    /// Контейнеровоз
    #[serde(alias = "container ship")]
    ContainerShip,
    /// Лесовоз
    #[serde(alias = "timber carrier")]
    TimberCarrier,
    /// Наливное судно    
    #[serde(alias = "tanker")]
    Tanker,
    /// Нефтепродуктовоз   
    #[serde(alias = "oil tanker")]
    OilTanker,
    /// Химовоз   
    #[serde(alias = "chemical tanker")]
    ChemicalTanker,
    /// Газовоз  
    #[serde(alias = "gas carrier")]
    GasCarrier,
    /// Накатное судно
    #[serde(alias = "ro-ro ship")]
    RoRo,
    /// Все остальные типы судов  
    #[serde(alias = "other")]
    Other,
}
///
impl ShipType {
    ///
    pub fn from_str(src: &str) -> Result<Self, Error> {
        Ok(match src.trim().to_lowercase().as_str() {
            "general dry cargo ship" => ShipType::GeneralDryCargoShip,
            "bulk carrier" => ShipType::BulkCarrier,
            "container ship" => ShipType::ContainerShip,
            "timber carrier" => ShipType::TimberCarrier,
            "tanker" => ShipType::Tanker,
            "oil tanker" => ShipType::OilTanker,
            "chemical tanker" => ShipType::ChemicalTanker,
            "gas carrier" => ShipType::GasCarrier,
            "ro-ro ship" => ShipType::RoRo,
            "other" => ShipType::Other,
            src @ _ => {
                return Err(Error::FromString(format!(
                    "ShipType from_str error: no type {src}"
                )))
            }
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
                ShipType::GeneralDryCargoShip => "GeneralDryCargoShip",
                ShipType::BulkCarrier => "BulkCarrier",
                ShipType::ContainerShip => "ContainerShip",
                ShipType::TimberCarrier => "TimberCarrier",
                ShipType::Tanker => "Tanker",
                ShipType::OilTanker => "OilTanker",
                ShipType::ChemicalTanker => "ChemicalTanker",
                ShipType::GasCarrier => "GasCarrier",
                ShipType::RoRo => "RoRo",
                ShipType::Other => "Other",
            },
        )
    }
}
