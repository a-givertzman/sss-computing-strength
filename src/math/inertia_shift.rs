use super::curve::Curve;

///момент инерции площади свободной поверхности
pub struct InertiaMoment {    
    pub x: f64, //поперечный момент
    pub y: f64, //продольный момент
}
///
impl InertiaMoment {
    ///
    pub fn new(x: f64, y: f64 ) -> Self {
        Self { x, y }
    }
}

///класс, хранящий зависимость момента инерции от значения
pub struct InertiaShift {    
    x: Curve, //кривая поперечного момента
    y: Curve, //кривая продольного момента
}

impl InertiaShift {
    ///
    pub fn new(x: Curve, y: Curve ) -> Self {
        Self { x, y }
    }
    ///моменты инерции площади свободной поверхности (x - поперечный, y - продольный)
    pub fn value(&self, key: f64) -> InertiaMoment {
        InertiaMoment::new(self.x.value(key), self.y.value(key))
    }
}