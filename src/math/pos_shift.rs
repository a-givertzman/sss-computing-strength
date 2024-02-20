use super::{curve::Curve, position::Position};


///класс, хранящий зависимость положения точки от значения
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