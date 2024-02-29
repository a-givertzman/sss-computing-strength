//! Груз со свойством жидкости - цистерна
use crate::{
    load::ILoad,
    math::{
        bound::Bound,
        inertia_shift::inertia_shift::InertiaShift,
        pos_shift::{IPosShift, PosShift},
        position::Position,
        surface_moment::SurfaceMoment,
    },
};
/// Цистерна, реализует интерфейс ILoad.  
/// Помимо массы имеет свойства свободной поверхности жидкости
/// и смещение центра тяжести в зависимости от объема жидкости.
#[derive(Clone)]
pub struct Tank {
    /// Плотность жидкости в цистерне
    density: f64,
    /// Объем жидкости в цистерне
    volume: f64,
    /// Границы цистерны
    bound: Bound,
    /// Кривая координат центра объема жидкости в цистерне в системе координат судна
    center: PosShift,
    /// Кривая момента инерции площади свободной поверхности жидкости
    free_surf_inertia: InertiaShift,
}
///
impl Tank {
    /// Основной конструктор
    pub fn new(
        density: f64,
        volume: f64,
        bound: Bound,
        center: PosShift,
        free_surf_inertia: InertiaShift,
    ) -> Self {
        assert!(density > 0., "density {} > 0", density);
        assert!(volume >= 0., "volume {} >= 0", volume);
        Self {
            density,
            volume,
            bound,
            center,
            free_surf_inertia,
        }
    }
}
///
impl ILoad for Tank {
    ///
    fn mass(&self, bound: Option<Bound>) -> f64 {
        self.volume
            * self.density
            * if let Some(bound) = bound {
                self.bound.part_ratio(&bound)
            } else {
                1.
            }
    }
    ///
    fn center(&self) -> Position {
        self.center.value(self.volume)
    }
    ///
    fn moment_surface(&self) -> SurfaceMoment {
        let result =
            SurfaceMoment::from_inertia(self.free_surf_inertia.value(self.volume), self.density);
        log::debug!("\t Tank result:{:?}", result);
        result
    }
}
