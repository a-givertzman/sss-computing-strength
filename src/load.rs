use crate::math::{bound::Bound, moment::Moment, position::Position};


///абстрактный груз, имеет массу и может вернуть какая его часть попадает в указанные границы
pub trait ILoad {
    ///масса груза
    fn mass(&self, bound: Option<Bound>) -> f64;
    ///момент массы
    fn moment(&self) -> Moment;
}

///груз на судне, имеет границы, центр масс и значение
pub struct LoadSpace {
    bound: Bound,  
    pos: Position,
    mass: f64,      
}

impl LoadSpace {
    ///
    pub fn new(bound: Bound, pos: Position, mass: f64) -> Self {
        assert!(bound.start() < pos.x(), "bound.start {} < pos.x {}", bound.start(), pos.x());
        assert!(bound.end() > pos.x(), "bound.end {} > pos.x {}", bound.end(), pos.x());
        Self { bound, pos, mass }
    }
    ///
    pub fn new_empty(bound: Bound, pos: Position) -> Self {
        Self::new(bound, pos, 0.)
    }
}

impl ILoad for LoadSpace {
    fn mass(&self, bound: Option<Bound>) -> f64 {
        let bound = bound.unwrap_or(self.bound);
        self.bound.part_ratio(&bound)*self.mass
    }

    fn moment(&self) -> Moment {
        Moment::from_pos(self.pos, self.mass)
    }
}
