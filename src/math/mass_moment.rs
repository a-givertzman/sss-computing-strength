//! Момент массы
use std::{iter::Sum, ops::Add};

use super::position::Position;

pub type MassMoment = super::position::Position;
///
impl MassMoment {
    ///рассчет момента из позиции и массы груза
    pub fn from_pos(position: Position, mass: f64) -> Self {
        Self::new(position.x()*mass, position.y()*mass, position.z()*mass,)
    }
    ///рассчет отстояния центра масс момента
    pub fn to_pos(&self, mass: f64) -> Position {
        Position::new(self.x()/mass, self.y()/mass, self.z()/mass)
    }
}

impl Add for MassMoment {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MassMoment::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z(),)
    }
}

impl Sum for MassMoment {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0., 0., 0.),|a, b| a + b )
    }
}
