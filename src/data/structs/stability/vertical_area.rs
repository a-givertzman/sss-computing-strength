//! Промежуточные структуры для serde_json для парсинга данных
//! Постоянные площади обледенения: горизонтальные поверхности и поверхности
//! парусности корпуса судна
use serde::{Deserialize, Serialize};

use super::DataArray;

/// Площадь обледенения
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerticalArea {
    /// Осадка, м 
    pub draught: f64,
    /// Значение площади, м^2
    pub area: f64,
    /// Статический момент площади парусности по оси X, m^3
    pub moment_x: f64,  
    /// Статический момент площади парусности по оси Z, m^3
    pub moment_z: f64,
}
///
impl std::fmt::Display for VerticalArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VerticalArea(draught:{}, area:{:?} moment_x:{}, moment_z:{})",
            self.draught, self.area, self.moment_x, self.moment_z,
        )
    }
}
///
pub type VerticalAreaArray = DataArray<VerticalArea>;
///
impl VerticalAreaArray {
    /// Преобразование данных в массив пар (осадкаб площадь)
    pub fn area(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.draught, v.area)).collect()
    }  
    /// Преобразование данных в массив пар (осадка, moment_x)
    pub fn moment_x(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.draught, v.moment_x)).collect()
    } 
    /// Преобразование данных в массив пар (осадка, moment_z)
    pub fn moment_z(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.draught, v.moment_z)).collect()
    } 
}
