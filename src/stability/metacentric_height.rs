//! Исправленная метацентрическая высота
use std::{cell::RefCell, rc::Rc};

use crate::{math::*, mass::IMass};
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
    /// Поперечная метацентрическая высота без учета поправки   
    /// на свободные поверхности жидких грузов
    h_cross_0: Rc<RefCell<Option<f64>>>,
    /// Продольная исправленная метацентрическая высота
    h_long_fix: Rc<RefCell<Option<f64>>>,
    /// Поперечная исправленная метацентрическая высота
    h_cross_fix: Rc<RefCell<Option<f64>>>,
    /// Исправленное отстояние центра масс судна по высоте
    z_g_fix: Rc<RefCell<Option<f64>>>,
}
///
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
            h_long_fix: Rc::new(RefCell::new(None)),
            h_cross_0: Rc::new(RefCell::new(None)),
            h_cross_fix: Rc::new(RefCell::new(None)),
            z_g_fix: Rc::new(RefCell::new(None)),
        }
    }
    /// Вычисление значений
    #[allow(non_snake_case)]
    fn calculate(&self) {
        // Аппликата продольного метацентра (2)
        let Z_m = self.center_draught_shift.z() + self.rad_long;
        // Поправка к продольной метацентрической высоте на влияние
        // свободной поверхности жидкости в цистернах (2)
        let delta_m_h = self.mass.delta_m_h();
        // Продольная метацентрическая высота без учета влияния
        // поправки на влияние свободной поверхности (3)
        let h_long_0 = Z_m - self.mass.shift().z();
        // Продольная исправленная метацентрическая высота (3)
        let h_long_fix = h_long_0 - delta_m_h.long();
        // Аппликата поперечного метацентра (8)
        let z_m = self.center_draught_shift.z() + self.rad_cross; //
                                                                  // Поперечная метацентрическая высота без учета влияния
                                                                  // поправки на влияние свободной поверхности (9)
        let h_cross_0 = z_m - self.mass.shift().z();
        // Поперечная исправленная метацентрическая высота (9)
        let h_cross_fix = h_cross_0 - delta_m_h.cross();
        // Исправленное отстояние центра масс судна по высоте (10)
        let z_g_fix: f64 = self.mass.shift().z() + delta_m_h.cross();
        log::info!("\t MetacentricHeight mass:{} center_draught:{} rad_cross:{} rad_long:{} Z_m:{Z_m} H_0:{h_long_0} H:{h_long_fix} z_m:{z_m} h_0:{h_cross_0} h:{h_cross_fix} z_g_fix:{z_g_fix}", 
        self.mass.sum(), self.center_draught_shift, self.rad_cross, self.rad_long );
        *self.h_long_fix.borrow_mut() = Some(h_long_fix);
        *self.h_cross_0.borrow_mut() = Some(h_cross_0);
        *self.h_cross_fix.borrow_mut() = Some(h_cross_fix);
        *self.z_g_fix.borrow_mut() = Some(z_g_fix);
    }
}
///
#[allow(dead_code)]
impl IMetacentricHeight for MetacentricHeight {
    /// Продольная исправленная метацентрическая высота
    fn h_long_fix(&self) -> f64 {
        if self.h_long_fix.borrow().is_none() {
            self.calculate();
        }
        self.h_long_fix
            .borrow()
            .clone()
            .expect("MetacentricHeight h_long_fix error")
    }
    /// Поперечная метацентрическая высота без учета поправки   
    /// на свободные поверхности жидких грузов
    fn h_cross_0(&self) -> f64 {
        if self.h_cross_0.borrow().is_none() {
            self.calculate();
        }
        self.h_cross_0
            .borrow()
            .clone()
            .expect("MetacentricHeight h_cross_0 error")
    }
    /// Поперечная исправленная метацентрическая высота
    fn h_cross_fix(&self) -> f64 {
        if self.h_cross_fix.borrow().is_none() {
            self.calculate();
        }
        self.h_cross_fix
            .borrow()
            .clone()
            .expect("MetacentricHeight h_cross_fix error")
    }
    /// Исправленное отстояние центра масс судна по высоте
    fn z_g_fix(&self) -> f64 {
        if self.z_g_fix.borrow().is_none() {
            self.calculate();
        }
        self.z_g_fix
            .borrow()
            .clone()
            .expect("MetacentricHeight z_g_fix error")
    }
}
///
#[doc(hidden)]
pub trait IMetacentricHeight {
    /// Продольная исправленная метацентрическая высота
    fn h_long_fix(&self) -> f64;
    /// Поперечная метацентрическая высота без учета поправки   
    /// на свободные поверхности жидких грузов
    fn h_cross_0(&self) -> f64;
    /// Поперечная исправленная метацентрическая высота
    fn h_cross_fix(&self) -> f64;
    /// Исправленное отстояние центра масс судна по высоте
    fn z_g_fix(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeMetacentricHeight {
    /// Продольная исправленная метацентрическая высота H
    h_long_fix: f64,
    /// Поперечная метацентрическая высота без учета поправки   
    /// на свободные поверхности жидких грузов
    h_cross_0: f64,
    /// Поперечная исправленная метацентрическая высота h
    h_cross_fix: f64,
    /// Исправленное отстояние центра масс судна по высоте
    z_g_fix: f64,
}
///
#[doc(hidden)]
#[allow(dead_code)]
impl FakeMetacentricHeight {
    /// Основной конструктор
    pub fn new(h_long_fix: f64, h_cross_0: f64, h_cross_fix: f64, z_g_fix: f64) -> Self {
        Self {
            h_long_fix,
            h_cross_0,
            h_cross_fix,
            z_g_fix,
        }
    }
}
///
#[doc(hidden)]
#[allow(dead_code)]
impl IMetacentricHeight for FakeMetacentricHeight {
    /// Продольная исправленная метацентрическая высота
    fn h_long_fix(&self) -> f64 {
        self.h_long_fix
    }
    /// Поперечная метацентрическая высота без учета поправки   
    /// на свободные поверхности жидких грузов
    fn h_cross_0(&self) -> f64 {
        self.h_cross_0
    }
    /// Поперечная исправленная метацентрическая высота
    fn h_cross_fix(&self) -> f64 {
        self.h_cross_fix
    }
    /// Исправленное отстояние центра масс судна по высоте
    fn z_g_fix(&self) -> f64 {
        self.z_g_fix
    }
}
