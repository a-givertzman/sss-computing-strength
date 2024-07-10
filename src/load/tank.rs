//! Цистерна с жидкостью
use crate::{math::*, ILoad, ILoadMass, LoadingType};

/// Цистерна с жидкостью.
/// Имеет свойства свободной поверхности жидкости.
pub trait ITank: ILoad {
    /// Момент свободной поверхности 
    fn moment_surface(&self) -> FreeSurfaceMoment;
    /// Тип груза
    fn load_type(&self) -> LoadingType;
}
/// Цистерна с жидкостью.
/// Имеет свойства свободной поверхности жидкости.
#[derive(Clone, Debug)]
pub struct Tank {
    /// Плотность жидкости в цистерне
    density: f64,
    /// Объем жидкости в цистерне
    volume: f64,
    /// Границы
    bound_x: Bound,
    /// Отстояние центра величины от объема
    shift: Option<Position>, 
    /// Поперечный момент инерции площади свободной поверхности жидкости
    inertia: InertiaMoment,
    /// Тип груза
    load_type: LoadingType,
}
///
impl Tank {
    /// Основной конструктор
    /// * density - Плотность жидкости в цистерне
    /// * volume - Объем жидкости в цистерне
    /// * bound_x - Границы цистерны по оси Х
    /// * shift - Отстояние центра величины от объема
    /// * inertia - Поперечный момент инерции площади свободной поверхности жидкости
    /// * load_type - Тип груза
    pub fn new(
        density: f64,
        volume: f64,
        bound_x: Bound,
        shift: Option<Position>,
        inertia: InertiaMoment,
        load_type: LoadingType,
    ) -> Self {
        assert!(density > 0., "density {} > 0", density);
        assert!(volume >= 0., "volume {} >= 0", volume);
        Self {
            density,
            volume,
            bound_x,
            shift,
            inertia,
            load_type,
        }
    }
}
///
impl ITank for Tank {
    /// Момент свободной поверхности 
    fn moment_surface(&self) -> FreeSurfaceMoment {
        let result =
            FreeSurfaceMoment::from_inertia(self.inertia.clone(), self.density);
 //       log::info!("\t Tank result:{:?}", result);    
        result
    }
    ///
    fn load_type(&self) -> LoadingType {
        self.load_type
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
        if let Some(shift) = self.shift.clone() {
            shift
        } else {
            Position::new(self.bound_x.center(), 0., 0.,)
        }
    }
}
///
impl ILoadMass for Tank{}
