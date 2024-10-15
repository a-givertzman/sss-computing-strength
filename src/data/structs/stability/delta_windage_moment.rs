//! Промежуточные структуры для serde_json для парсинга данных
//! Разница в статических моментах относительно миделя и ОП
use serde::{Deserialize, Serialize};

use super::DataArray;

/// Данные по шпангоуту
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeltaWindageMomentData {
    /// Осадка
    pub draught: f64,
    /// Разница в статических моментах относительно миделя
    pub value_x: f64,
    /// Разница в статических моментах относительно ОП
    pub value_z: f64,
}
//
impl std::fmt::Display for DeltaWindageMomentData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ComputedFrameData(draught:{}, value_x:{}, value_z:{})",
            self.draught, self.value_x, self.value_z,
        )
    }
}
pub type DeltaWindageMomentDataArray = DataArray<DeltaWindageMomentData>;
//
impl DeltaWindageMomentDataArray {
    /// Преобразование данных в массив ключ + значение по х
    pub fn x(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.draught, v.value_x)).collect()
    }
    /// Преобразование данных в массив ключ + значение по z
    pub fn z(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.draught, v.value_z)).collect()
    }
}
