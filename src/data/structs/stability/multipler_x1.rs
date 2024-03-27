//! Безразмерный множитель Х_1 Табл. 2.1.5.1-1
use serde::{Deserialize, Serialize};

/// Промежуточные структуры для serde_json для парсинга
/// Безразмерный множитель Х_1 Табл. 2.1.5.1-1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiplerX1Data {
    /// Отношение ширины к осадке B/d
    pub b_div_d: f64,
    /// Безразмерный множитель Х_1
    pub x1: f64,
}
///
impl std::fmt::Display for MultiplerX1Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MultiplerX1Data(B/d:{}, X1:{} )",
            self.b_div_d, self.x1,
        )
    }
}
