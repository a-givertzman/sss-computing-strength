//! Поправка к продольной метацентрической высоте на  
//! влияние свободной поверхности жидкости в цистернах
use std::{fmt::Display, ops::Add};

use crate::math::FreeSurfaceMoment;

/// Поправка к продольной метацентрической высоте на  
/// влияние свободной поверхности жидкости в цистернах
#[derive(Copy, Clone)]
pub struct DeltaMH {
    /// Продольная составляющая
    pub long: f64,
    /// Поперечная составляющая
    pub trans: f64,
}
//
impl Add for DeltaMH {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            long: self.long + rhs.long,
            trans: self.trans + rhs.trans,
        }
    }
}
//
impl DeltaMH {
    ///рассчет отстояния Центра Масс момента
    pub fn new(long: f64, lat: f64) -> Self {
        Self { long, trans: lat }
    }
    ///рассчет поправки из момента свободной поверхности
    pub fn from_moment(moment: FreeSurfaceMoment, mass: f64) -> Self {
        Self::new(moment.x() / mass, moment.y() / mass)
    }
    /// Продольная составляющая
    pub fn long(&self) -> f64 {
        self.long
    }
    /// Поперечная составляющая
    pub fn trans(&self) -> f64 {
        self.trans
    }
}
//
impl Display for DeltaMH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeltaMH: (long:{}, trans:{})", self.long(), self.trans())
    }
}
