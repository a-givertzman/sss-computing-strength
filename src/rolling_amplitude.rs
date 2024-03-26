//! Амплитуда качки судна
use std::rc::Rc;

use crate::{mass::IMass, math::Curve, rolling_period::RollingPeriod, ICurve};

/// Амплитуда качки судна с круглой скулой (2.1.5)
pub struct RollingAmplitude {
    /// Коэффициент полноты судна
    c_b: f64,
    /// Суммарная габаритная площадь скуловых килей
    a_k: Option<f64>,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    mass: Rc<dyn IMass>, 
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
    t: RollingPeriod,
}
///
impl RollingAmplitude {
    // Основной конструктор
    pub fn new(
        // Коэффициент полноты судна
        c_b: f64,
        // Суммарная габаритная площадь скуловых килей
        a_k: Option<f64>,
        // Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
        mass: Rc<dyn IMass>, 
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
        t: RollingPeriod,
    ) -> Self {
        assert!(d > 0., "RollingAmplitude draught {d} > 0.");
        Self {
            c_b,
            a_k,
            mass,
            l_wl,
            b,
            d,
            k,
            x_1,
            x_2,
            s,
            t,
        }
    }
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    pub fn calculate(&self) -> f64 {
        let k = self.a_k.map(|a_k| self.k.value(a_k/(self.l_wl*self.b))).unwrap_or(1.);
        let x_1 = self.x_1.value(self.b / self.d);
        let x_2 = self.x_2.value(self.c_b);
        let r = (0.73 + 0.6 * (self.mass.shift().z() - self.d) / self.d).min(1.);
        let s = self.s.value(self.t.calculate());
        // (2.1.5.1)
        let res = 109. * k * x_1 * x_2 * (r * s).sqrt();
        log::info!("\t RollingAmplitude b:{} d:{} z_g:{} c_b:{} k:{k} x_1:{x_1} x_2:{x_2} r:{r} s:{s} angle:{res}",
        self.b, self.d, self.mass.shift().z(), self.c_b);
        res
    }
}
