//! Промежуточные структуры для serde_json для парсинга данных
//! Координаты и диаметр винтов 
//! относительно центра корпуса судна
use serde::{Deserialize, Serialize};
use crate::Position;
use super::DataArray;
/// Координаты и диаметр винтов  относительно центра судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScrewData {
    /// Название 
    pub name: String, 
    /// Координаты центра винта относительно центра корпуса судна, м
    pub z: f64,
    pub x: f64,
    pub y: f64,  
    /// Диаметр винта, м
    pub d: f64,  
}
//
impl std::fmt::Display for ScrewData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ScrewData(name:{} pos:(x:{} y:{} z:{}) d:{})",
            self.name, self.x, self.y, self.z, self.d
        )
    }
}
//
pub type ScrewDataArray = DataArray<ScrewData>;
//
impl ScrewDataArray {
    /// Преобразование данных в массив
    pub fn data(&self) -> Vec<ScrewParsedData> {      
        self.data.iter().map(|v| ScrewParsedData{name: v.name.clone(), pos: Position::new(v.x, v.y, v.z), d: v.d}).collect()
    }  
}
//
#[derive(Debug, Clone, PartialEq)]
pub struct ScrewParsedData {
    /// Название 
    pub name: String, 
    /// Координаты центра винта относительно центра корпуса судна, м
    pub pos: Position,
    /// Диаметр винта, м
    pub d: f64,  
}
//
impl std::fmt::Display for ScrewParsedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ScrewParsedData(name:{} pos:{} d:{})",
            self.name, self.pos, self.d
        )
    }
}
