//! Площадь
use crate::{Bound, Moment, Position};

/// Площадь
#[derive(Debug, Clone)]
pub struct Area {
    /// Значение площади, м^2
    value: f64,
    /// Смещение центра
    shift: Option<Position>,    
    /// Ограничение по оси Х
    bound_x: Bound,
}
///
impl Area {
    /// Основной конструктор
    /// * value - Значение площади, м^2
    /// * shift_z - Смещение центра по оси Z 
    /// * bound_x - Ограничение по оси Х
    pub fn new(    
        value: f64,
        shift: Option<Position>,    
        bound_x: Bound,
    ) -> Self {
        Self {
            value,
            shift,
            bound_x,
        }
    }
    /// Площадь попадающая в Bound или вся если Bound отсутствует
    pub fn value(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound_x.part_ratio(&bound) * self.value
        } else {
            self.value
        }
    }     
    /// Момент площади 
    pub fn moment(&self) -> Moment {
        Moment::from_pos(self.shift.unwrap_or(Position::new(self.bound_x.center(), 0., 0.)), self.value)
    }
}
