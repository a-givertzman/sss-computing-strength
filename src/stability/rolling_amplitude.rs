//! Амплитуда качки судна
use std::rc::Rc;

use crate::{math::{Curve, ICurve}, IMetacentricHeight,};

use super::rolling_period::IRollingPeriod;

/// Амплитуда качки судна с круглой скулой (2.1.5)
pub struct RollingAmplitude {
    /// Суммарная габаритная площадь скуловых килей
    a_k: Option<f64>,
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>, 
    /// Объемное водоизмещение
    volume: f64,
    /// Длина судна по ватерлинии
    l_wl: f64,
    /// Ширина судна B
    b: f64,
    /// Осадка судна d
    d: f64,
    /// Коэффициент k для судов, имеющих скуловые кили или
    /// брусковый киль. Табл. 2.1.5.2
    k: Curve,
    /// Безразмерный множитель Х_1 Табл. 2.1.5.1-1
    x_1: Curve,
    /// Безразмерный множитель Х_2 Табл. 2.1.5.1-2
    x_2: Curve,
    /// Безразмерный множитель S Табл. 2.1.5.1-3
    s: Curve,
    /// Период качки судна
    t: Box<dyn IRollingPeriod>,
}
///
impl RollingAmplitude {
    /// Основной конструктор
        /// * a_k - Суммарная габаритная площадь скуловых килей
        /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
        /// * volume - Объемное водоизмещение
        /// * l_wl - Длина судна по ватерлинии
        /// * b - Ширина судна B
        /// * d - Осадка судна d
        /// * k - Коэффициент k для судов, имеющих скуловые кили или
        /// брусковый киль. Табл. 2.1.5.2
        /// * x_1 - Безразмерный множитель Х_1 Табл. 2.1.5.1-1
        /// * x_2 - Безразмерный множитель Х_2 Табл. 2.1.5.1-2
        /// * s - Безразмерный множитель S Табл. 2.1.5.1-3
        /// * t - Период качки судна
    pub fn new(
        a_k: Option<f64>,
        metacentric_height: Rc<dyn IMetacentricHeight>, 
        volume: f64,
        l_wl: f64,
        b: f64,
        d: f64,
        k: Curve,
        x_1: Curve,
        x_2: Curve,
        s: Curve,
        t: impl IRollingPeriod + 'static,
    ) -> Self {
        assert!(d > 0., "RollingAmplitude draught {d} > 0.");
        Self {
            a_k,
            metacentric_height,
            volume,
            l_wl,
            b,
            d,
            k,
            x_1,
            x_2,
            s,
            t: Box::new(t),
        }
    }
}
///
impl IRollingAmplitude for RollingAmplitude {
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    fn calculate(&self) -> f64 {
        // Коэффициент полноты судна
        let c_b = self.volume / (self.l_wl * self.b * self.d);
        let k = self.a_k.map(|a_k| self.k.value(a_k*100./(self.l_wl*self.b))).unwrap_or(1.);
        let x_1 = self.x_1.value(self.b / self.d);
        let x_2 = self.x_2.value(c_b);
        let r = (0.73 + 0.6 * (self.metacentric_height.z_g_fix() - self.d) / self.d).min(1.);
        let t = self.t.calculate();
        let s = self.s.value(t);
        // (2.1.5.1)
        let res = 109. * k * x_1 * x_2 * (r * s).sqrt();
        log::info!("\t RollingAmplitude l:{} b:{} d:{} z_g_fix:{} c_b:{} k:{k} x_1:{x_1} x_2:{x_2} r:{r} t:{t} s:{s} angle:{res}",
        self.l_wl, self.b, self.d, self.metacentric_height.z_g_fix(), c_b);
        res
    }
}
#[doc(hidden)]
pub trait IRollingAmplitude {
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    fn calculate(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeRollingAmplitude {
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeRollingAmplitude {
    pub fn new(
        value: f64,
    ) -> Self {
        Self {
            value,
        }
    }
}
#[doc(hidden)]
impl IRollingAmplitude for FakeRollingAmplitude {
    fn calculate(&self) -> f64 {
        self.value
    }
}

