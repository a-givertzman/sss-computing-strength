//! Промежуточные структуры для serde_json для парсинга данных
//! Постоянные площади обледенения: горизонтальные поверхности и поверхности
//! парусности корпуса судна
use serde::{Deserialize, Serialize};

use super::DataArray;

/// Площадь обледенения
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IcingAreaData {
    pub name: String, 
    /// Тип площади: 'v'-вертикальная, 'h'-горизонтальная 
    pub area_type: String,
    /// Значение площади, м^2
    pub area_value: f64,
    /// Ограничение по оси Х
    pub bound_x1: String,
    pub bound_x2: String, 
    /// Тип ограничения, значение в метрах или номера
    /// физических шпангоутов
    pub bound_type: String,
}
///
impl std::fmt::Display for IcingAreaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IcingAreaData(area_type:{}, area_value:{}, bound_x1:{}, bound_x2:{}, bound_type:{})",
            self.area_type, self.area_value, self.bound_x1, self.bound_x2, self.bound_type,
        )
    }
}
///
pub type IcingAreaDataArray = DataArray<IcingAreaData>;
///
impl IcingAreaDataArray {
    /// Преобразование данных в массив
    pub fn data(self) -> Vec<IcingAreaData> {
        self.data
    }  
}
/// Площадь обледенения
#[derive(Debug)]
pub struct ParsedIcingArea {
    /// Значение площади, м^2
    pub area_value: f64,
    /// Ограничение по оси Х
    pub bound_x: (f64, f64),
}
