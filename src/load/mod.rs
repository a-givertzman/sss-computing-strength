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
