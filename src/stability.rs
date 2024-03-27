//! Расчет характеристик остойчивости судна
use crate::{
    error::Error, math::Curve, rolling_amplitude::IRollingAmplitude, stability_arm::IStabilityArm, wind::IWind, ICurve
};
/// Расчет характеристик остойчивости судна
pub struct Stability {
    /// Угол заливания отверстий
    flooding_angle: f64,
    /// Диаграмма плеч статической остойчивости
    stability_arm: Box<dyn IStabilityArm>,
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    rolling_amplitude: Box<dyn IRollingAmplitude>,
    /// Расчет плеча кренящего момента от давления ветра
    wind: Box<dyn IWind>,
}

impl Stability {
    pub fn new(
        // Угол заливания отверстий
        flooding_angle: f64,
        // Диаграмма плеч статической остойчивости
        stability_arm: Box<dyn IStabilityArm>,
        // Амплитуда качки судна с круглой скулой (2.1.5)
        rolling_amplitude: Box<dyn IRollingAmplitude>,
        // Расчет плеча кренящего момента от давления ветра
        wind: Box<dyn IWind>,
    ) -> Self {
        Self {
            flooding_angle,
            stability_arm,
            rolling_amplitude,
            wind,
        }
    }
    /// Расчет критерия погоды К (2.1.2)
    pub fn k(&mut self) -> Result<f64, Error> {
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
        let curve = Curve::new_catmull_rom(&self.stability_arm.diagram());
        // расчет а
        let a_angle_first = theta_w1 - self.rolling_amplitude.calculate();
        let a_angle_second = l_w2_angle_first;
        let a_delta_angle = a_angle_second - a_angle_first;
        let a = -curve.integral(a_angle_first, a_angle_second) + a_delta_angle*l_w2;        
        // расчет b
        let b_angle_first = l_w2_angle_first;
        let b_angle_second = theta_w2.min(theta_f).min(theta_c);
        let b_delta_angle = b_angle_second - b_angle_first;
        let b = curve.integral(b_angle_first, b_angle_second) - b_delta_angle*l_w2;
        let res = b / a;
        log::info!("\t Stability k l_w1:{l_w1} l_w2:{l_w2} theta_w1:{theta_w1} l_w2_angle1:{l_w2_angle_first}
            theta_c:{theta_c} a:{a} b:{b} k:{res}");
        Ok(res)
    }
}