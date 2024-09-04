pub mod load_constant;
pub mod compartment;
pub mod cargo;
pub mod bulkhead;

pub use load_constant::*;
pub use compartment::*;
pub use cargo::*;
pub use bulkhead::*;

use serde::{Deserialize, Serialize};

/// Тип груза по назначению
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize,)]
pub enum CargoGeneralCategory {
    #[serde(alias="lightship")]
    Lightship,
    #[serde(alias="ballast")]
    Ballast,
    #[serde(alias="stores")]
    Stores,
    #[serde(alias="cargo")]
    Cargo,
}
///
impl std::fmt::Display for CargoGeneralCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CargoGeneralCategory::Lightship => "Lightship", 
                CargoGeneralCategory::Ballast => "Ballast", 
                CargoGeneralCategory::Stores => "Stores", 
                CargoGeneralCategory::Cargo => "Cargo", 
            },
        )
    }
}
/// Физический тип груза судна
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize,)]
pub enum MatterType {
    #[serde(alias="bulk")]
    Bulk,
    #[serde(alias="liquid")]
    Liquid,
    #[serde(alias="solid")]
    Solid,
}
///
impl std::fmt::Display for MatterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MatterType::Bulk => "Bulk",
                MatterType::Liquid => "Liquid", 
                MatterType::Solid => "Solid", 
            },
        )
    }
}

