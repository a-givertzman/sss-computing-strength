//! Промежуточные структуры для serde_json для парсинга данных
//! Постоянные площади обледенения: горизонтальные поверхности
//! корпуса судна для расчета прочности. Дополнительно содержат
//! данные по распределению по длинне.
use serde::{Deserialize, Serialize};
use crate::data::structs::DataArray;
/// Постоянные площади обледенения: горизонтальные поверхности
/// корпуса судна для расчета прочности. Дополнительно содержат
/// данные по распределению по длинне.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HStrArea {
    /// Название
    pub name: String,
    /// Значение площади, м^2
    pub value: f64,
    /// Ограничение по оси Х, м
    pub bound_x1: f64,
    pub bound_x2: f64,
}
//
impl std::fmt::Display for HStrArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HStrArea(value:{} bound:({}, {}))",
            self.value, self.bound_x1, self.bound_x2
        )
    }
}
//
pub type HStrAreaArray = DataArray<HStrArea>;
//
impl HStrAreaArray {
    /// Преобразование данных в массив
    pub fn data(self) -> Vec<HStrArea> {
        self.data
    }
}
