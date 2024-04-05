//! Промежуточные структуры для serde_json для парсинга данных
//! разбиения корпуса для расчете эпюров
use serde::{Deserialize, Serialize};

use super::DataArray;

/// Данные по шпангоуту
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComputedFrameData {
    /// Индекс шпангоута
    pub index: usize,
    /// Величина параметра
    pub value: f64,
}
///
impl std::fmt::Display for ComputedFrameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ComputedFrameData(index:{}, value:{} )",
            self.index, self.value,
        )
    }
}
pub type ComputedFrameDataArray = DataArray<ComputedFrameData>;
///
impl ComputedFrameDataArray {
    /// Преобразование и возвращает данные в виде вектора индекс/данные шпангоута
    pub fn data(self) -> Vec<(usize, f64)> {
        self.data.into_iter().map(|v| {(v.index, v.value)}).collect()
    }
}
