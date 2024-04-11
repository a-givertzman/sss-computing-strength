//! Нагрузка на судно: постоянный и переменный груз
mod tank;
mod space;

use crate::math::*;

pub use space::LoadSpace as LoadSpace;
pub use tank::Tank as Tank;

/// Абстрактный груз: контейнер, трюм или бак.
/// Имеет массу и может вернуть какая его часть попадает в указанные границы
pub trait ILoad {
    /// масса груза
    fn mass(&self, _bound: Option<Bound>) -> f64 {
        0.
    }   
    /// Смещение центра масс груза относительно начала координат судна
    fn mass_shift(&self) -> Position{
        Position::new(0., 0., 0.)
    }
    /// момент массы
    fn mass_moment(&self) -> Moment {
        Moment::from_pos(self.mass_shift(), self.mass(None))
    }
    /// момент свободной поверхности
    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::new(0., 0.)
    }
    /// Площадь парусности
    fn windage_area(&self, _bound: Option<Bound>) -> f64 {
        0.
    }
    /// Смещение центра парусности
    fn windage_shift(&self) -> Position {
        Position::new(0., 0., 0.)
    }
    /// Статический момент площади парусности палубного груза, м^3
    fn windage_moment(&self) -> Moment {
        Moment::from_pos(self.windage_shift(), self.windage_area(None))
    }
    /// Площадь горизонтальной поверхности, м^2
    fn horizontal_area(&self) -> f64 {
        0.
    }
    /// Высота груза, м
    fn height(&self) -> f64 {
        0.
    }
    /// Момент массы льда горизонтальной поверхности палубного груза относительно основания, т*м
    fn moment_ice_delta(&self, ice_mass_per_m_square: f64) -> Moment {
        Moment::new( 0., 0., self.height()*self.horizontal_area()*ice_mass_per_m_square)
    }
}
