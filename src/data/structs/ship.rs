//! Промежуточные структуры для serde_json для парсинга данных судна
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::error::Error;
/// Общие по судну и расчету
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipData {
    /// Параметр в виде текста
    pub key: String,
    /// Величина параметра
    pub value: f64,
}
///
impl std::fmt::Display for ShipData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ShipData(key:{}, value:{})",
            self.key,
            self.value,
        )
    }
}
/// Массив данных по расчету
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipArray {
    pub data: Vec<ShipData>,
}
///
#[allow(dead_code)]
impl ShipArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(src)?)
    }
    /// Преобразование и возвращает данные в виде мапы ключ/значение
    pub fn data(self) -> HashMap<String, f64> {
        self.data.into_iter().map(|v| (v.key, v.value)).collect()
    }
}
///
/// Кривая погружаемой площади от осадки
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterWaterlineData {
    pub key: f64,
    pub value: f64,
}
/// Массив кривых погружаемой площади от осадки
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterWaterlineArray {
    pub data: Vec<CenterWaterlineData>,
}
///
#[allow(dead_code)]
impl CenterWaterlineArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(src)?)
    }
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value) ).collect()
    }
}
/// Кривая продольного метацентрического радиуса
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RadLongData {
    pub key: f64,
    pub value: f64,
}
/// Массив кривых продольного метацентрического радиуса
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RadLongDataArray {
    pub data: Vec<RadLongData>,
}
///
#[allow(dead_code)]
impl RadLongDataArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(src)?)
    }
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value) ).collect()
    }
}
/// Кривая средней осадки
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeanDraughtData {
    pub key: f64,
    pub value: f64,
}
/// Массив кривых средней осадки
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeanDraughtDataArray {
    pub data: Vec<MeanDraughtData>,
}
///
#[allow(dead_code)]
impl MeanDraughtDataArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(src)?)
    }
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value) ).collect()
    }
}
/// Кривая отстояния центра величины погруженной части судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterDraughtShiftData {
    pub key: f64,
    pub value_x: f64,
    pub value_y: f64,
    pub value_z: f64,
}
/// Массив кривых отстояния центра величины погруженной части судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterDraughtShiftDataArray {
    pub data: Vec<CenterDraughtShiftData>,
}
///
#[allow(dead_code)]
impl CenterDraughtShiftDataArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(src)?)
    }
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
