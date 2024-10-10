//! Промежуточные структуры для serde_json для парсинга данных
//! Координаты точки на корпусе судна
//! относительно центра корпуса судна

use serde::{Deserialize, Serialize};

/// Координаты точки на корпусе судна
/// относительно центра
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PointData {
    /// Название
    pub name: String,
    /// Координаты относительно центра корпуса судна, м
    pub z: f64,
    pub x: f64,
    pub y: f64,
}
///
impl std::fmt::Display for PointData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PointData(name:{} pos:(x:{} y:{} z:{}))",
            self.name, self.x, self.y, self.z
        )
    }
}
