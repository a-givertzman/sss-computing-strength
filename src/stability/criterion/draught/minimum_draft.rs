//! Минимальная осадка

/// Минимальная осадка
pub struct MinimumDraft {
    /// Длинна судна между перпендикулярами
    length_lbp: f64,
}
//
impl MinimumDraft {
    /// Конструктор по умолчанию.
    /// * length_lbp - Длинна судна между перпендикулярами
    pub fn new(
        length_lbp: f64,
    ) -> Self {
        Self {
            length_lbp, 
        }
    }
    /// Теоретическая осадка на миделе
    pub fn middle(&self) -> f64 {
        2.0 + 0.02*self.length_lbp
    }
    /// Теоретическая осадка в носу
    pub fn bow(&self) -> f64 {
        self.middle() - 0.015*self.length_lbp/2.
    }
}
