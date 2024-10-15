//! Момент
use super::position::Position;
pub type Moment = super::position::Position;
/// Момент
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
