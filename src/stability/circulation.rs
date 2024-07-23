//! Расчет угла крена на циркуляции

use std::rc::Rc;

use crate::{Error, ILeverDiagram, IMass, IParameters, IShipMoment, ParameterID};

/// Расчет угла крена на циркуляции
pub struct Circulation {
    /// Эксплуатационная скорость судна, m/s
    v_0: f64,
    /// Длина судна по ватерлинии
    l_wl: f64,
    /// Осадка судна d
    d: f64,
    /// Масса судна
    mass: Rc<dyn IMass>,
    /// Момент массы судна
    moment: Rc<dyn IShipMoment>,
    /// Диаграмма плеч статической и динамической остойчивости
    lever_diagram: Rc<dyn ILeverDiagram>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
///
impl Circulation {
    /// Основной конструктор
    /// * v_0 - Эксплуатационная скорость судна, m/s
    /// * l_wl - Длина судна по ватерлинии
    /// * d - Осадка судна d
    /// * mass - Масса судна
    /// * moment - Момент массы судна
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        v_0: f64,
        l_wl: f64,
        d: f64,
        mass: Rc<dyn IMass>,
        moment: Rc<dyn IShipMoment>,
        lever_diagram: Rc<dyn ILeverDiagram>,
        parameters: Rc<dyn IParameters>,
    ) -> Result<Self, Error> {
        if l_wl <= 0. {
            return Err(Error::FromString(format!(
                "Circulation new error: l_wl <= 0."
            )));
        }
        if d <= 0. {
            return Err(Error::FromString(format!("Circulation new error: d <= 0.")));
        }
        Ok(Self {
            v_0,
            l_wl,
            d,
            mass,
            moment,
            lever_diagram,
            parameters,
        })
    }
    /// Плечо кренящего момента на циркуляции при скорости v, m/s
    pub fn heel_lever(&self, v: f64) -> Result<f64, Error>  {
        // Кренящий момент на циркуляции
        let m_r = 0.2
            * (v * v * self.mass.sum()? / self.l_wl)
            * (self.moment.shift().z() - self.d / 2.).abs();
        // Плечо кренящего момента на циркуляции
        let l_r = m_r / self.mass.sum()?;
    //    log::info!("Circulation angle v:{v} m_r:{m_r} l_r:{l_r}");
        Ok(l_r)
    }
}
///
impl ICirculation for Circulation {
    /// Угла крена на циркуляции при скорости v, m/s
    fn angle(&self) -> Result<Option<f64>, Error> {
        // Угол соответствующий плечу кренящего момента
        let angle = self
            .lever_diagram
            .angle(self.heel_lever(self.v_0)?)?
            .first()
            .copied();
        log::info!("Circulation angle {:?} ", angle);
        self.parameters.add(ParameterID::VesselSpeed, self.v_0);
        Ok(angle)
    }
    /// Максимальная скорость при заданном угле крена
    fn velocity(&self, src_angle: f64) -> Result<f64, Error> {
        let mut current_vel = 10.; // m/s
        let mut delta_vel = current_vel / 2.;
        for _i in 0..20 {
            let delta_angle = src_angle
                - self
                    .lever_diagram
                    .angle(self.heel_lever(current_vel)?)?
                    .first()
                    .copied()
                    .unwrap_or(90.);
            if delta_angle.abs() < 0.001 {
                break;
            }
            //         log::info!("Circulation velocity src_angle:{src_angle} current_vel:{current_vel} delta_vel:{delta_vel} delta_angle:{delta_angle}");
            current_vel = delta_vel * delta_angle.signum();
            delta_vel /= 2.;
        }
        self.parameters.add(ParameterID::VesselSpeed, current_vel);
        Ok(current_vel)
    }
}
#[doc(hidden)]
pub trait ICirculation {
    /// Угла крена на циркуляции при эксплуатационной скорости $V_0$
    fn angle(&self) -> Result<Option<f64>, Error>;
    /// Максимальная скорость при заданном угле крена
    fn velocity(&self, angle: f64) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeAccelleration {
    angle: f64,
    velocity: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeAccelleration {
    pub fn new(angle: f64, velocity: f64) -> Self {
        Self { angle, velocity }
    }
}
#[doc(hidden)]
impl ICirculation for FakeAccelleration {
    ///
    fn angle(&self) -> Result<Option<f64>, Error> {
        Ok(Some(self.angle))
    }
    ///
    fn velocity(&self, _: f64) -> Result<f64, Error> {
        Ok(self.velocity)
    }
}
