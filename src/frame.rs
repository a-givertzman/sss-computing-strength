use crate::math::curve::Curve;

///Шпангоут
pub struct Frame {
    area: Curve, //кривая погружаемой площади, м^2
}
///
impl Frame {
    ///
    pub fn new(area: Curve) -> Frame {
        Self { area }
    }
    ///погруженная площадь сечения
    pub fn area(&self, draft: f64) -> f64 {
        self.area.value(draft)
    }
}
