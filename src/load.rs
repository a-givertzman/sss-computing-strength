//! Нагрузка на судно: постоянный и переменный груз
use crate::math::{
    bound::Bound, mass_moment::MassMoment, position::Position, surface_moment::SurfaceMoment,
};
///
/// Абстрактный груз: контейнер, трюм или бак.
/// Имеет массу и может вернуть какая его часть попадает в указанные границы
pub trait ILoad {
    /// Центер масс груза
    fn center(&self) -> Position;
    /// Масса груза
    fn mass(&self, bound: Option<Bound>) -> f64;
    /// Момент массы груза
    fn moment_mass(&self) -> MassMoment {
        MassMoment::from_pos(self.center(), self.mass(None))
    }
    /// Момент свободной поверхности для жидкого груза
    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::new(0., 0.)
    }
}
/// Груз, контенер, трюм и т.п. твердый груз, имеет границы, центр масс и значение
pub struct LoadSpace {
    /// Масса
    mass: f64,
    /// Границы
    bound: Bound,
    /// Смещение центра масс относительно начала координат судна
    center: Position,
}
///
#[allow(dead_code)]
impl LoadSpace {
    ///
    pub fn new(mass: f64, bound: Bound, center: Position) -> Self {
        assert!(
            bound.start() < center.x(),
            "bound.start {} < pos.x {}",
            bound.start(),
            center.x()
        );
        assert!(
            bound.end() > center.x(),
            "bound.end {} > pos.x {}",
            bound.end(),
            center.x()
        );
        Self {
            bound,
            center,
            mass,
        }
    }
}
///
impl ILoad for LoadSpace {
    fn mass(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound.part_ratio(&bound) * self.mass
        } else {
            self.mass
        }
    }

    fn center(&self) -> Position {
        self.center
    }
}
