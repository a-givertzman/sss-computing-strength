//! Расчет критерия погоды К

use std::{f64::consts::PI, rc::Rc};

use crate::{Error, IParameters, ParameterID};

use super::{rolling_amplitude::IRollingAmplitude, lever_diagram::ILeverDiagram, wind::IWind};

/// Расчет критерия погоды К
pub struct Stability {
    /// Угол заливания отверстий
    flooding_angle: f64,
    /// Диаграмма плеч статической и динамической остойчивости
    lever_diagram: Rc<dyn ILeverDiagram>,
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    rolling_amplitude: Rc<dyn IRollingAmplitude>,
    /// Расчет плеча кренящего момента от давления ветра
    wind: Rc<dyn IWind>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>, 
}
///
impl Stability {
    /// Конструктор по умолчанию.
    /// * flooding_angle - Угол заливания отверстий
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    /// * rolling_amplitude - Амплитуда качки судна с круглой скулой (2.1.5)
    /// * wind - Расчет плеча кренящего момента от давления ветра
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        flooding_angle: f64,
        lever_diagram: Rc<dyn ILeverDiagram>,
        rolling_amplitude: Rc<dyn IRollingAmplitude>,
        wind: Rc<dyn IWind>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            flooding_angle,
            lever_diagram,
            rolling_amplitude,
            wind,
            parameters,
        }
    }
}
///
impl IStability for Stability {
    /// Расчет критерия погоды К (2.1.2)
    fn k(&self) -> Result<f64, Error> {
        let l_w1 = self.wind.arm_wind_static()?;
        let l_w2 = self.wind.arm_wind_dynamic()?;        
        let theta_w1 = *self
            .lever_diagram
            .angle(l_w1)?
            .first()
            .ok_or(Error::Calculate(
                "Stability calculate error: no angle for l_w1".to_owned(),
            ))?;
        let sunset_angle = *self
            .lever_diagram
            .angle(0.)?
            .get(1).unwrap_or(&90.);
        let theta_w2: f64 = 50.;
        let theta_f = self.flooding_angle;
        let l_w2_angles = self.lever_diagram.angle(l_w2)?;
        let l_w2_angle_first = *l_w2_angles.first().ok_or(Error::Calculate(
            "Stability calculate error: no angle for l_w2".to_owned(),
        ))?;
        let theta_c = *l_w2_angles.get(1).ok_or(Error::Calculate(
            "Stability calculate error: no second angle for l_w2".to_owned(), 
        ))?;
        // расчет а
        let (rolling_period, rolling_amplitude) = self.rolling_amplitude.calculate()?;
        let a_angle_first = theta_w1 - rolling_amplitude.round();
        let a_angle_second = l_w2_angle_first;
        let a_delta_angle = a_angle_second - a_angle_first;
        let a_s1 = self.lever_diagram.dso_area(a_angle_first, a_angle_second)?;
        let a_s2 = a_delta_angle * l_w2 * PI / 180.;
        let a = a_s2 - a_s1;        
        // расчет b
        let b_angle_first = l_w2_angle_first;
        let b_angle_second = theta_w2.min(theta_f).min(theta_c);
        let b_delta_angle = b_angle_second - b_angle_first;
        let b_s1 = self.lever_diagram.dso_area(b_angle_first, b_angle_second)?;
        let b_s2 = b_delta_angle * l_w2 * PI / 180.;
        let b = b_s1 - b_s2;  
        let k = b / a;
    //    log::info!("\t Stability k l_w1:{l_w1} l_w2:{l_w2} theta_w1:{theta_w1}  theta_w2:{theta_w2} theta_c:{theta_c} theta_f:{theta_f}
    //        a_angle1:{a_angle_first} a_angle2:{l_w2_angle_first} a_s1:{a_s1} a_s2:{a_s2} a:{a} 
    //        b_angle1:{l_w2_angle_first} b_angle2:{b_angle_second} b_s1:{b_s1} b_s2:{b_s2} b:{b} k:{k}");
        self.parameters.add(ParameterID::StaticWindageHeelingAngle, theta_w1);
        self.parameters.add(ParameterID::DynamicWindageHeelingAngle, l_w2_angle_first);
        self.parameters.add(ParameterID::HeelingAngleOfSecondPointOfIntersectionWith, theta_c);
        self.parameters.add(ParameterID::RollAmplitude, rolling_amplitude);
        self.parameters.add(ParameterID::RollPeriod, rolling_period);
        self.parameters.add(ParameterID::AreaA, a);
        self.parameters.add(ParameterID::AreaB, b);
        self.parameters.add(ParameterID::SunsetAngle, sunset_angle);
        Ok(k)
    }
}
#[doc(hidden)]
pub trait IStability {
    /// Расчет критерия погоды К (2.1.2)
    fn k(&self) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeStability {
    k: Option<f64>,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeStability {
    pub fn new(
        k: Option<f64>,
    ) -> Self {
        Self {
            k,
        }
    }
}
#[doc(hidden)]
impl IStability for FakeStability {
    /// Расчет критерия погоды К (2.1.2)
    fn k(&self) -> Result<f64, Error> {
        self.k.ok_or(Error::FromString("Some error!".to_string()))
    }
}


