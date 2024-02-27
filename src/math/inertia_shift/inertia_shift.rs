//! Кривая момента инерции
use crate::math::{
    curve::{Curve, ICurve},
    inertia_shift::inertia_moment::InertiaMoment,
};

/// 
/// To be added...
#[derive(Clone)]
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