//! Навалочный смещаемый груз

use crate::Error;

/// Навалочный смещаемый груз.
/// Имеет свойства смещения груза в сторону крена судна.
pub trait IBulk {
    /// Кренящий момент от смещения сыпучего груза
    fn moment(&self) -> f64;
}
/// Навалочный смещаемый груз.
pub struct Bulk {
    /// Удельный погрузочный объем, м³/т.
    s_f: f64,
    /// Объемный кренящий момент
    moment: f64,
}
//
impl Bulk {
    /// Основной конструктор
    /// * s_f - Удельный погрузочный объем, м³/т.
    /// * moment - Объемный кренящий момент
    pub fn new(
        s_f: f64,     
        moment: f64,
    ) -> Result<Self, Error> {
        if s_f <= 0. {
            return Err(Error::FromString(format!("Bulk new error: s_f {s_f} <= 0.")));
        }
        Ok(Self {
            s_f,         
            moment,
        })
    }
}
//
impl IBulk for Bulk {
    /// Кренящий момент от смещения сыпучего груза
    fn moment(&self) -> f64 {
        self.moment / self.s_f
    }
}
