//! Груз - цистерна с жидкостью
use crate::math::*;

/// Груз - цистерна с жидкостью, реализует интерфейс ILoad.  
/// Помимо массы имеет свойства свободной поверхности жидкости.
#[derive(Clone)]
pub struct Tank {    
    /// плотность жидкости в цистерне 
    density: f64,     
    /// объем жидкости в цистерне
    volume: f64,  
    /// границы цистерны
    bound: Bound,  
    /// кривая координат центра объема жидкости в цистерне в системе координат судна
    center: PosShift, 
    /// кривая момента инерции площади свободной поверхности жидкости
    free_surf_inertia: InertiaShift,
}
///
impl Tank {
    ///
    pub fn new(density: f64, volume: f64, bound: Bound, center: PosShift, free_surf_inertia: InertiaShift, ) -> Self {
        assert!(density > 0., "density {} > 0", density);
        assert!(volume >= 0., "volume {} >= 0", volume);
        Self { density, volume, bound, center, free_surf_inertia }
    }
}
///
impl super::load::ILoad for Tank {
    fn mass(&self, bound: Option<Bound>) -> f64 {
        self.volume*self.density*if let Some(bound) = bound {
            self.bound.part_ratio(&bound)
        } else {
            1.
        }
    }

    fn mass_shift(&self) -> Position {
        self.center.value(self.volume)
    }

    fn moment_surface(&self) -> SurfaceMoment {
        let result = SurfaceMoment::from_inertia(self.free_surf_inertia.value(self.volume), self.density);
        log::info!("\t Tank result:{:?}", result);
        result
    }
}
