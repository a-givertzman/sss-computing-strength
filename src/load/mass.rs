//! Масса груз
use crate::{math::*, ILoad};

/// Абстрактная масса груза.
/// Может вернуть какая масса попадает в указанные границы
pub trait ILoadMass: ILoad {
    /// Масса груза, попадающая в Bound или вся если Bound не заданно
    fn value(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound_x().part_ratio(&bound) * self.mass()
        } else {
            self.mass()
        }
    }
    /// Статический момент массы
    fn moment(&self) -> Moment {
        Moment::from_pos(self.shift(), self.mass())
    }
}

/// Абстрактный груз - заглушка для учета массы
pub struct LoadMass { 
    /// Масса груза 
    mass: f64,
    /// Границы груза 
    bound_x: Bound,
    /// Смещение центра
    shift: Option<Position>,
}
///
impl LoadMass {
    /// Основной конструктор
    /// * mass - Масса груза
    /// * bound_x - границы груза вдоль продольной оси
    /// * shift - Смещение центра
    pub fn new(
        mass: f64,
        bound_x: Bound,
        shift: Option<Position>,
    ) -> Self {
        Self {
            mass,
            bound_x,
            shift,
        }
    }
}
///
impl ILoad for LoadMass {
    ///
    fn mass(&self) -> f64 {
        self.mass
    }
    ///
    fn bound_x(&self) -> Bound {
        self.bound_x
    }
    ///
    fn shift(&self) -> Position {
        if let Some(shift) = self.shift.clone() {
            shift
        } else {
            Position::new(self.bound_x.center(), 0., 0.,)
        }
    }
}
///
impl ILoadMass for LoadMass {}


