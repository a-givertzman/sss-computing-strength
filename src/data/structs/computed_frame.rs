//! Промежуточные структуры для serde_json для парсинга данных
//! разбиения корпуса для расчете эпюров
use serde::{Deserialize, Serialize};

use super::DataArray;

/// Данные по шпангоуту
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComputedFrameData {
    /// Индекс шпангоута
    pub index: usize,
    /// Параметр в виде текста
    pub key: String,
    /// Величина параметра
    pub value: f64,
}
///
impl std::fmt::Display for ComputedFrameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ComputedFrameData(index:{}, key:{}, value:{} )",
            self.index, self.key, self.value,
        )
    }
}
pub type ComputedFrameDataArray = DataArray<ComputedFrameData>;
///
impl ComputedFrameDataArray {
    /// Преобразование и возвращает данные в виде вектора индекс/данные шпангоута
    pub fn data(self) -> Vec<(usize, f64)> {
        let mut res = Vec::new();
        self.data.into_iter().for_each(|v| {
            if v.key == "shift_x" {
                res.push((v.index, v.value));    
            }
        });
        res
    }
}
