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
    /// поперечный метацентрические радиус
    rad_lat: Box<dyn ICurve>,
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
        rad_lat: impl ICurve + 'static,                 // поперечный метацентрические радиус
        mass: Rc<dyn IMass>,                            // все грузы судна
    ) -> Self {
        assert!(water_density > 0., "water_density {water_density} > 0.");
        assert!(ship_length > 0., "ship_length {ship_length} > 0.");
        Self {
            water_density,
            ship_length,
            center_draught_shift: Box::new(center_draught_shift),
            rad_long: Box::new(rad_long),
            rad_lat: Box::new(rad_lat),
            mass,
        }
    }
    /// Значение дифферента, коэффициент используемый при вычислении осадки носа и кормы
    #[allow(non_snake_case)]
    pub fn value(&self) -> f64 {
        // Суммарная масса судна и грузов
        let mass_sum = self.mass.sum(); 
        // Объемное водоизмещение (1)
        let volume = mass_sum / self.water_density;
        // Отстояние центра величины погруженной части судна
        let center_draught_shift = self.center_draught_shift.value(volume); 
        // Отстояние центра масс
        let mass_shift = self.mass.shift();
        // Продольный метацентрические радиус
        let rad_long = self.rad_long.value(volume);
        // Аппликата продольного метацентра (2)
        let Z_m = center_draught_shift.z() + rad_long;
        // Поправка к продольной метацентрической высоте на влияние   
        // свободной поверхности жидкости в цистернах (2) 
        let delta_m_h = self.mass.delta_m_h(); 
        // Продольная метацентрическая высота без учета влияния
        // поправки на влияние свободной поверхности (3)
        let H_0 = Z_m - mass_shift.z(); 
        // Продольная исправленная метацентрическая высота (3)
        let H = H_0 - delta_m_h.long(); 
        // Момент дифферентующий на 1 см осадки (4)
        let trim_moment = (mass_sum * H) / (100. * self.ship_length);
        // Дифферент судна (5)
        let value = mass_sum * (mass_shift.x() - center_draught_shift.x()) / (100. * trim_moment);
        log::info!("\t Trim mass:{mass_sum} volume:{volume} center_draught:{center_draught_shift} rad:{rad_long} Z_m:{Z_m} H_0:{H_0} H:{H} M:{trim_moment} result:{value}");
        // Поперечный метацентрические радиус
        let rad_lat = self.rad_lat.value(volume);
        // Ааппликата поперечного метацентра (8)
        let z_m = center_draught_shift.z() + rad_lat;//        
        // Поперечная метацентрическая высота без учета влияния
        // поправки на влияние свободной поверхности (9)
        let h_0 = z_m - mass_shift.z();         
        // Поперечная исправленная метацентрическая высота (9)
        let h = h_0 - delta_m_h.lat();   
        // Исправленное отстояние центра масс судна по высоте (10) 
        let z_g_fix = mass_shift.z() + delta_m_h.lat(); 

        value

        // Trim mass:2354.3786000000005  
        // Mass shift:Position(-3.5246486210841343, -0.02762147090531659, 5.07403924500503) 
        //  volume:2296.954731707318 
        // center_draught:Position(-0.2798811092549283, 0, 0.8431351820915471) 
        // rad:621.1053725359169 
        // Z_m:621.9485077180085 
        // H_0:616.8744684730035 
        // H:616.7540545344806 
        // M:124.53452379581597 
        // result:-0.6134372187708639  //-0.8853018196   
        // -0.8853018196  = 2354.3786 * (-3.524648621 - -0.2798811) / (100. * trim_moment);
        // 8629,160185019 = (100. * trim_moment);
        // 86,29160185 = trim_moment 
        //86.29160185 = (2354.3786 * H) / (100. * 116.6);
        //427,356959387 = H;
    }
}
