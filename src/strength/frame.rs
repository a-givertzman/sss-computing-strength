//! Шпангоут
use crate::math::{Curve, ICurve};

///Шпангоут, содержит кривую погруженной площади сечения в зависимости от осадки
pub struct Frame {
    /// Смещение относительно миделя, $м$
    shift_x: f64,
    /// Кривая погружаемой площади, $м^2$
    area: Curve, 
}
///
impl Frame {
    ///
    pub fn new(shift_x: f64, area: Curve) -> Frame {
        Self { shift_x, area }
    }
    /// Расчет погруженной площади сечения
    /// - draft: осадка судна в районе шпангоута
    pub fn area(&self, draft: f64) -> f64 {
        self.area.value(draft)
    }
    /// Смещение относительно миделя
    pub fn shift_x(&self) -> f64 {
        self.shift_x
    }
}
