//! Момент
use std::{iter::Sum, ops::Add};

use super::position::Position;

pub type Moment = super::position::Position;
///
impl Moment {
    /// Рассчет момента из позиции и значения
    pub fn from_pos(position: Position, value: f64) -> Self {
        Self::new(position.x()*value, position.y()*value, position.z()*value,)
    }
    /// Рассчет отстояния центра момента
    pub fn to_pos(&self, value: f64) -> Position {
        Position::new(self.x()/value, self.y()/value, self.z()/value)
    }
    /// Масштабирование момента на константу
    pub fn scale(&self, value: f64) -> Self {
        Self::new(self.x()*value, self.y()*value, self.z()*value)
    }
}
///
impl Add for Moment {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Moment::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z(),)
    }
}
///
impl Sum for Moment {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0., 0., 0.),|a, b| a + b )
    }
}
