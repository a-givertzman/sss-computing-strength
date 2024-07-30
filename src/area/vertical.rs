//! Распределение площади парусности
use crate::{Bound, Error};

/// Распределение площадь парусности
#[derive(Debug, Clone)]
pub struct VerticalArea {
    /// Значение площади, м^2
    value: f64,
  
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
        bound_x: Bound,
    ) -> Self {
        Self {
            value,
            bound_x,
        }
    }
    /// Площадь попадающая в Bound или вся если Bound отсутствует
    pub fn value(&self, bound: &Bound) -> Result<f64, Error> {
        Ok(self.bound_x.part_ratio(bound)? * self.value)
    }     
}
