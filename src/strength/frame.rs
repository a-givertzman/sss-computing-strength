//! Шпангоут
use crate::{math::{Curve, ICurve}, Error};
///Шпангоут, содержит кривую погруженной площади сечения в зависимости от осадки
pub struct Frame {
    /// Смещение относительно миделя, $м$
    shift_x: f64,
    /// Кривая погружаемой площади, $м^2$
    area: Curve,
}
//
impl Frame {
    /// Главный конструктор
    /// * shift_x - смещение шпангоута относительно миделя по оси Х, м
    /// * area - кривая погружаемой площади, м^2
    pub fn new(shift_x: f64, area: Curve) -> Frame {
        Self { shift_x, area }
    }
    /// Расчет погруженной площади сечения
    /// - draft: осадка судна в районе шпангоута
    pub fn area(&self, draft: f64) -> Result<f64, Error> {
        self.area.value(draft)
    }
    /// Смещение относительно миделя
    pub fn shift_x(&self) -> f64 {
        self.shift_x
    }
}
