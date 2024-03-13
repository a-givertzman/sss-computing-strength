//! Дифферент. Угол наклона корпуса судна в продольной плоскости.
use std::rc::Rc;

use crate::{
    mass::IMass,
    math::*,
};
/// Дифферент судна. Вычисляется с учетом влияния свободных  
/// поверхностей жидкости.
pub struct Trim {
    water_density: f64, // плотность окружающей воды
    /// длинна судна
    ship_length: f64,
    /// отстояние центра величины погруженной части судна       
    center_draught_shift: Box<dyn IPosShift>,
    /// продольный метацентрические радиус
    rad_long: Box<dyn ICurve>,
    /// все грузы судна
    mass: Rc<dyn IMass>,
}
impl Trim {
    /// Основной конструктор
    pub fn new(
        water_density: f64,                             // плотность окружающей воды
        ship_length: f64,                               // длинна судна
        center_draught_shift: impl IPosShift + 'static, // отстояние центра величины погруженной части судна
        rad_long: impl ICurve + 'static,                // продольный метацентрические радиус
        mass: Rc<dyn IMass>,                            // все грузы судна
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
        // суммарная масса судна и грузов
        let mass_sum = self.mass.sum();
        //объемное водоизмещение
        let volume = mass_sum / self.water_density;
        //отстояние центра величины погруженной части судна
        let center_draught_shift = self.center_draught_shift.value(volume);
        //продольный метацентрические радиус
        let rad_long = self.rad_long.value(volume);
        //аппликата продольного метацентра
        let Z_m = center_draught_shift.z() + rad_long;
        //продольная метацентрическая высота без учета влияния
        //поправки на влияние свободной поверхности
        let H_0 = Z_m - self.mass.shift().z();
        //продольная исправленная метацентрическая высота
        let H = H_0 - self.mass.delta_m_h();
        //момент дифферентующий на 1 см осадки
        let trim_moment = (mass_sum * H) / (100. * self.ship_length);
        //дифферент судна
        let value = mass_sum * (self.mass.shift().x() - center_draught_shift.x()) / (100. * trim_moment);
        log::info!("\t Trim mass:{mass_sum} volume:{volume} center:{center_draught_shift} rad:{rad_long} Z_m:{Z_m} H_0:{H_0} H:{H} M:{trim_moment} result:{value}");
        value
    }
}
