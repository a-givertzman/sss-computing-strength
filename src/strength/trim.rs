//! Дифферент. Угол наклона корпуса судна в продольной плоскости.
use crate::stability::metacentric_height::IMetacentricHeight;
use std::rc::Rc;

use crate::math::*;

use super::mass::IMass;

/// Дифферент судна. Вычисляется с учетом влияния свободных  
/// поверхностей жидкости.
pub struct Trim {
    /// длинна судна
    ship_length: f64,
    /// отстояние центра величины погруженной части судна       
    center_draught_shift: Position,
    /// Исправленная метацентрическая высота
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// все грузы судна
    mass: Rc<dyn IMass>,
}
///
impl Trim {
    /// Основной конструктор
    pub fn new(
        ship_length: f64,                      // длинна судна
        center_draught_shift: Position,        // отстояние центра величины погруженной части судна
        metacentric_height: Rc<dyn IMetacentricHeight>, // Исправленная метацентрическая высота
        mass: Rc<dyn IMass>,                   // все грузы судна
    ) -> Self {
        assert!(ship_length > 0., "ship_length {ship_length} > 0.");
        Self {
            ship_length,
            center_draught_shift,
            metacentric_height,
            mass,
        }
    }
    /// Значение дифферента, коэффициент используемый при вычислении осадки носа и кормы
    #[allow(non_snake_case)]
    pub fn value(&mut self) -> f64 {
        // Продольная исправленная метацентрическая высота (3)
        let H = self.metacentric_height.h_long();
        // Момент дифферентующий на 1 см осадки (4)
        let trim_moment = (self.mass.sum() * H) / (100. * self.ship_length);
        // Дифферент судна (5)
        let value = self.mass.sum() * (self.mass.shift().x() - self.center_draught_shift.x())
            / (100. * trim_moment);
        log::info!(
            "\t Trim H:{H} mass:{} center_draught:{} M:{trim_moment} result:{value}",
            self.mass.sum(),
            self.center_draught_shift
        );
        value
    }
}
