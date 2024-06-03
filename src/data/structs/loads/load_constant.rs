//! Промежуточные структуры для serde_json для парсинга данных груза
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::data::structs::DataArray;

/// Груз, приходящийся на шпацию
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadsData {
    /// Масса на шпацию
    pub mass: f64,
    /// Диапазон по длинне
    pub bound_x1: f64,
    pub bound_x2: f64,
    /// Тип задания диапазона 
    /// (физ. шпангоуты или метры)
    pub bound_type: String,  
}
///
impl std::fmt::Display for LoadsData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadsData(mass:{}, bound_x1:{}, bound_x2:{},bound_type:{} )",
            self.mass,            
            self.bound_x1,
            self.bound_x2,
            self.bound_type,
        )
    }
}
/// Массив данных по грузам
pub type LoadConstantArray = DataArray<LoadsData>;
///
///
impl LoadConstantArray {
    /// 
    pub fn data(self) -> Vec<LoadsData> {
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
}
