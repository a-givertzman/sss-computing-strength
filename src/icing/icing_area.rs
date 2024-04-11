//! Площадь обледенения
use crate::Bound;

/// Площадь обледенения
#[derive(Debug)]
pub struct IcingArea {
    /// Значение площади, м^2
    area: f64,
    /// Ограничение по оси Х
    bound_x: Bound,
}
///
impl IcingArea {
    /// Основной конструктор
    /// * area_value - Значение площади, м^2
    /// * bound_x - Ограничение по оси Х
    pub fn new(    
        area: f64,
        bound_x: Bound,
    ) -> Self {
        Self {
            area,
            bound_x,
        }
    }
    /// Площадь попадающая в Bound или вся если Bound отсутствует
    pub fn area(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound_x.part_ratio(&bound) * self.area
        } else {
            self.area
        }
    }     
}
