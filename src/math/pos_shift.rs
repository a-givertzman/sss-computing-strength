//! Зависимость положения точки от некоторого значения
use super::{curve::{Curve, ICurve}, position::Position};
/// Зависимость положения точки от некоторого значения.
/// Интерполирует значение по ключу.
///
/// # Example
///
/// ```
/// # #![allow(unused_mut)]
/// let mut res = PosShift::new(
///    Curve::new(vec![(0., 0.), (2., 2.)]),
///    Curve::new(vec![(2., 0.), (0., 2.)]),
///    Curve::new(vec![(1., 0.), (-1., 2.)]),
/// ).value(1.);
/// asserteq!(res, Position::new(1., 1., 0.));
/// ```
///
#[derive(Clone)]
pub struct PosShift {    
    x: Curve, 
    y: Curve, 
    z: Curve,
}

impl PosShift {
    ///
    pub fn new(x: Curve, y: Curve, z: Curve ) -> Self {
        Self { x, y, z }
    }
}

impl IPosShift for PosShift {
    ///
    fn value(&self, key: f64) -> Position {
        Position::new(self.x.value(key), self.y.value(key), self.z.value(key))
    }
}

#[doc(hidden)]
pub trait IPosShift {
    fn value(&self, key: f64) -> Position;
}
#[doc(hidden)]
/// заглушка для тестирования
pub struct FakePosShift {
    data: Position,
}
#[doc(hidden)]
impl FakePosShift {
    pub fn new(data: Position) -> Self {
        Self { data }
    }
}
#[doc(hidden)]
impl IPosShift for FakePosShift {
    fn value(&self, _: f64) -> Position {
        self.data.clone()
    }
}