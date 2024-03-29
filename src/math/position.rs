//! Точка относительно Центра Судна
#[derive(Clone, Copy, Debug, PartialEq)]
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
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
