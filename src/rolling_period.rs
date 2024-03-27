//! Период качки судна  (2.1.5)

use std::rc::Rc;

use crate::metacentric_height::IMetacentricHeight;

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
///
impl RollingPeriod {
    /// Основной конструктор
    pub fn new(
        //  Ширина судна B
        b: f64,
        // Осадка судна d
        d: f64,
        // Длина судна по ватерлинии
        l_wl: f64,
        // Исправленная метацентрическая высота
        metacentric_height: Rc<dyn IMetacentricHeight>,
    ) -> Self {
        Self { b, d, l_wl, metacentric_height }
    }
    /// Коэффициент для расчета периода
    fn c(&self) -> f64 {
        0.373 + 0.023 * self.b / self.d - 0.043 * self.l_wl / 100.0
    }
}
///
impl IRollingPeriod for RollingPeriod {
    /// Период качки судна
    fn calculate(&self) -> f64 {
        let c = self.c();
        let h_sqrt = self.metacentric_height.h_cross().sqrt();
        let res = 2. *  c * self.b / h_sqrt;
        log::info!("\t RollingPeriod c:{c} h_sqrt: {h_sqrt} T:{res}");
        res
    }
}
#[doc(hidden)]
pub trait IRollingPeriod {
    /// Период качки судна
    fn calculate(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeRollingPeriod {
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeRollingPeriod {
    pub fn new(
        value: f64,
    ) -> Self {
        Self {
            value,
        }
    }
}
#[doc(hidden)]
impl IRollingPeriod for FakeRollingPeriod {
    fn calculate(&self) -> f64 {
        self.value
    }
}

