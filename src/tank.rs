use crate::{load::ILoad, math::{bound::Bound, inertia_shift::InertiaShift, pos_shift::PosShift, position::Position, surface_moment::SurfaceMoment}};

///цистерна, помимо распределение массы имеет свойства инерции жидкости
pub struct Tank {    
    density: f64, //плотность     
    volume: f64,  //объем 
    bound: Bound,  //границы
    center: PosShift, //координаты центра объема жидкости в цистерне в системе координат судна
    free_surf_inertia: InertiaShift, //момент инерции площади свободной поверхности жидкости
}

impl Tank {
    ///
    pub fn new(density: f64, volume: f64, bound: Bound, center: PosShift, free_surf_inertia: InertiaShift, ) -> Self {
        assert!(density > 0., "density {} > 0", density);
        assert!(volume >= 0., "volume {} >= 0", volume);
        Self { density, volume, bound, center, free_surf_inertia }
    }
    ///
    pub fn new_empty(density: f64, bound: Bound, center: PosShift, free_surf_inertia: InertiaShift,) -> Self {
        Self::new(density, 0., bound, center, free_surf_inertia)
    }
    ///
    fn mass(&self) -> f64 {
        self.volume*self.density
    }
}

impl ILoad for Tank {
    fn mass(&self, bound: Option<Bound>) -> f64 {
        self.volume*self.density*if let Some(bound) = bound {
            self.bound.part_ratio(&bound)
        } else {
            1.
        }
    }

    fn center(&self) -> Position {
        self.center.value(self.volume)
    }

    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::from_inertia(self.free_surf_inertia.value(self.volume), self.density)
    }
}
