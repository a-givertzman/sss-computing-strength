//! Исправленная метацентрическая высота
use std::rc::Rc;

use crate::{mass::IMass, math::*};
/// Продольная и поперечная исправленная метацентрическая высота.
pub struct MetacentricHeight {
    /// отстояние центра величины погруженной части судна       
    center_draught_shift: Position,
    /// продольный метацентрические радиус
    rad_long: f64,
    /// поперечный метацентрические радиус
    rad_cross: f64,
    /// все грузы судна
    mass: Rc<dyn IMass>,
    /// Продольная исправленная метацентрическая высота
    h_long: Option<f64>,
    /// Поперечная исправленная метацентрическая высота
    h_cross: Option<f64>,
    /// Исправленное отстояние центра масс судна по высоте
    z_g_fix: Option<f64>,
}
impl MetacentricHeight {
    /// Основной конструктор
    pub fn new(
        center_draught_shift: Position, // отстояние центра величины погруженной части судна
        rad_long: f64,                  // продольный метацентрические радиус
        rad_cross: f64,                 // поперечный метацентрические радиус
        mass: Rc<dyn IMass>,            // все грузы судна
    ) -> Self {
        Self {
            center_draught_shift,
            rad_long,
            rad_cross,
            mass,
            h_long: None,
            h_cross: None,
            z_g_fix: None,
        }
    }
    /// Вычисление значений
    #[allow(non_snake_case)]
    fn calculate(&mut self) {
        // Аппликата продольного метацентра (2)
        let Z_m = self.center_draught_shift.z() + self.rad_long;
        // Поправка к продольной метацентрической высоте на влияние
        // свободной поверхности жидкости в цистернах (2)
        let delta_m_h = self.mass.delta_m_h();
        // Продольная метацентрическая высота без учета влияния
        // поправки на влияние свободной поверхности (3)
        let H_0 = Z_m - self.mass.shift().z();
        // Продольная исправленная метацентрическая высота (3)
        self.h_long = Some(H_0 - delta_m_h.long());
        // Аппликата поперечного метацентра (8)
        let z_m = self.center_draught_shift.z() + self.rad_cross; //
                                                                  // Поперечная метацентрическая высота без учета влияния
                                                                  // поправки на влияние свободной поверхности (9)
        let h_0 = z_m - self.mass.shift().z();
        // Поперечная исправленная метацентрическая высота (9)
        self.h_cross = Some(h_0 - delta_m_h.cross());
        // Исправленное отстояние центра масс судна по высоте (10)
        self.z_g_fix = Some(self.mass.shift().z() + delta_m_h.cross());
        log::info!("\t MetacentricHeight mass:{} center_draught:{} rad_cross:{} rad_long:{} Z_m:{Z_m} H:{} z_m:{z_m} h:{} z_g_fix:{}", 
        self.mass.sum(), self.center_draught_shift, self.rad_cross, self.rad_long, self.h_long.expect("MetacentricHeight value h_long error"), self.h_cross.expect("MetacentricHeight value h_cross error"), self.z_g_fix.expect("MetacentricHeight value z_g_fix error") );
    }
    /// Продольная исправленная метацентрическая высота
    pub fn h_long(&mut self) -> f64 {
        if self.h_long.is_none() {
            self.calculate();
        }
        self.h_long.expect("MetacentricHeight h_long error")
    }
    /// Поперечная исправленная метацентрическая высота
    pub fn h_cross(&mut self) -> f64 {
        if self.h_cross.is_none() {
            self.calculate();
        }
        self.h_cross.expect("MetacentricHeight h_cross error")
    }
    /// Исправленное отстояние центра масс судна по высоте
    pub fn z_g_fix(&mut self) -> f64 {
        if self.z_g_fix.is_none() {
            self.calculate();
        }
        self.z_g_fix.expect("MetacentricHeight z_g_fix error")
    }
}
