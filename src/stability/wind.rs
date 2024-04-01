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
    windage: Box<dyn IWindage>,
    /// Ускорение свободного падения
    g: f64,
    /// Все грузы судна
    mass: Rc<dyn IMass>,
    /// Плечо кренящего момента постоянного ветра
    l_w_1: Option<f64>,
    /// Плечо кренящего момента порыва ветра
    l_w_2: Option<f64>,
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
        windage: Box<dyn IWindage>,
        g: f64,             
        mass: Rc<dyn IMass>, 
    ) -> Self {
        Self {
            p_v,
            m,
            windage,
            g,
            mass,
            l_w_1: None,
            l_w_2: None,
        }
    }
    /// Расчет плечей моментов от ветра
    fn calculate(&mut self) {
        // (2.1.4.1-1)
        let l_w_1 = (self.p_v * self.windage.a_v() * self.windage.z_v()) / (1000. * self.g * self.mass.sum());
        // (2.1.4.1-2)
        let l_w_2 = (1. + self.m) * l_w_1;
        log::info!("Wind l_w_1:{l_w_1} l_w_2:{l_w_2}");
        self.l_w_1 = Some(l_w_1);
        self.l_w_2 = Some(l_w_2);
    }
}
///
impl IWind for Wind {
    /// Плечо кренящего момента постоянного ветра
    fn arm_wind_static(&mut self) -> f64 {
        if self.l_w_1.is_none() {
            self.calculate();
        }
        self.l_w_1.expect("Wind arm_wind_static error: no l_w_2!")
    }
    /// Плечо кренящего момента порыва ветра
    fn arm_wind_dynamic(&mut self) -> f64 {
        if self.l_w_2.is_none() {
            self.calculate();
        }
        self.l_w_2.expect("Wind arm_wind_dynamic error: no l_w_2!")
    }
}
#[doc(hidden)]
pub trait IWind {
    /// Плечо кренящего момента постоянного ветра
    fn arm_wind_static(&mut self) -> f64;
    /// Плечо кренящего момента порыва ветра
    fn arm_wind_dynamic(&mut self) -> f64;
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
    fn arm_wind_static(&mut self) -> f64 {
        self.arm_wind_static
    }
    /// Плечо кренящего момента порыва ветра
    fn arm_wind_dynamic(&mut self) -> f64 {
        self.arm_wind_dynamic
    }
}

