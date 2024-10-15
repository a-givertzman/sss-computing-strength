//! Площадь горизонтальной поверхности для расчета прочности
use crate::{Bound, Error};
/// Площадь горизонтальной поверхности для расчета прочности
#[derive(Debug, Clone)]
pub struct HAreaStrength {
    /// Значение площади, м^2
    value: f64,  
    /// Ограничение по оси Х
    bound_x: Bound,
}
//
impl HAreaStrength {
    /// Основной конструктор
    /// * value - Значение площади, м^2
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
