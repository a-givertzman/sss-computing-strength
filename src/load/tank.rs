//! Цистерна с жидкостью
use std::rc::Rc;

use crate::{math::*, ILoad, ILoadMass};

use self::inertia_shift::IInertiaShift;


/// Цистерна с жидкостью.
/// Имеет свойства свободной поверхности жидкости.
pub trait ITank: ILoad {
    /// Момент свободной поверхности 
    fn moment_surface(&self) -> FreeSurfaceMoment;
}
/// Цистерна с жидкостью.
/// Имеет свойства свободной поверхности жидкости.
#[derive(Clone)]
pub struct Tank {
    /// Плотность жидкости в цистерне
    density: f64,
    /// Объем жидкости в цистерне
    volume: f64,
    /// Границы
    bound_x: Bound,
    /// Зависимость отстояния центра величины от объема
    center: Rc<dyn IPosShift>,    
    /// Кривая поперечного момента инерции площади свободной поверхности жидкости
    inertia: Rc<dyn IInertiaShift>,
}
///
impl Tank {
    /// Основной конструктор
    /// * density - Плотность жидкости в цистерне
    /// * volume - Объем жидкости в цистерне
    /// * bound_x - Границы цистерны по оси Х
    /// * center - Кривая координат центра объема жидкости в цистерне в системе координат судна
    /// * inertia - Кривая момента инерции площади свободной поверхности жидкости
    pub fn new(
        density: f64,
        volume: f64,
        bound_x: Bound,
        center: Rc<dyn IPosShift>, 
        inertia: Rc<dyn IInertiaShift>,
    ) -> Self {
        assert!(density > 0., "density {} > 0", density);
        assert!(volume >= 0., "volume {} >= 0", volume);
        Self {
            density,
            volume,
            bound_x,
            center,
            inertia,
        }
    }
}
///
impl ITank for Tank {
    /// Момент свободной поверхности 
    fn moment_surface(&self) -> FreeSurfaceMoment {
        let result =
            FreeSurfaceMoment::from_inertia(self.inertia.value(self.volume), self.density);
        log::info!("\t Tank result:{:?}", result);    
        result
    }
}
///
impl ILoad for Tank {
    ///
    fn mass(&self) -> f64 {
        self.density*self.volume
    }
    ///
    fn bound_x(&self) -> Bound {
        self.bound_x
    }    
    ///
    fn shift(&self) -> Position {
        self.center.value(self.volume)
    } 
}
///
impl ILoadMass for Tank{}
