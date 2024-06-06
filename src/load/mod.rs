//! Нагрузка на судно: постоянный и переменный груз. 
use serde::{Deserialize, Serialize};
use crate::{Bound, Position};
mod tank;
mod desk;
mod mass;
mod bulk;

pub use tank::*;
pub use desk::*;
pub use mass::*;
pub use bulk::*;

/// Тип груза
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize,)]
//#[serde(untagged)]
pub enum LoadingType {
    #[serde(alias="lightship")]
    Lightship,
    #[serde(alias="ballast")]
    Ballast,
    #[serde(alias="store")]
    Store,
    #[serde(alias="cargo")]
    Cargo,
}
///
impl std::fmt::Display for LoadingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LoadingType::Lightship => "Lightship",
                LoadingType::Ballast => "Ballast",
                LoadingType::Store => "Store",
                LoadingType::Cargo => "Cargo",  
            },
        )
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
