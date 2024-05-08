//! Промежуточные структуры для serde_json для парсинга данных
//! разбиения корпуса для расчете эпюров
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::DataArray;

/// Данные по шпангоуту
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComputedFrameData {
    /// Индекс шпангоута
    pub index: i32,
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
            "ComputedFrameData(index:{}, key:{} value:{} )",
            self.index, self.key, self.value,
        )
    }
}
pub type ComputedFrameDataArray = DataArray<ComputedFrameData>;
///
impl ComputedFrameDataArray {
    /// Преобразование и возвращает данные в виде вектора (индекс, начало, конец) шпации
    pub fn data(self) -> Vec<(f64, f64)> {
        let mut map: HashMap<i32, (f64, f64)> = HashMap::new();
        self.data.into_iter().for_each(|v| {
            if let Some((start_x, end_x)) = map.get_mut(&v.index) {
                match v.key.as_str() {
                    "start_x" => *start_x = v.value,
                    "end_x" => *end_x = v.value,
                    _ => (),
                };
            } else {
                match v.key.as_str() {
                    "start_x" => {
                        map.insert(v.index, (v.value, 0.));
                    }
                    "end_x" => {
                        map.insert(v.index, (0., v.value));
                    }
                    _ => (),
                };
            }
        });
        let mut result: Vec<_> = map
            .iter_mut()
            .map(|(index, (start_x, end_x))| (index, start_x, end_x))
            .collect();
        result.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
        result
            .into_iter()
            .map(|(_, &mut start_x, &mut end_x)| (start_x, end_x))
            .collect::< Vec<(f64, f64)>>()
    }
}
