use crate::math::curve::Curve;

///Шпангоут
pub struct Frame {
    area: Curve, //кривая погружаемой площади, м^2
    x: f64,      //расстояние от начала координат, м
}

impl Frame {
    ///
    pub fn new(area: Curve, x: f64) -> Frame {
        Self { area, x }
    }
    ///погруженная площадь сечения
    pub fn area(&self, draft: f64) -> f64 {
        self.area.value(draft)
    }
    ///расстояние от центра корабля  
    pub fn x(&self) -> f64 {
        self.x
    }
}


///Шпангоут
pub struct Frame {
    area: Curve, //кривая погружаемой площади, м^2
    x: f64,      //расстояние от начала координат, м
}

impl Frame {
    ///
    pub fn new(area: Curve, x: f64) -> Frame {
        Self { area, x }
    }
    ///погруженная площадь сечения
    pub fn area(&self, draft: f64) -> f64 {
        self.area.value(draft)
    }
    ///расстояние от центра корабля  
    pub fn x(&self) -> f64 {
        self.x
    }
}
