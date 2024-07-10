//! Интерфейс для расчета дифферента

pub trait ITrim {
    /// Вычисление дифферента
    fn value(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeTrim {
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeTrim {
    //
    pub fn new(value: f64) -> Self {
        //(t/self.ship_length).atan()*180.0/PI;  
        Self { value }
    }
    //
    pub fn from_angle(value_angle: f64, ship_lenght: f64) -> Self {
        //(t/self.ship_length).atan()*180.0/PI;  
        Self { value: (value_angle*std::f64::consts::PI/180.0).tan()*ship_lenght }
    }
}
#[doc(hidden)]
impl ITrim for FakeTrim {
    fn value(&self) -> f64 {
        self.value
    }
}

