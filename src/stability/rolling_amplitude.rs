//! Амплитуда качки судна
use std::rc::Rc;

use crate::{math::{Curve, ICurve}, mass::IMass};

use super::rolling_period::IRollingPeriod;

/// Амплитуда качки судна с круглой скулой (2.1.5)
pub struct RollingAmplitude {
    /// Суммарная габаритная площадь скуловых килей
    a_k: Option<f64>,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    mass: Rc<dyn IMass>, 
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
    // Основной конструктор
    pub fn new(
        // Суммарная габаритная площадь скуловых килей
        a_k: Option<f64>,
        // Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
        mass: Rc<dyn IMass>, 
        // Объемное водоизмещение
        volume: f64,
        // Длина судна по ватерлинии
        l_wl: f64,
        // Ширина судна B
        b: f64,
        // Осадка судна d
        d: f64,
        // Коэффициент k для судов, имеющих скуловые кили или
        // брусковый киль. Табл. 2.1.5.2
        k: Curve,
        // Безразмерный множитель Х_1 Табл. 2.1.5.1-1
        x_1: Curve,
        // Безразмерный множитель Х_2 Табл. 2.1.5.1-2
        x_2: Curve,
        // Безразмерный множитель S Табл. 2.1.5.1-3
        s: Curve,
        // Период качки судна
        t: impl IRollingPeriod + 'static,
    ) -> Self {
        assert!(d > 0., "RollingAmplitude draught {d} > 0.");
        Self {
            a_k,
            mass,
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
        let k = self.a_k.map(|a_k| self.k.value(a_k/(self.l_wl*self.b))).unwrap_or(1.);
        let x_1 = self.x_1.value(self.b / self.d);
        let x_2 = self.x_2.value(c_b);
        let r = (0.73 + 0.6 * (self.mass.shift().z() - self.d) / self.d).min(1.);
        let s = self.s.value(self.t.calculate());
        // (2.1.5.1)
        let res = 109. * k * x_1 * x_2 * (r * s).sqrt();
        log::info!("\t RollingAmplitude b:{} d:{} z_g:{} c_b:{} k:{k} x_1:{x_1} x_2:{x_2} r:{r} s:{s} angle:{res}",
        self.b, self.d, self.mass.shift().z(), c_b);
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

