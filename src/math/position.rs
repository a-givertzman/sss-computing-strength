//! Точка относительно Центра Судна
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
