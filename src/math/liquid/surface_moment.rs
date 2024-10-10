//! Момент свободной поверхности жидкости
use std::{iter::Sum, ops::Add};

use super::inertia_moment::InertiaMoment;

/// Момент свободной поверхности жидкости
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreeSurfaceMoment {
    x: f64, // поперечный
    y: f64, // продольный
}
//
impl FreeSurfaceMoment {
    /// Конструктор
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    /// Рассчет момента свободной поверхности из момента инерции и плотности жидкости
    pub fn from_inertia(inertia_moment: InertiaMoment, density: f64) -> Self {
        Self::new(inertia_moment.x*density, inertia_moment.y*density)
    }
    //
    pub fn x(&self) -> f64 {
        self.x
    }
    //
    pub fn y(&self) -> f64 {
        self.y
    }
}
//
impl Add for FreeSurfaceMoment {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        FreeSurfaceMoment::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}
//
impl Sum for FreeSurfaceMoment {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0., 0., ),|a, b| a + b )
    }
}
//
impl std::fmt::Display for FreeSurfaceMoment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y(),)
    }
}
