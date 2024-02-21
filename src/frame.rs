//! Шпангоут
use crate::math::curve::Curve;

///Шпангоут, содержит кривую погруженной площади сечения в зависимости от осадки
pub struct Frame {
    /// кривая погружаемой площади, $м^2$
    area: Curve, 
}
///
impl Frame {
    ///
    pub fn new(area: Curve) -> Frame {
        Self { area }
    }
    /// Расчет погруженной площади сечения
    /// - draft: осадка судна в районе шпангоута
    pub fn area(&self, draft: f64) -> f64 {
        self.area.value(draft)
    }
}
