//! Исправленная метацентрическая высота
use std::{cell::RefCell, rc::Rc};

use crate::{mass::IMass, math::*, IParameters, ITank, LoadingType, ParameterID, Parameters};
/// Продольная и поперечная исправленная метацентрическая высота.
#[derive(Clone)]
pub struct MetacentricHeight {
    /// Отстояние центра величины погруженной части судна       
    center_draught_shift: Position,
    /// Продольный метацентрические радиус
    rad_long: f64,
    /// Поперечный метацентрические радиус
    rad_trans: f64,
    /// Все жидкие грузы судна
    tanks: Vec<Rc<dyn ITank>>,
    /// Все грузы судна
    mass: Rc<dyn IMass>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
    /// Поперечная метацентрическая высота без учета поправки   
    /// на свободные поверхности жидких грузов
    h_trans_0: Rc<RefCell<Option<f64>>>,
    /// Продольная исправленная метацентрическая высота
    h_long_fix: Rc<RefCell<Option<f64>>>,
    /// Поперечная исправленная метацентрическая высота
    h_trans_fix: Rc<RefCell<Option<f64>>>,
    /// Исправленное отстояние центра масс судна по высоте
    z_g_fix: Rc<RefCell<Option<f64>>>,
}
///
impl  MetacentricHeight {
    /// Основной конструктор
    /// * center_draught_shift - Отстояние центра величины погруженной части судна       
    /// * rad_long Продольный - метацентрические радиус
    /// * rad_trans - Поперечный метацентрические радиус
    /// * tanks - Все жидкие грузы судна
    /// * mass - Все грузы судна
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        center_draught_shift: Position, 
        rad_long: f64,                 
        rad_trans: f64,                
        tanks: Vec<Rc<dyn ITank>>,
        mass: Rc<dyn IMass>,    
        parameters: Rc<dyn IParameters>       
    ) -> Self {
        Self {
            center_draught_shift,
            rad_long,
            rad_trans,
            tanks,
            mass,
            parameters,
            h_long_fix: Rc::new(RefCell::new(None)),
            h_trans_0: Rc::new(RefCell::new(None)),
            h_trans_fix: Rc::new(RefCell::new(None)),
            z_g_fix: Rc::new(RefCell::new(None)),
        }
    }
    /// Вычисление значений
    #[allow(non_snake_case)]
    fn calculate(&self) {
        // Аппликата продольного метацентра (2)
        let Z_m = self.center_draught_shift.z() + self.rad_long;
        // Поправка к продольной метацентрической высоте на влияние
        // свободной поверхности жидкости в цистернах балласта и запасов
        let delta_m_h_ballast = DeltaMH::from_moment(self
        .tanks
        .iter()
        .filter(|v| v.load_type() == LoadingType::Ballast )
        .map(|c| c.moment_surface())
        .sum::<FreeSurfaceMoment>(), self.mass.sum());
        let delta_m_h_store = DeltaMH::from_moment(self
        .tanks
        .iter()
        .filter(|v| v.load_type() != LoadingType::Ballast )
        .map(|c| c.moment_surface())
        .sum::<FreeSurfaceMoment>(), self.mass.sum());
       // let delta_m_h = DeltaMH::from_moment(moment_surface, self.mass.sum());
        let delta_m_h = delta_m_h_ballast + delta_m_h_store;
        // Продольная метацентрическая высота без учета влияния
        // поправки на влияние свободной поверхности (3)
        let h_long_0 = Z_m - self.mass.shift().z();
        // Продольная исправленная метацентрическая высота (3)
        let h_long_fix = h_long_0 - delta_m_h.long();
        // Аппликата поперечного метацентра (8)
        let z_m = self.center_draught_shift.z() + self.rad_trans; //
        // Поперечная метацентрическая высота без учета влияния
        // поправки на влияние свободной поверхности (9)
        let h_trans_0 = z_m - self.mass.shift().z();
        // Поперечная исправленная метацентрическая высота (9)
        let h_trans_fix = h_trans_0 - delta_m_h.trans();
        // Исправленное отстояние центра масс судна по высоте (10)
        let z_g_fix: f64 = self.mass.shift().z() + delta_m_h.trans();
        log::info!("\t MetacentricHeight mass:{} mass_z:{}center_draught:{} rad_trans:{} rad_long:{} delta_m_h_ballast:{} delta_m_h_store:{} Z_m:{Z_m} H_0:{h_long_0} H:{h_long_fix} z_m:{z_m} h_0:{h_trans_0} h:{h_trans_fix} z_g_fix:{z_g_fix}", 
            self.mass.sum(), self.mass.shift().z(), self.center_draught_shift, self.rad_trans, self.rad_long, delta_m_h_ballast.trans, delta_m_h_store.trans() );
        *self.h_long_fix.borrow_mut() = Some(h_long_fix);
        *self.h_trans_0.borrow_mut() = Some(h_trans_0);
        *self.h_trans_fix.borrow_mut() = Some(h_trans_fix);
        *self.z_g_fix.borrow_mut() = Some(z_g_fix);
        self.parameters.add(ParameterID::CenterMassZFix, z_g_fix);
        self.parameters.add(ParameterID::MetacentricLongRadZ, Z_m);
        self.parameters.add(ParameterID::MetacentricTransRadZ, z_m);
        self.parameters.add(ParameterID::MetacentricTransBallast, delta_m_h_ballast.trans());
        self.parameters.add(ParameterID::MetacentricLongBallast, delta_m_h_ballast.long());
        self.parameters.add(ParameterID::MetacentricTransStore, delta_m_h_store.trans());
        self.parameters.add(ParameterID::MetacentricLongStore, delta_m_h_store.long());
        self.parameters.add(ParameterID::MetacentricTransRad, self.rad_trans);
        self.parameters.add(ParameterID::MetacentricLongRad, self.rad_long);
        self.parameters.add(ParameterID::MetacentricTransHeight, h_trans_0);
        self.parameters.add(ParameterID::MetacentricTransHeightFix, h_trans_fix);
        self.parameters.add(ParameterID::MetacentricLongHeight, h_long_0);
        self.parameters.add(ParameterID::MetacentricLongHeightFix, h_long_fix);
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
            .expect("MetacentricHeight h_long_fix error")
    }
    /// Поперечная метацентрическая высота без учета поправки   
    /// на свободные поверхности жидких грузов
    fn h_trans_0(&self) -> f64 {
        if self.h_trans_0.borrow().is_none() {
            self.calculate();
        }
        self.h_trans_0
            .borrow()
            .expect("MetacentricHeight h_trans_0 error")
    }
    /// Поперечная исправленная метацентрическая высота
    fn h_trans_fix(&self) -> f64 {
        if self.h_trans_fix.borrow().is_none() {
            self.calculate();
        }
        self.h_trans_fix
            .borrow()
            .expect("MetacentricHeight h_trans_fix error")
    }
    /// Исправленное отстояние центра масс судна по высоте
    fn z_g_fix(&self) -> f64 {
        if self.z_g_fix.borrow().is_none() {
            self.calculate();
        }
        self.z_g_fix
            .borrow()
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
    fn h_trans_0(&self) -> f64;
    /// Поперечная исправленная метацентрическая высота
    fn h_trans_fix(&self) -> f64;
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
    h_trans_0: f64,
    /// Поперечная исправленная метацентрическая высота h
    h_trans_fix: f64,
    /// Исправленное отстояние центра масс судна по высоте
    z_g_fix: f64,
}
///
#[doc(hidden)]
#[allow(dead_code)]
impl FakeMetacentricHeight {
    /// Основной конструктор
    pub fn new(h_long_fix: f64, h_trans_0: f64, h_trans_fix: f64, z_g_fix: f64) -> Self {
        Self {
            h_long_fix,
            h_trans_0,
            h_trans_fix,
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
    fn h_trans_0(&self) -> f64 {
        self.h_trans_0
    }
    /// Поперечная исправленная метацентрическая высота
    fn h_trans_fix(&self) -> f64 {
        self.h_trans_fix
    }
    /// Исправленное отстояние центра масс судна по высоте
    fn z_g_fix(&self) -> f64 {
        self.z_g_fix
    }
}
