//! Площадь парусности
use crate::{Bound, Moment, Position};

/// Площадь парусности
#[derive(Debug, Clone)]
pub struct VerticalArea {
    /// Значение площади, м^2
    value: f64,
    /// Смещение центра
    shift_z: f64,    
    /// Ограничение по оси Х
    bound_x: Bound,
}
///
impl VerticalArea {
    /// Основной конструктор
    /// * value - Значение площади, м^2
    /// * shift_z - Смещение центра по оси Z 
    /// * bound_x - Ограничение по оси Х
    pub fn new(    
        value: f64,
        shift_z: f64,    
        bound_x: Bound,
    ) -> Self {
        Self {
            value,
            shift_z,
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
        Moment::from_pos(Position::new(self.bound_x.center(), 0., self.shift_z), self.value)
    }
}
