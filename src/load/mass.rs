//! Масса груз
use crate::math::*;

/// Абстрактная масса груза.
/// Может вернуть какая масса попадает в указанные границы
pub struct LoadMass {
    /// Общая масса
    mass: f64,
    /// Смещение центра масс
    mass_shift: Option<Position>,
    /// Границы массы
    bound_x: Bound,
    bound_y: Option<Bound>,
    bound_z: Option<Bound>,
}

#[allow(dead_code)]
impl LoadMass {
    /// Основной конструктор
    /// * mass - Общая масса груза
    /// * mass_shift - Смещение центра масс
    /// * bound_x - границы груза вдоль продольной оси
    /// * bound_y - границы груза вдоль попересной оси
    /// * bound_z - границы груза вдоль вертикальной оси
    pub fn new(
        mass: f64,
        mass_shift: Option<Position>,
        bound_x: Bound,
        bound_y: Option<Bound>,
        bound_z: Option<Bound>,
    ) -> Self {
        assert!(mass_shift.is_some() || (bound_y.is_some() && bound_z.is_some()),
        "LoadMass new mass_shift.is_some() || (bound_y.is_some() && bound_z.is_some()" );
        Self {
            mass,
            mass_shift,
            bound_x,
            bound_y,
            bound_z,
        }
    }
    /// Дополнительный конструктор
    /// * mass - Общая масса груза
    /// * mass_shift - Смещение центра масс
    /// * bound_x - границы груза вдоль продольной оси
    pub fn from(
        mass: f64,
        mass_shift: Option<Position>,
        bound_x: (f64, f64),
        bound_y: Option<(f64, f64)>,
        bound_z: Option<(f64, f64)>,
    ) -> Self {
        assert!(mass_shift.is_some() || (bound_y.is_some() && bound_z.is_some()),
        "LoadMass from mass_shift.is_some() || (bound_y.is_some() && bound_z.is_some())" );
        Self::new (
            mass,
            mass_shift,
            Bound::from(bound_x),
            bound_y.map(|v| Bound::from(v)),
            bound_y.map(|v| Bound::from(v)),
        )
    }
}
///
impl LoadMass {
    /// Масса груза, попадающая в Bound или вся если Bound не заданно
    pub fn value(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound_x.part_ratio(&bound) * self.mass
        } else {
            self.mass
        }
    }
    /// Смещение центра масс груза относительно начала координат судна
    pub fn shift(&self) -> Position {
        if let Some(mass_shift) = self.mass_shift.clone() {
            mass_shift
        } else {
            Position::new(
                self.bound_x.center(), 
                self.bound_y.expect("LoadMass shift error: no bound_y!").center(),
                self.bound_y.expect("LoadMass shift error: no bound_z!").center(),
            )
        }
    }
}
