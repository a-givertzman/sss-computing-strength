use std::{iter::Sum, ops::Add};

use super::position::Position;

pub type Moment = super::position::Position;

///класс инкапсулирующий момент массы
impl Moment {
    ///рассчет момента из позиции и массы груза
    pub fn from_pos(position: Position, mass: f64) -> Self {
        Self::new(position.x()*mass, position.y()*mass, position.z()*mass,)
    }
    ///рассчет отстояния центра масс момента
    pub fn to_pos(&self, mass: f64) -> Position {
        Position::new(self.x()/mass, self.y()/mass, self.z()/mass)
    }
}

impl Add for Moment {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Moment::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z(),)
    }
}

impl Sum for Moment {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0., 0., 0.),|a, b| a + b )
    }
}
