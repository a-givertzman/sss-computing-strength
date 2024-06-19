//! Промежуточные структуры для serde_json для парсинга данных груза
use serde::{Deserialize, Serialize};
use crate::data::structs::DataArray;

/// Тип груза, приходящегося на шпацию
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize,)]
pub enum LoadConstantType {
    #[serde(alias="hull")]
    Hull,
    #[serde(alias="equipment")]
    Equipment,
}
///
impl std::fmt::Display for LoadConstantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LoadConstantType::Hull => "Hull",
                LoadConstantType::Equipment => "Equipment", 
            },
        )
    }
}
/// Груз, приходящийся на шпацию
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadConstantData {
    /// Масса на шпацию
    pub mass: f64,
    /// Диапазон по длинне
    pub bound_x1: f64,
    pub bound_x2: f64,
    /// Тип задания диапазона 
    /// (физ. шпангоуты или метры)
    pub bound_type: String,  
    /// Тип груза, приходящегося на шпацию
    pub loading_type: LoadConstantType,
}
///
impl std::fmt::Display for LoadConstantData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadConstantData(mass:{}, bound_x1:{}, bound_x2:{}, bound_type:{}, type:{})",
            self.mass,            
            self.bound_x1,
            self.bound_x2,
            self.bound_type,
            self.loading_type,
        )
    }
}
/// Массив данных по грузам
pub type LoadConstantArray = DataArray<LoadConstantData>;
///
impl LoadConstantArray {
    /// 
    pub fn data(self) -> Vec<LoadConstantData> {
        self.data
    }
}
/// Груз
#[derive(Debug)]
pub struct ParsedLoadConstantData {
    /// масса, т
    pub mass: f64,
    /// Границы груза
    pub bound_x: (f64, f64),
    /// Границы груза
    pub loading_type: LoadConstantType,
}
