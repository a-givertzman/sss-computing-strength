//! Промежуточные структуры для serde_json для парсинга данных груза
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::DataArray;

/// Груз, конструкции корпуса, контейнер или другой твердый груз
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadSpaceData {
    /// ID груза
    pub space_id: usize,
    /// Параметр в виде текста
    pub key: String,
    /// Величина параметра
    pub value: f64,
}
///
impl std::fmt::Display for LoadSpaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(index:{}, key:{}, value:{} )",
            self.space_id,
            self.key,
            self.value,
        )
    }
}
/// Массив данных по грузам
pub type LoadSpaceArray = DataArray<LoadSpaceData>;
///
impl LoadSpaceArray {
    /// Преобразование и возвращает данные в виде мапы id/данные груза
    pub fn data(&self) -> HashMap<usize, HashMap<String, f64>> {
        let mut map: HashMap<usize, HashMap<String, f64>> = HashMap::new();
        self.data.iter().for_each(|v| {
            if let Some(sub_map) = map.get_mut(&v.space_id) {
                sub_map.insert(v.key.clone(), v.value);
            } else {
                map.insert(v.space_id, HashMap::from([(v.key.clone(), v.value)]));       
            }
        });
        map
    }
}
