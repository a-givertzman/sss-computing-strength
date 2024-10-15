//! Промежуточные структуры для serde_json для парсинга данных судна
use super::DataArray;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Общие по судну и расчету
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipData {
    /// Параметр в виде текста
    pub key: String,
    /// Величина параметра
    pub value: f64,
}
//
impl std::fmt::Display for ShipData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShipData(key:{}, value:{})", self.key, self.value,)
    }
}
/// Массив данных по расчету
pub type ShipParametersArray = DataArray<ShipData>;
//
impl ShipParametersArray {
    /// Преобразование и возвращает данные в виде мапы ключ/значение
    pub fn data(self) -> HashMap<String, f64> {
        self.data.into_iter().map(|v| (v.key, v.value)).collect()
    }
}
