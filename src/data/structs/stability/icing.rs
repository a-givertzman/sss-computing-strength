//! Промежуточные структуры для serde_json для парсинга коэффициентов обледенения
use std::collections::HashMap;

use super::DataArray;
use serde::{Deserialize, Serialize};
/// Коэффициенты обледенения
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IcingData {
    /// Параметр в виде текста
    pub key: String,
    /// Величина параметра
    pub value: f64,
}
//
impl std::fmt::Display for IcingData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IcingData(key:{}, value:{})", self.key, self.value,)
    }
}
/// Массив данных по расчету
pub type IcingArray = DataArray<IcingData>;
//
impl IcingArray {
    /// Преобразование и возвращает данные в виде мапы ключ/значение
    pub fn data(self) -> HashMap<String, f64> {
        self.data.into_iter().map(|v| (v.key, v.value)).collect()
    }
}
