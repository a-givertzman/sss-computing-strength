//! Безразмерный множитель S Табл. 2.1.5.1-3
use crate::data::structs::DataArray;
use serde::{Deserialize, Serialize};

/// Промежуточные структуры для serde_json для парсинга
/// Безразмерный множитель S Табл. 2.1.5.1-3
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiplerSData {
    /// Район плавания суджна
    pub area: String,
    /// Период качки Т, с
    pub t: f64,
    /// Безразмерный множитель S
    pub s: f64,
}
///
impl std::fmt::Display for MultiplerSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MultiplerSData(area:{}, T:{}, S:{} )",
            self.area, self.t, self.s,
        )
    }
}
///
pub type MultiplerSArray = DataArray<MultiplerSData>;
///
impl MultiplerSArray {
    /// Условия района плавания
    pub fn get_area(&self, area_name: &str) -> Vec<(f64, f64)> {
        let filter = if area_name.contains("Unlimited") {
            |data: &&MultiplerSData| data.area.eq_ignore_ascii_case(&"Unlimited")
        } else {
            |data: &&MultiplerSData| !data.area.eq_ignore_ascii_case(&"Unlimited")
        };

        self.data
            .iter()
            .filter(|data| filter(data))
            .map(|data| (data.t, data.s))
            .collect::<Vec<(f64, f64)>>()
    }
}
