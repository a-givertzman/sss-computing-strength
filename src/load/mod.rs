//! Нагрузка на судно: постоянный и переменный груз. 

mod tank;
mod desk;
mod mass;
mod bulk;

pub use tank::*;
pub use desk::*;
pub use mass::*;
pub use bulk::*;

use crate::{Bound, Position};

/// Тип груза
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LoadType {
    Lightship,
    Ballast,
    Store,
    Cargo,
    None,
}
///
impl From<String> for LoadType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "lightship" => LoadType::Lightship,
            "ballast" => LoadType::Ballast,
            "store" => LoadType::Store,
            "cargo" => LoadType::Cargo,  
            _ => LoadType::None,
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
