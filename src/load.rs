use crate::math::{bound::Bound, mass_moment::MassMoment, position::Position, surface_moment::SurfaceMoment};


///абстрактный груз, имеет массу и может вернуть какая его часть попадает в указанные границы
pub trait ILoad {
    ///центер масс груза
    fn center(&self) -> Position;
    ///масса груза
    fn mass(&self, bound: Option<Bound>) -> f64;
    ///момент массы
    fn moment_mass(&self) -> MassMoment {
        MassMoment::from_pos(self.center(), self.mass(None))
    }
    ///момент свободной поверхности
    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::new(0., 0.,)
    }
}

///груз на судне, имеет границы, центр масс и значение
pub struct LoadSpace {
    bound: Bound,  
    center: Position,
    mass: f64,      
}

impl LoadSpace {
    ///
    pub fn new(bound: Bound, center: Position, mass: f64) -> Self {
        assert!(bound.start() < center.x(), "bound.start {} < pos.x {}", bound.start(), center.x());
        assert!(bound.end() > center.x(), "bound.end {} > pos.x {}", bound.end(), center.x());
        Self { bound, center, mass }
    }
}

impl ILoad for LoadSpace {
    fn mass(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
        self.bound.part_ratio(&bound)*self.mass
        } else {
            self.mass
        }
    }

    fn center(&self) -> Position {
        self.center
    }
}
