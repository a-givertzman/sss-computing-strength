//! Промежуточные структуры для serde_json для парсинга данных груза
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::error::Error;

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadSpaceArray {
    pub data: Vec<LoadSpaceData>,
}
///
#[allow(dead_code)]
impl LoadSpaceArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        let result: Self = serde_json::from_str(src)?;
  /*      if let Some(s) = result.data.iter().find(|s| s.mass < 0.) {
            return Err(Error::Parameter(format!("Error parse LoadSpaceArray: mass of load_space must be greater or equal to 0, {}", s)));
        }
        if let Some(s) = result.data.iter().find(|s| s.bound.0 >= s.bound.1 || s.bound.2 >= s.bound.3 ) {
            return Err(Error::Parameter(format!("Error parse LoadSpaceArray: Bound error! {}", s)));
        }*/
        Ok(result)
    }
    /// Преобразование и возвращает данные в виде мапы id/данные груза
    pub fn data(self) -> HashMap<usize, HashMap<String, f64>> {
        let mut map: HashMap<usize, HashMap<String, f64>> = HashMap::new();
        self.data.into_iter().for_each(|v| {
            if let Some(sub_map) = map.get_mut(&v.space_id) {
                sub_map.insert(v.key, v.value);
            } else {
                map.insert(v.space_id, HashMap::from([(v.key, v.value)]));       
            }
        });
        map
    }
}