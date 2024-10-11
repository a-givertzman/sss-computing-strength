//! Амплитуда качки судна
use std::rc::Rc;
use crate::{
    math::ICurve,
    Error, IMetacentricHeight,
};
use super::rolling_period::IRollingPeriod;
/// Амплитуда качки судна с круглой скулой (2.1.5)
pub struct RollingAmplitude {
    /// Суммарная габаритная площадь скуловых килей
    a_k: Option<f64>,
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Объемное водоизмещение
    volume: f64,
    /// Длина судна по ватерлинии при текущей осадке
    l_wl: f64,
    /// Ширина судна полная
    b: f64,
    /// Ширина судна по ватерлинии ватерлинии при текущей осадке
    b_wl: f64,
    /// Осадка судна d
    d: f64,
    /// Коэффициент k для судов, имеющих скуловые кили или
    /// брусковый киль. Табл. 2.1.5.2
    k: Rc<dyn ICurve>,
    /// Безразмерный множитель Х_1 Табл. 2.1.5.1-1
    x_1: Rc<dyn ICurve>,
    /// Безразмерный множитель Х_2 Табл. 2.1.5.1-2
    x_2: Rc<dyn ICurve>,
    /// Безразмерный множитель S Табл. 2.1.5.1-3
    s: Rc<dyn ICurve>,
    /// Период качки судна
    t: Rc<dyn IRollingPeriod>,
}
//
impl RollingAmplitude {
    /// Основной конструктор
    /// * a_k - Суммарная габаритная площадь скуловых килей
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    /// * volume - Объемное водоизмещение
    /// * l_wl - Длина судна по ватерлинии при текущей осадке
    /// *  - Ширина судна полная
    /// * b_wl - Ширина судна по ватерлинии ватерлинии при текущей осадке
    /// * d - Осадка судна d
    /// * k - Коэффициент k для судов, имеющих скуловые кили или брусковый киль. Табл. 2.1.5.2
    /// * x_1 - Безразмерный множитель Х_1 Табл. 2.1.5.1-1
    /// * x_2 - Безразмерный множитель Х_2 Табл. 2.1.5.1-2
    /// * s - Безразмерный множитель S Табл. 2.1.5.1-3
    /// * t - Период качки судна
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a_k: Option<f64>,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        volume: f64,
        l_wl: f64,
        b: f64,
        b_wl: f64,
        d: f64,
        k: Rc<dyn ICurve>,
        x_1: Rc<dyn ICurve>,
        x_2: Rc<dyn ICurve>,
        s: Rc<dyn ICurve>,
        t: Rc<dyn IRollingPeriod>,
    ) -> Result<Self, Error> {
        if d <= 0. {
            return Err(Error::FromString(
                "RollingAmplitude new error: draught <= 0.".to_string(),
            ));
        }
        Ok(Self {
            a_k,
            metacentric_height,
            volume,
            l_wl,
            b,
            b_wl,
            d,
            k,
            x_1,
            x_2,
            s,
            t,
        })
    }
}
//
impl IRollingAmplitude for RollingAmplitude {
    /// Период и амплитуда качки судна с круглой скулой (2.1.5)
    fn calculate(&self) -> Result<(f64, f64), Error> {
        // Коэффициент полноты судна
        let c_b = self.volume / (self.l_wl * self.b_wl * self.d);
        let k = if let Some(a_k) = self.a_k {
            self.k.value(a_k * 100. / (self.l_wl * self.b))?
        } else {
            1.
        };
        let x_1 = self.x_1.value(self.b / self.d)?;
        let x_2 = self.x_2.value(c_b)?;
        let r = (0.73 + 0.6 * (self.metacentric_height.z_g_fix()? - self.d) / self.d).min(1.);
        let t = self.t.calculate()?;
        let s = self.s.value(t)?;
        // (2.1.5.1)
        let res = 109. * k * x_1 * x_2 * (r * s).sqrt();
        //           log::info!("\t RollingAmplitude l:{} b:{} d:{} z_g_fix:{} c_b:{} k:{k} x_1:{x_1} x_2:{x_2} r:{r} t:{t} s:{s} angle:{res}",
        //          self.l_wl, self.b, self.d, self.metacentric_height.z_g_fix()?, c_b);
        Ok((t, res.round()))
    }
}
#[doc(hidden)]
pub trait IRollingAmplitude {
    /// Период и амплитуда качки судна с круглой скулой (2.1.5)
    fn calculate(&self) -> Result<(f64, f64), Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeRollingAmplitude {
    t: f64,
    a: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeRollingAmplitude {
    pub fn new(t: f64, a: f64) -> Self {
        Self { t, a }
    }
}
#[doc(hidden)]
impl IRollingAmplitude for FakeRollingAmplitude {
    fn calculate(&self) -> Result<(f64, f64), Error> {
        Ok((self.t, self.a))
    }
}
