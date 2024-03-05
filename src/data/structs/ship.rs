//! Промежуточные структуры для serde_json для парсинга данных судна
use serde::{Deserialize, Serialize};
use crate::error::Error;
/// Общие по судну и расчету
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipData {
    /// Длина судна
    pub ship_length: f64,
    /// Плотность воды
    pub water_density: f64,
    /// Количество разбиений
    pub n_parts: usize,
}
///
impl std::fmt::Display for ShipData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ShipData(ship_length:{}, water_density:{}, n_parts:{})",
            self.ship_length,
            self.water_density,
            self.n_parts
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
        let result: Self = serde_json::from_str(src)?;
        let errors = result.data.iter().filter_map(|v| {
            if v.ship_length <= 0. {
                return Some(Error::Parameter(format!("Error parse ShipArray: value of ship's length must be positive {}", v)));
            }
            if v.n_parts <= 0 {
                return Some(Error::Parameter(format!("Error parse ShipArray: number of frames must be positive {}", v)));
            }
            if v.water_density <= 0. {
                return Some(Error::Parameter(format!("Error parse ShipArray: value of water density must be positive {}", v)));
            }
            None
        }).collect::<Vec<_>>();
        if let Some(err) = errors.into_iter().last() {
            return Err(err);
        };
        Ok(result)
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
        let result: Self = serde_json::from_str(src)?;
   /*     if result.data.len() <= 1 {
            return Err(Error::Parameter(format!("Error parse CenterWaterlineArray: number of waterline's points greater or equal to 2")));
        }
   */     Ok(result)
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
        let result: Self = serde_json::from_str(src)?;
 /*       if result.data.len() <= 1 {
            return Err(Error::Parameter(format!("Error parse RadLongDataArray: number of rad_long's points greater or equal to 2")));
       }
*/      Ok(result)
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
        let result: Self = serde_json::from_str(src)?;
 /*       if result.data.len() <= 1 {
            return Err(Error::Parameter(format!("Error parse MeanDraughtDataArray: number of mean_draught's points greater or equal to 2")));
        }*/
        Ok(result)
    }
    /// Преобразование данных в массив ключ + значение
    pub fn data(&self) -> Vec<(f64, f64)> {
        self.data.iter().map(|v| (v.key, v.value) ).collect()
    }
}
/// Кривая отстояния центра величины погруженной части судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterShiftData {
    pub key: f64,
    pub value_x: f64,
    pub value_y: f64,
    pub value_z: f64,
}
/// Массив кривых отстояния центра величины погруженной части судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterShiftDataArray {
    pub data: Vec<CenterShiftData>,
}
///
#[allow(dead_code)]
impl CenterShiftDataArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        let result: Self = serde_json::from_str(src)?;
   /*     if result.data.len() <= 1 {
            return Err(Error::Parameter(format!("Error parse CenterShiftDataArray: number of center_shift's points greater or equal to 2")));
        }*/
        Ok(result)
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
