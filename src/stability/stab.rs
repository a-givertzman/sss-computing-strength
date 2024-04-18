//! Расчет характеристик остойчивости судна

use std::f64::consts::PI;

use crate::{math::{Curve, ICurve}, Error};

use super::{rolling_amplitude::IRollingAmplitude, stability_arm::IStabilityArm, wind::IWind};

/// Расчет характеристик остойчивости судна
pub struct Stability {
    /// Угол заливания отверстий
    flooding_angle: f64,
    /// Диаграмма плеч статической и динамической остойчивости
    stability_arm: Box<dyn IStabilityArm>,
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    rolling_amplitude: Box<dyn IRollingAmplitude>,
    /// Расчет плеча кренящего момента от давления ветра
    wind: Box<dyn IWind>,
    /// Кривая диаграммы плеч статической остойчивости
    dso_curve: Option<Curve>,
    /// Критерий погоды
    k: Option<f64>,
}

impl Stability {
    /// Конструктор по умолчанию.
    /// * flooding_angle - Угол заливания отверстий
    /// * stability_arm - Диаграмма плеч статической и динамической остойчивости
    /// * rolling_amplitude - Амплитуда качки судна с круглой скулой (2.1.5)
    /// * wind - Расчет плеча кренящего момента от давления ветра
    pub fn new(
        flooding_angle: f64,
        stability_arm: Box<dyn IStabilityArm>,
        rolling_amplitude: Box<dyn IRollingAmplitude>,
        wind: Box<dyn IWind>,
    ) -> Self {
        Self {
            flooding_angle,
            stability_arm,
            rolling_amplitude,
            wind,
            dso_curve: None, 
            k: None,
        }
    }
    /// Расчет критерия погоды К (2.1.2)
    pub fn k(&mut self) -> Result<f64, Error> {
        if self.k.is_none() {
            self.calculate()?;
        }

        Ok(self.k.expect("Stability k error: no k!"))
    }
    /// Площадь под положительной частью диаграммы статической остойчивости
    pub fn dso_area(&mut self, angle1: f64, angle2: f64) -> Result<f64, Error> {
        assert!(angle1 < angle2, "Stability dso_area angle1 {angle1} < angle2 {angle2}");
        if self.k.is_none() {
            self.calculate()?;
        }

        Ok(self.dso_curve.as_ref().expect("Stability k error: no k!").integral(angle1, angle2))
    }
    /// Угол, соответствующий максимуму диаграммы статической остойчивости
    pub fn theta_max(&mut self) -> Result<f64, Error>  {
        if self.k.is_none() {
            self.calculate()?;
        }
        Ok(self.stability_arm.theta_max())
    }
    fn calculate(&mut self) -> Result<(), Error>  {
        let l_w1 = self.wind.arm_wind_static();
        let l_w2 = self.wind.arm_wind_dynamic();
        let theta_w1 = *self
            .stability_arm
            .angle(l_w1)
            .first()
            .ok_or(Error::Calculate(
                "Stability calculate error: no angle for l_w1".to_owned(),
            ))?;
        let theta_w2: f64 = 50.;
        let theta_f = self.flooding_angle;
        let l_w2_angles = self.stability_arm.angle(l_w2);
        let l_w2_angle_first = *l_w2_angles.first().ok_or(Error::Calculate(
            "Stability calculate error: no angle for l_w2".to_owned(),
        ))?;
        let theta_c = *l_w2_angles.get(1).ok_or(Error::Calculate(
            "Stability calculate error: no second angle for l_w2".to_owned(), 
        ))?;
        let dso_curve = Curve::new_catmull_rom(&self.stability_arm.dso());
        // расчет а
        let a_angle_first = theta_w1 - self.rolling_amplitude.calculate().round();
        let a_angle_second = l_w2_angle_first;
        let a_delta_angle = a_angle_second - a_angle_first;
        let a_s1 = dso_curve.integral(a_angle_first, a_angle_second);
        let a_s2 = a_delta_angle*l_w2;
        let a = (a_s2 - a_s1)*PI/180.;        
        // расчет b
        let b_angle_first = l_w2_angle_first;
        let b_angle_second = theta_w2.min(theta_f).min(theta_c);
        let b_delta_angle = b_angle_second - b_angle_first;
        let b_s1 = dso_curve.integral(b_angle_first, b_angle_second);
        let b_s2 = b_delta_angle*l_w2;
        let b = (b_s1 - b_s2)*PI/180.;  
        let k = b / a;
        log::info!("\t Stability k l_w1:{l_w1} l_w2:{l_w2} theta_w1:{theta_w1}  theta_w2:{theta_w2} theta_c:{theta_c} theta_f:{theta_f}
            a_angle1:{a_angle_first} a_angle2:{l_w2_angle_first} a_s1:{a_s1} a_s2:{a_s2} a:{a} 
            b_angle1:{l_w2_angle_first} b_angle2:{b_angle_second} b_s1:{b_s1} b_s2:{b_s2} b:{b} 
            k:{k}");

        self.dso_curve = Some(dso_curve);
        self.k = Some(k);
        Ok(())
    }
}
