//! Промежуточные структуры для serde_json для парсинга данных груза
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::data::structs::DataArray;

/// Груз, приходящийся на шпацию
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadsData {
    /// индекс шпации
    pub frame_space_index: usize,
    /// Параметр в виде текста
    pub key: String,
    /// Масса на шпацию
    pub value: f64,
}
///
impl std::fmt::Display for LoadsData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(index:{}, key:{}, value:{} )",
            self.frame_space_index,
            self.key,
            self.value,
        )
    }
}
/// Массив данных по грузам
pub type LoadConstantArray = DataArray<LoadsData>;
///
impl LoadConstantArray {
    /// Преобразование и возвращает данные в виде мапы индекс шпации/масса
    pub fn data(&self) -> HashMap<usize, f64> {
        let mut map = HashMap::new();
        self.data.iter().for_each(|v| {
            if let Some(value) = map.get_mut(&v.frame_space_index) {
                *value += v.value;
            } else {
                map.insert(v.frame_space_index, v.value);       
            }
        });
        map
    }
}
