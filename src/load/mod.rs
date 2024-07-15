//! Нагрузка на судно: постоянный и переменный груз. 
use serde::{Deserialize, Serialize};
use crate::{data::structs::loads::{CargoType, CompartmentType, LoadConstantType}, Bound, Position};
mod tank;
mod desk;
mod mass;
mod bulk;

pub use tank::*;
pub use desk::*;
pub use mass::*;
pub use bulk::*;

/// Тип груза
#[derive(Debug, Copy, Clone, Eq, PartialEq,)]
pub enum LoadingType {
    Hull,
    Equipment,
    Ballast,
    Stores,
    Cargo,
}
///
impl std::fmt::Display for LoadingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LoadingType::Hull => "Hull",
                LoadingType::Equipment => "Equipment",
                LoadingType::Ballast => "Ballast",
                LoadingType::Stores => "Stores",
                LoadingType::Cargo => "Cargo",  
            },
        )
    }
}
///
impl From<CargoType> for LoadingType {
    fn from(value: CargoType) -> Self {
        match value {
            CargoType::Ballast => LoadingType::Ballast,
            CargoType::Stores => LoadingType::Stores,
            CargoType::Cargo => LoadingType::Cargo,
        }
    }
}
///
impl From<CompartmentType> for LoadingType {
    fn from(value: CompartmentType) -> Self {
        match value {
            CompartmentType::Ballast => LoadingType::Ballast,
            CompartmentType::Store => LoadingType::Stores,
            CompartmentType::Cargo => LoadingType::Cargo,
        }
    }
}
///
impl From<LoadConstantType> for LoadingType {
    fn from(value: LoadConstantType) -> Self {
        match value {
            LoadConstantType::Equipment => LoadingType::Equipment,
            LoadConstantType::Hull => LoadingType::Hull,
        }
    }
}
/// Абстрактный груз: контейнер, трюм или бак.
/// Имеет массу и может вернуть какая его часть попадает в указанные границы
pub trait ILoad {
    /// Суммарная масса груза
    fn mass(&self) -> f64;
    /// Границы груза вдоль продольной оси
    fn bound_x(&self) -> Bound;
    /// Смещение центра груза относительно начала координат судна
    fn shift(&self) -> Position;
}
