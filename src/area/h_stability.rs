//! Площадь горизонтальной поверхности для расчета остойчивости
use crate::{Moment, Position};

/// Площадь горизонтальной поверхности для расчета остойчивости
#[derive(Debug, Clone)]
pub struct HAreaStability {
    /// Значение площади, м^2
    value: f64,
    /// Смещение центра
    shift: Position,    
}
///
impl HAreaStability {
    /// Основной конструктор
    /// * value - Значение площади, м^2
    /// * shift - Смещение центра
    pub fn new(    
        value: f64,
        shift: Position,    
    ) -> Self {
        Self {
            value,
            shift,
        }
    }   
    /// Момент площади 
    pub fn moment(&self) -> Moment {
        Moment::from_pos(self.shift.clone(), self.value)
    }
}
