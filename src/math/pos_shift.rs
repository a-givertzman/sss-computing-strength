//! Зависимость положения точки от некоторого значения
use super::{curve::Curve, position::Position};
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
    ///
    pub fn value(&self, key: f64) -> Position {
        Position::new(self.x.value(key), self.y.value(key), self.z.value(key))
    }
}