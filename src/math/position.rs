//! Точка относительно Центра 
use std::{iter::Sum, ops::{Add, AddAssign}};

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64
}
///
impl Position {
    ///
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    /// Дополнительный конструктор  
    /// * (f64, f64) - (начало диапазона, конец диапазона)
    pub fn from(v: (f64, f64, f64)) -> Self {
        Self::new(
            v.0,
            v.1,
            v.2,
        )
    }
    ///
    pub fn x(&self) -> f64 {
        self.x
    }
    ///
    pub fn y(&self) -> f64 {
        self.y
    }
    ///
    pub fn z(&self) -> f64 {
        self.z
    }
}
///
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position({}, {}, {})", self.x(), self.y(), self.z())
    }
}
///
impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z(),)
    }
}
///
impl Sum for Position {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0., 0., 0.),|a, b| a + b )
    }
}
///
impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

