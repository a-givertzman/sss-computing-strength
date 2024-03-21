//! Поправка к продольной метацентрической высоте на  
//! влияние свободной поверхности жидкости в цистернах 
use crate::SurfaceMoment;

/// Поправка к продольной метацентрической высоте на  
/// влияние свободной поверхности жидкости в цистернах 
#[derive(Clone)]
pub struct DeltaMH {
    /// Продольная составляющая
    pub long: f64, 
    /// Поперечная составляющая
    pub cross: f64,
}

impl DeltaMH {
    ///рассчет отстояния Центра Масс момента
    pub fn new(long: f64, lat: f64) -> Self {
        Self{ long, cross: lat }
    }
    ///рассчет поправки из момента свободной поверхности
    pub fn from_moment(moment: SurfaceMoment, mass: f64) -> Self {
        Self::new(moment.x()/mass, moment.y()/mass)
    }
    /// Продольная составляющая
    pub fn long(&self) -> f64 {
        self.long
    }
    /// Поперечная составляющая
    pub fn cross(&self) -> f64 {
        self.cross
    }
}