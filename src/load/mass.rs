//! Масса груз
use crate::math::*;

/// Абстрактная масса груза.
/// Может вернуть какая масса попадает в указанные границы
pub trait IMass {
    /// Суммарная масса груза
    fn sum(&self) -> f64;
    /// Границы груза вдоль продольной оси
    fn bound_x(&self) -> Bound;
    /// Масса груза, попадающая в Bound или вся если Bound не заданно
    fn value(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound_x().part_ratio(&bound) * self.sum()
        } else {
            self.sum()
        }
    }
    /// Смещение центра масс груза относительно начала координат судна
    fn mass_shift(&self) -> Position;
}
