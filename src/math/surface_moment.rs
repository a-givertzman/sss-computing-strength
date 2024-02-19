use std::{iter::Sum, ops::Add};

use super::inertia_shift::InertiaMoment;

///класс инкапсулирующий момент свободной поверхности
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceMoment {
    x: f64,
    y: f64,
}
///
impl SurfaceMoment {
    ///
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    ///рассчет момента свободной поверхности из момента инерции и плотности жидкости
    pub fn from_inertia(inertia_moment: InertiaMoment, density: f64) -> Self {
        Self::new(inertia_moment.x*density, inertia_moment.y*density)
    }
    ///
    pub fn x(&self) -> f64 {
        self.x
    }
    ///
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl Add for SurfaceMoment {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SurfaceMoment::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Sum for SurfaceMoment {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0., 0., ),|a, b| a + b )
    }
}
