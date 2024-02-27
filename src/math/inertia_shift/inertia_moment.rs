//! Момент инерции площади свободной поверхности
#[derive(Clone)]
pub struct InertiaMoment {    
    pub x: f64, //поперечный момент
    pub y: f64, //продольный момент
}
///
impl InertiaMoment {
    ///
    /// x - поперечный, y - продольный
    pub fn new(x: f64, y: f64 ) -> Self {
        Self { x, y }
    }
}
