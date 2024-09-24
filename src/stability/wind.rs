//! Расчет плеча кренящего момента от давления ветра
use crate::{data::structs::NavigationAreaData, windage::IWindage, Error, IMass, IParameters, ParameterID};
use std::rc::Rc;

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
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
///
impl Wind {
    /// Основной конструктор
    /// * navigation_area: Параметры района плавания
    /// * a_v: f64, Площадь парусности
    /// * z_v: f64, Плечо парусности
    /// * g: f64,   Ускорение свободного падения
    /// * mass: Rc<dyn IMass>, Все грузы судна
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        navigation_area: NavigationAreaData,
        windage: Rc<dyn IWindage>,
        g: f64,
        mass: Rc<dyn IMass>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            p_v: navigation_area.p_v,
            m: navigation_area.m,
            windage,
            g,
            mass,
            parameters,
        }
    }
}
///
impl IWind for Wind {
    /// Плечо кренящего момента постоянного ветра
    fn arm_wind_static(&self) -> Result<f64, Error> {
        let res = (self.p_v * self.windage.a_v()? * self.windage.z_v()?)
            / (1000. * self.g * self.mass.sum()?);
        //    log::info!("\t Wind arm_wind_static mass_sum:{} p_v:{} a_v:{}  z_v:{} res:{res}",
        //    self.mass.sum()?, self.p_v, self.windage.a_v()?, self.windage.z_v()?,);
        self.parameters.add(ParameterID::WindPressure, self.p_v);
        self.parameters
            .add(ParameterID::WindageArea, self.windage.a_v()?);
        self.parameters
            .add(ParameterID::WindageAreaLever, self.windage.z_v()?);
        self.parameters
            .add(ParameterID::StaticWindageHeelingLever, res);
        Ok(res)
    }
    /// Плечо кренящего момента порыва ветра
    fn arm_wind_dynamic(&self) -> Result<f64, Error> {
        let res = (1. + self.m) * self.arm_wind_static()?;
        self.parameters
            .add(ParameterID::DynamicWindageHeelingLever, res);
        Ok(res)
    }
}
#[doc(hidden)]
pub trait IWind {
    /// Плечо кренящего момента постоянного ветра
    fn arm_wind_static(&self) -> Result<f64, Error>;
    /// Плечо кренящего момента порыва ветра
    fn arm_wind_dynamic(&self) -> Result<f64, Error>;
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
    pub fn new(arm_wind_static: f64, arm_wind_dynamic: f64) -> Self {
        Self {
            arm_wind_static,
            arm_wind_dynamic,
        }
    }
}
#[doc(hidden)]
impl IWind for FakeWind {
    /// Плечо кренящего момента постоянного ветра
    fn arm_wind_static(&self) -> Result<f64, Error> {
        Ok(self.arm_wind_static)
    }
    /// Плечо кренящего момента порыва ветра
    fn arm_wind_dynamic(&self) -> Result<f64, Error> {
        Ok(self.arm_wind_dynamic)
    }
}
