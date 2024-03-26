//! Безразмерный множитель Х_2 Табл. 2.1.5.1-2
use serde::{Deserialize, Serialize};

/// Промежуточные структуры для serde_json для парсинга
/// Безразмерный множитель Х_2 Табл. 2.1.5.1-2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiplerX2Data {
    /// Коэфициент общей полноты судна Cb
    pub c_b: f64,
    /// Безразмерный множитель Х_2
    pub x2: f64,
}
///
impl std::fmt::Display for MultiplerX2Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MultiplerX2Data(Cb:{}, X2:{} )",
            self.c_b, self.x2,
        )
    }
}