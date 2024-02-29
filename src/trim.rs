//! Дифферент. Угол наклона корпуса судна в продольной плоскости.
use std::rc::Rc;

use crate::{
    mass::IMass,
    math::{curve::ICurve, pos_shift::IPosShift},
};
/// Дифферент судна. Вычисляется с учетом влияния свободных  
/// поверхностей жидкости.
pub struct Trim {
    /// Плотность окружающей воды
    water_density: f64,
    /// Длинна судна
    ship_length: f64,
    /// Отстояние центра величины погруженной части судна       
    center_draught_shift: Box<dyn IPosShift>,
    /// Продольный метацентрические радиус
    rad_long: Box<dyn ICurve>,
    /// Все грузы судна
    mass: Rc<dyn IMass>,
}
impl Trim {
    /// Основной конструктор
    pub fn new(
        water_density: f64,                             // Плотность окружающей воды
        ship_length: f64,                               // Длинна судна
        center_draught_shift: impl IPosShift + 'static, // Отстояние центра величины погруженной части судна
        rad_long: impl ICurve + 'static,                // Продольный метацентрические радиус
        mass: Rc<dyn IMass>,                            // Все грузы судна
    ) -> Self {
        assert!(water_density > 0., "water_density {water_density} > 0.");
        assert!(ship_length > 0., "ship_length {ship_length} > 0.");
        Self {
            water_density,
            ship_length,
            center_draught_shift: Box::new(center_draught_shift),
            rad_long: Box::new(rad_long),
            mass,
        }
    }
    /// Значение дифферента, коэффициент используемый при вычислении осадки носа и кормы
    #[allow(non_snake_case)]
    pub fn value(&self) -> f64 {
        // Суммарная масса судна и грузов
        let mass_sum = self.mass.sum();
        // Объемное водоизмещение
        let volume = mass_sum / self.water_density;
        // Отстояние центра величины погруженной части судна
        let center_draught_shift = self.center_draught_shift.value(volume);
        // Продольный метацентрические радиус
        let rad_long = self.rad_long.value(volume);
        // Аппликата продольного метацентра
        let Z_m = center_draught_shift.z() + rad_long;
        // Продольная метацентрическая высота без учета влияния
        // Поправки на влияние свободной поверхности
        let H_0 = Z_m - center_draught_shift.z();
        // Продольная исправленная метацентрическая высота
        let H = H_0 - self.mass.delta_m_h();
        // Момент дифферентующий на 1 см осадки
        let trim_moment = (mass_sum * H) / (100. * self.ship_length);
        // Дифферент судна
        let value =
            mass_sum * (self.mass.shift().x() - center_draught_shift.x()) / (100. * trim_moment);
        log::debug!("\t Trim mass:{mass_sum} volume:{volume} center:{center_draught_shift} rad:{rad_long} Z_m:{Z_m} H_0:{H_0} H:{H} M:{trim_moment} result:{value}");
        value
    }
}
