//! Расчет плеча кренящего момента от давления ветра
use std::rc::Rc;
use crate::{mass::IMass, windage::IWindage};

/// Расчет плеча кренящего момента от давления ветра
pub struct Wind {
    /// Предполагаемое давление ветра
    p_v: f64,
    /// Добавка на порывистость ветра
    m: f64,
    /// Парусность судна
    windage: Rc<dyn IWindage>,
    /// Ускорение свободного падения
    g: f64,
    /// Все грузы судна
    mass: Rc<dyn IMass>,
}
///
impl Wind {
    /// Основной конструктор
    /// * p_v: f64  Предполагаемое давление ветра
    /// * m: f64,   Добавка на порывистость ветра
    /// * a_v: f64, Площадь парусности
    /// * z_v: f64, Плечо парусности
    /// * g: f64,   Ускорение свободного падения
    /// * mass: Rc<dyn IMass>, Все грузы судна
    pub fn new(
        p_v: f64,   
        m: f64,          
        windage: Rc<dyn IWindage>,
        g: f64,             
        mass: Rc<dyn IMass>, 
    ) -> Self {
        Self {
            p_v,
            m,
            windage,
            g,
            mass,
        }
    }
}
///
impl IWind for Wind {
    /// Плечо кренящего момента постоянного ветра
    fn arm_wind_static(&self) -> f64 {
        log::info!("\t Wind arm_wind_static mass_sum:{} p_v:{} a_v:{}  z_v:{}",
        self.mass.sum(), self.p_v, self.windage.a_v(), self.windage.z_v(),);
        (self.p_v * self.windage.a_v() * self.windage.z_v()) / (1000. * self.g * self.mass.sum())
    }
    /// Плечо кренящего момента порыва ветра
    fn arm_wind_dynamic(&self) -> f64 {
        (1. + self.m) * self.arm_wind_static()
    }
}
#[doc(hidden)]
pub trait IWind {
    /// Плечо кренящего момента постоянного ветра
    fn arm_wind_static(&self) -> f64;
    /// Плечо кренящего момента порыва ветра
    fn arm_wind_dynamic(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeWind {
    arm_wind_static: f64,
    /// Плечо кренящего момента порыва ветра
    arm_wind_dynamic: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeWind {
    pub fn new(
        arm_wind_static: f64,
        arm_wind_dynamic: f64,
    ) -> Self {
        Self {
            arm_wind_static,
            arm_wind_dynamic,
        }
    }
}
#[doc(hidden)]
impl IWind for FakeWind {
    /// Плечо кренящего момента постоянного ветра
    fn arm_wind_static(&self) -> f64 {
        self.arm_wind_static
    }
    /// Плечо кренящего момента порыва ветра
    fn arm_wind_dynamic(&self) -> f64 {
        self.arm_wind_dynamic
    }
}

