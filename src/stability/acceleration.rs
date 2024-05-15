//! Расчет критерия ускорения

use std::rc::Rc;

use crate::{ICurve, IMetacentricHeight, IRollingAmplitude, IRollingPeriod};

/// Расчет критерия ускорения
pub struct Acceleration {
    ///  Ширина судна B
    b: f64,
    /// Осадка судна d
    d: f64,
    /// Коэффициент, учитывающий особенности качки судов смешанного типа
    k_theta: Rc<dyn ICurve>,
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    rolling_amplitude: Rc<dyn IRollingAmplitude>,    
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>,   
    /// Период качки судна  (2.1.5)
    rolling_period: Rc<dyn IRollingPeriod>,   
}
/// 
impl Acceleration {
    /// Основной конструктор
    /// * b - Ширина судна B
    /// * d - Осадка судна d
    /// * k_theta - Коэффициент, учитывающий особенности качки судов смешанного типа
    /// * rolling_amplitude - Амплитуда качки судна с круглой скулой (2.1.5)
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    /// * rolling_period - Период качки судна  (2.1.5)
    pub fn new(
        b: f64,
        d: f64,
        k_theta: Rc<dyn ICurve>,
        rolling_amplitude: Rc<dyn IRollingAmplitude>,    
        metacentric_height: Rc<dyn IMetacentricHeight>,   
        rolling_period: Rc<dyn IRollingPeriod>,  
    ) -> Self {
        Self {
            b,
            d,
            k_theta,
            rolling_amplitude,    
            metacentric_height,   
            rolling_period,  
        }
    }
}
///
impl IAcceleration for Acceleration {
    /// Расчет критерия ускорения
    fn calculate(&self) -> f64 {
        let c = self.rolling_period.c();    
        let h_trans_0 = self.metacentric_height.h_trans_0();    
        let k_theta = self.k_theta.value(self.b/self.d);
        let theta_1_r = self.rolling_amplitude.calculate();
        let a = 0.0105 * h_trans_0/(c*c*self.b)*k_theta*theta_1_r;
        let k = 0.3/a; // >= 1;
        k
    }
}
#[doc(hidden)]
pub trait IAcceleration {
    /// Расчет критерия ускорения
    fn calculate(&self) -> f64;
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
    fn calculate(&self) -> f64 {
        self.value
    }
}


