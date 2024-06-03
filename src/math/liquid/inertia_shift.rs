//! Кривая момента инерции
use crate::math::curve::{Curve, ICurve};
use super::inertia_moment::InertiaMoment;

/// 
/// To be added...
#[derive(Clone)]
pub struct InertiaShift {    
    x: Curve, //кривая поперечного момента
    y: Curve, //кривая продольного момента
}
///
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
///
impl IInertiaShift for InertiaShift {
    ///моменты инерции площади свободной поверхности (x - поперечный, y - продольный)
    fn value(&self, key: f64) -> InertiaMoment {
        InertiaMoment::new(self.x.value(key), self.y.value(key))
    }   
}
#[doc(hidden)]
pub trait IInertiaShift {
    ///моменты инерции площади свободной поверхности (x - поперечный, y - продольный)
    fn value(&self, key: f64) -> InertiaMoment;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeInertiaShift {
    ///моменты инерции площади свободной поверхности (x - поперечный, y - продольный)
    value: InertiaMoment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeInertiaShift {
    pub fn new(
        value: InertiaMoment,
    ) -> Self {
        Self {
            value,
        }
    }
}
#[doc(hidden)]
impl IInertiaShift for FakeInertiaShift {
    ///
    fn value(&self, _: f64) -> InertiaMoment {
        self.value.clone()
    }    
}


