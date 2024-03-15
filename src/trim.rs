//! Дифферент. Угол наклона корпуса судна в продольной плоскости.
use std::rc::Rc;

use crate::{
    mass::IMass,
    math::*,
};
/// Дифферент судна. Вычисляется с учетом влияния свободных  
/// поверхностей жидкости.
pub struct Trim {
    /// длинна судна
    ship_length: f64,
    /// отстояние центра величины погруженной части судна       
    center_draught_shift: Position,
    /// продольный метацентрические радиус
    rad_long: f64,
    /// поперечный метацентрические радиус
    rad_cross: f64,
    /// все грузы судна
    mass: Rc<dyn IMass>,
}
impl Trim {
    /// Основной конструктор
    pub fn new(
        ship_length: f64,                               // длинна судна
        center_draught_shift: Position, // отстояние центра величины погруженной части судна
        rad_long: f64,                // продольный метацентрические радиус
        rad_cross: f64,                 // поперечный метацентрические радиус
        mass: Rc<dyn IMass>,                            // все грузы судна
    ) -> Self {
        assert!(ship_length > 0., "ship_length {ship_length} > 0.");
        Self {
            ship_length,
            center_draught_shift,
            rad_long,
            rad_cross,
            mass,
        }
    }
    /// Значение дифферента, коэффициент используемый при вычислении осадки носа и кормы
    #[allow(non_snake_case)]
    pub fn value(&self) -> f64 {
        // Аппликата продольного метацентра (2)
        let Z_m = self.center_draught_shift.z() + self.rad_long;
        // Поправка к продольной метацентрической высоте на влияние   
        // свободной поверхности жидкости в цистернах (2) 
        let delta_m_h = self.mass.delta_m_h(); 
        // Продольная метацентрическая высота без учета влияния
        // поправки на влияние свободной поверхности (3)
        let H_0 = Z_m - self.mass.shift().z(); 
        // Продольная исправленная метацентрическая высота (3)
        let H = H_0 - delta_m_h.long(); 
        // Момент дифферентующий на 1 см осадки (4)
        let trim_moment = (self.mass.sum() * H) / (100. * self.ship_length);
        // Дифферент судна (5)
        let value = self.mass.sum() * (self.mass.shift().x() - self.center_draught_shift.x()) / (100. * trim_moment);
        log::info!("\t Trim mass:{} center_draught:{} rad_cross:{} rad_long:{} Z_m:{Z_m} H_0:{H_0} H:{H} M:{trim_moment} result:{value}", 
        self.mass.sum(), self.center_draught_shift, self.rad_cross, self.rad_long );

        // Ааппликата поперечного метацентра (8)
        let z_m = self.center_draught_shift.z() + self.rad_cross;//        
        // Поперечная метацентрическая высота без учета влияния
        // поправки на влияние свободной поверхности (9)
        let h_0 = z_m - self.mass.shift().z();         
        // Поперечная исправленная метацентрическая высота (9)
        let h = h_0 - delta_m_h.cross();   
        // Исправленное отстояние центра масс судна по высоте (10) 
        let z_g_fix = self.mass.shift().z() + delta_m_h.cross(); 

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
