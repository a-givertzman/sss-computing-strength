//! Промежуточные структуры для serde_json для парсинга данных судна
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::DataArray;

/// Общие по судну и расчету
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipData {
    /// Параметр в виде текста
    pub key: String,
    /// Величина параметра
    pub value: String,
    /// Тип параметра
    pub value_type: String,
}
///
impl std::fmt::Display for ShipData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ShipData(key:{}, value:{} value_type:{})",
            self.key,
            self.value,
            self.value_type,
        )
    }
}
/// Массив данных по расчету
pub type ShipArray = DataArray<ShipData>;
///
impl ShipArray {
    /// Преобразование и возвращает данные в виде мапы ключ/значение
    pub fn data(self) -> HashMap<String, (String, String)> {
        self.data.into_iter().map(|v| (v.key, (v.value, v.value_type))).collect()
    }
}
