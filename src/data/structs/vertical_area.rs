//! Промежуточные структуры для serde_json для парсинга данных
//! Постоянные площади обледенения: горизонтальные поверхности и поверхности
//! парусности корпуса судна
use serde::{Deserialize, Serialize};

use super::DataArray;

/// Площадь обледенения
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerticalAreaData {
    /// Название 
    pub name: String, 
    /// Значение площади, м^2
    pub area_value: f64,
    /// Смещение центра по оси Х
    pub shift_x: Option<f64>,  
    /// Ограничение по оси Х
    pub bound_x1: f64,
    pub bound_x2: f64, 
    /// Тип ограничения, значение в метрах или номера
    /// физических шпангоутов
    pub bound_type: String,
}
///
impl std::fmt::Display for VerticalAreaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IcingAreaData(area_value:{}, shift_x:{:?} bound_x1:{}, bound_x2:{}, bound_type:{})",
            self.area_value, self.shift_x, self.bound_x1, self.bound_x2, self.bound_type,
        )
    }
}
///
pub type VerticalAreaDataArray = DataArray<VerticalAreaData>;
///
impl VerticalAreaDataArray {
    /// Преобразование данных в массив
    pub fn data(self) -> Vec<VerticalAreaData> {
        self.data
    }  
}
/// Площадь обледенения
#[derive(Debug)]
pub struct ParsedVerticalArea {
    /// Значение площади, м^2
    pub value: f64,
    /// Смещение центра по оси Х
    pub shift_x: Option<f64>,    
    /// Ограничение по оси Х
    pub bound_x: (f64, f64),
}
