//! Промежуточные структуры для serde_json для парсинга данных
//! Постоянные площади обледенения: поверхности
//! парусности корпуса судна
use serde::{Deserialize, Serialize};

use crate::data::structs::DataArray;

/// Площадь обледенения
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerticalArea {
    /// Название 
    pub name: String, 
    /// Значение площади, м^2
    pub value: f64,
    /// Смещение центра по оси Z
    pub shift_z: f64,  
    /// Ограничение по оси Х, м
    pub bound_x1: f64,
    pub bound_x2: f64, 
}
///
impl std::fmt::Display for VerticalArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VerticalArea(avalue:{}, shift_z:{:?} bound:({}, {}))",
            self.value, self.shift_z, self.bound_x1, self.bound_x2
        )
    }
}
///
pub type VerticalAreaArray = DataArray<VerticalArea>;
///
impl VerticalAreaArray {
    /// Преобразование данных в массив
    pub fn data(self) -> Vec<VerticalArea> {
        self.data
    }  
}
