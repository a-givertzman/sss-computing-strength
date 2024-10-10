//! Расчет критерия ускорения

use std::rc::Rc;

use crate::{Error, ICurve, IMetacentricHeight, IRollingAmplitude, IRollingPeriod};

/// Расчет критерия ускорения
pub struct Acceleration {
    ///  Ширина судна B
    b: f64,
    /// Осадка судна, м
    d: f64,
    /// Коэффициент, учитывающий особенности качки судов смешанного типа
    k_theta: Rc<dyn ICurve>,
    /// Период качки судна
    rolling_period: Rc<dyn IRollingPeriod>,  
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    rolling_amplitude: Rc<dyn IRollingAmplitude>,    
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>,   
}
/// 
impl Acceleration {
    /// Основной конструктор
    /// * b - Ширина судна, м
    /// * d - Осадка судна, м
    /// * k_theta - Коэффициент, учитывающий особенности качки судов смешанного типа
    /// * rolling_period - Период качки судна
    /// * rolling_amplitude - Амплитуда качки судна с круглой скулой (2.1.5)
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    pub fn new(
        b: f64,
        d: f64,
        k_theta: Rc<dyn ICurve>,
        rolling_period: Rc<dyn IRollingPeriod>,  
        rolling_amplitude: Rc<dyn IRollingAmplitude>,    
        metacentric_height: Rc<dyn IMetacentricHeight>,   
    ) -> Self {
        Self {
            b,
            d,
            k_theta,
            rolling_period,
            rolling_amplitude,    
            metacentric_height,   
        }
    }
}
///
impl IAcceleration for Acceleration {
    /// Расчет критерия ускорения
    fn calculate(&self) -> Result<f64, Error> {
        let h_trans_0 = self.metacentric_height.h_trans_0()?;    
        let k_theta = self.k_theta.value(self.b/self.d)?;
        let c = self.rolling_period.c();
        let (_, theta_1_r) = self.rolling_amplitude.calculate()?;
        let a = 0.0105 * h_trans_0/(c*c*self.b)*k_theta*theta_1_r;
        let k = 0.3/a; // >= 1;
        log::trace!("Acceleration calculate: zg_fix:{} b:{} d:{} h_trans_0:{h_trans_0} k_theta:{k_theta} c:{c} theta_1_r:{theta_1_r} a:{a} k:{k}",  self.metacentric_height.z_g_fix().unwrap(), self.b, self.d);
        Ok(k)
    }
}
#[doc(hidden)]
pub trait IAcceleration {
    /// Расчет критерия ускорения
    fn calculate(&self) -> Result<f64, Error> ;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeAccelleration {
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeAccelleration {
    pub fn new(
        value: f64,
    ) -> Self {
        Self {
            value,
        }
    }
}
#[doc(hidden)]
impl IAcceleration for FakeAccelleration {
    ///
    fn calculate(&self) -> Result<f64, Error>  {
        Ok(self.value)
    }
}


