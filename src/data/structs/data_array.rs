//! Промежуточные структуры для serde_json для парсинга данных судна
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::serde_parser::IFromJson;

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pair {
    pub key: f64,
    pub value: f64,
}
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Triple {
    pub key: f64,
    pub value_x: f64,
    pub value_y: f64,
}
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quadruple {
    pub key: f64,
    pub value_x: f64,
    pub value_y: f64,
    pub value_z: f64,
}
/// Массив ключ + значение
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataArray<T> {
    pub data: Vec<T>,
    pub error: HashMap<String, String>,
}
///
impl <T> IFromJson for DataArray<T> {
    ///
    fn error(&self) -> Option<&String> {
        self.error.values().next()
    }
}
///
impl DataArray<Pair> {
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value) ).collect()
    }
}
///
#[allow(dead_code)]
impl DataArray<Triple> {
    /// Преобразование данных в массив ключ + значение по х
    pub fn x(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value_x) ).collect()
    }
    /// Преобразование данных в массив ключ + значение по у
    pub fn y(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value_y) ).collect()
    }   
}
///
impl DataArray<Quadruple> {
    /// Преобразование данных в массив ключ + значение по х
    pub fn x(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value_x) ).collect()
    }
    /// Преобразование данных в массив ключ + значение по у
    pub fn y(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value_y) ).collect()
    }
    /// Преобразование данных в массив ключ + значение по z
    pub fn z(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value_z) ).collect()
    }    
}
