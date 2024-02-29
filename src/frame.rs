//! Шпангоут
use crate::math::curve::{Curve, ICurve};

///Шпангоут, содержит кривую погруженной площади сечения в зависимости от осадки
pub struct Frame {
    /// Смещение относительно миделя
    delta_x: f64,
    /// Кривая погружаемой площади, $м^2$
    area: Curve,
}
///
impl Frame {
    ///
    pub fn new( delta_x: f64, area: Curve ) -> Frame {
        Self { delta_x, area }
    }
    /// Расчет погруженной площади сечения
    /// - draft: осадка судна в районе шпангоута
    pub fn area(&self, draft: f64) -> f64 {
        self.area.value(draft)
    }
    /// Смещение относительно миделя
    pub fn delta_x(&self) -> f64 {
        self.delta_x
    }
}
