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
pub struct HStrAreaData {
    /// Название 
    pub name: String, 
    /// Значение площади, м^2
    pub value: f64,
    /// Ограничение по оси Х
    pub bound_x1: f64,
    pub bound_x2: f64, 
    /// Тип ограничения, значение в метрах или номера
    /// физических шпангоутов
    pub bound_type: String,
}
///
impl std::fmt::Display for HStrAreaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HStrAreaData(value:{} bound:({}, {}), bound_type:{})",
            self.value, self.bound_x1, self.bound_x2, self.bound_type,
        )
    }
}
///
pub type HStrAreaDataArray = DataArray<HStrAreaData>;
///
impl HStrAreaDataArray {
    /// Преобразование данных в массив
    pub fn data(self) -> Vec<HStrAreaData> {
        self.data
    }  
}
/// Площадь обледенения
#[derive(Debug)]
pub struct ParsedHStrArea {
    /// Значение площади, м^2
    pub value: f64,  
    /// Ограничение по оси Х
    pub bound_x: (f64, f64),
}
///
impl std::fmt::Display for ParsedHStrArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParsedHStrArea(area_value:{} bound:({}, {}))",
            self.value, self.bound_x.0, self.bound_x.1,
        )
    }
}