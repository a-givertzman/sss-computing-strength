//! Период качки судна  (2.1.5)

use std::rc::Rc;

use crate::Error;

use super::metacentric_height::IMetacentricHeight;

/// Период качки судна  (2.1.5)
pub struct RollingPeriod {
    ///  Ширина судна B
    b: f64,
    /// Осадка судна d
    d: f64,
    /// Длина судна по ватерлинии
    l_wl: f64,
    /// Исправленная метацентрическая высота
    metacentric_height: Rc<dyn IMetacentricHeight>,
}
//
impl RollingPeriod {
    /// Основной конструктор
    pub fn new(
        // Длина судна по ватерлинии
        l_wl: f64,
        //  Ширина судна B
        b: f64,
        // Осадка судна d
        d: f64,
        // Исправленная метацентрическая высота
        metacentric_height: Rc<dyn IMetacentricHeight>,
    ) -> Self {
        Self { l_wl, b, d, metacentric_height }
    }
}
//
impl IRollingPeriod for RollingPeriod {
    /// Период качки судна
    fn calculate(&self) -> Result<f64, Error> {
        let c = self.c();
        Ok(if self.metacentric_height.h_trans_fix()? > 0. {
            let h_sqrt = self.metacentric_height.h_trans_fix()?.sqrt();
            let res= 2. *  c * self.b / h_sqrt;
    //        log::info!("\t RollingPeriod calculate l_wl:{} b:{} d:{} c:{c} h_sqrt: {h_sqrt} T:{res}", self.l_wl, self.b, self.d);
            res
        } else {
   //         log::info!("\t RollingPeriod calculate error: h_trans_fix is negative!");
            0.
        })
    }
    /// Коэффициент для расчета периода
    fn c(&self) -> f64 {
        0.373 + 0.023 * self.b / self.d - 0.043 * self.l_wl / 100.0
    }
}
#[doc(hidden)]
pub trait IRollingPeriod {
    /// Период качки судна
    fn calculate(&self) -> Result<f64, Error> ;
    /// Коэффициент для расчета периода
    fn c(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeRollingPeriod {
    value: f64,
    c: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeRollingPeriod {
    pub fn new(
        value: f64,
        c: f64,
    ) -> Self {
        Self {
            value,
            c,
        }
    }
}
#[doc(hidden)]
impl IRollingPeriod for FakeRollingPeriod {
    fn calculate(&self) -> Result<f64, Error>  {
        Ok(self.value)
    }
    fn c(&self) -> f64 {
        self.c
    }
}

