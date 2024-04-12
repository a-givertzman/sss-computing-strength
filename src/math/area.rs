//! Площадь
use crate::Bound;

/// Площадь
#[derive(Debug)]
pub struct Area {
    /// Значение площади, м^2
    value: f64,
    /// Смещение центра по оси Х
    shift_x: Option<f64>,    
    /// Ограничение по оси Х
    bound_x: Bound,
}
///
impl Area {
    /// Основной конструктор
    /// * value - Значение площади, м^2
    /// * shift_x - Смещение центра по оси Х
    /// * bound_x - Ограничение по оси Х
    pub fn new(    
        value: f64,
        shift_x: Option<f64>,
        bound_x: Bound,
    ) -> Self {
        Self {
            value,
            shift_x,
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
}
