//! Интерфейс для расчета дифферента

use crate::Error;

pub trait ITrim {
    /// Вычисление средней осадки и дифферента
    fn value(&self) -> Result<(f64, f64), Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeTrim {
    mean_draught: f64,
    trim: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeTrim {
    //
    pub fn new(mean_draught: f64, trim: f64) -> Self {
        //(t/self.ship_length).atan()*180.0/PI;
        Self { mean_draught, trim }
    }
    //
    pub fn from_angle(mean_draught: f64, value_angle: f64, ship_lenght: f64) -> Self {
        //(t/self.ship_length).atan()*180.0/PI;
        Self {
            mean_draught,
            trim: (value_angle * std::f64::consts::PI / 180.0).tan() * ship_lenght,
        }
    }
}
#[doc(hidden)]
impl ITrim for FakeTrim {
    fn value(&self) -> Result<(f64, f64), Error> {
        Ok((self.mean_draught, self.trim))
    }
}
