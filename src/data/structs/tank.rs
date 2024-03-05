//! Промежуточные структуры для serde_json для парсинга данных цистерн
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::error::Error;
/// Данные по цистерне содержащей жидкий груз
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TankData {
    /// ID цистерны
    pub tank_id: usize,
    /// Параметр в виде текста
    pub key: String,
    /// Величина параметра
    pub value: f64,
}
///
impl std::fmt::Display for TankData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TankData(tank_id:{}, key:{}, value:{})",
            self.tank_id,
            self.key,
            self.value
        )
    }
}
/// Массив данных по жидким грузам
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TankDataArray {
    pub data: Vec<TankData>,
}
///
#[allow(dead_code)]
impl TankDataArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        let result: Self = serde_json::from_str(src)?;
 /*       if let Some(tank) = result.data.iter().find(|t| t.key.contains("density") && t.value <= 0.) {
            return Err(Error::Parameter(format!("Error parse TankDataArray: density of liquid must be greater or equal to 0 {}", tank)));
        }
        if let Some(tank) = result.data.iter().find(|t| t.key.contains("volume") && t.value <= 0.) {
            return Err(Error::Parameter(format!("Error parse TankDataArray: volume of liquid must be greater or equal to 0 {}", tank)));
        }
        if let Some(tank) = result.tanks.iter().find(|t| t.center.len() <= 1) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(tank.center.len() as u64),
                &"number of center's points greater or equal to 2",
            ));
        }
        if let Some(tank) = result.tanks.iter().find(|t| t.free_surf_inertia.len() <= 1) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(tank.free_surf_inertia.len() as u64),
                &"number of free_surf_inertia's points greater or equal to 2",
            ));
        }*/
        Ok(result)
    }
    /// Преобразование и возвращает данные в виде мапы индекс груза/данные груза
    pub fn data(self) -> HashMap<usize, HashMap<String, f64>> {
        let mut map: HashMap<usize, HashMap<String, f64>> = HashMap::new();
        self.data.into_iter().for_each(|v| {
            if let Some(sub_map) = map.get_mut(&v.tank_id) {
                sub_map.insert(v.key, v.value);
            } else {
                map.insert(v.tank_id, HashMap::from([(v.key, v.value)]));       
            }
        });
        map
    }
}

/// Кривая координат центра объема жидкости в цистерне в системе координат судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterVolumeData {
    data: HashMap<usize, Vec<(f64, f64, f64, f64)>>,
}
///
#[allow(dead_code)]
impl CenterVolumeData {
    ///
    pub fn new(data: HashMap<usize, Vec<(f64, f64, f64, f64)>>) -> Self {
        Self{data}
    }
    /// Для заданного ID возвращает вектор значений по Х
    pub fn x(&self, id: usize) -> Option<Vec<(f64, f64)>> {
        self.data.get(&id).map(|v| v.iter().map(|v| (v.0, v.1)).collect::<Vec<_>>())
    }
    /// Для заданного ID возвращает вектор значений по Y
    pub fn y(&self, id: usize) -> Option<Vec<(f64, f64)>> {
        self.data.get(&id).map(|v| v.iter().map(|v| (v.0, v.2)).collect::<Vec<_>>())
    }
    /// Для заданного ID возвращает вектор значений по Z
    pub fn z(&self, id: usize) -> Option<Vec<(f64, f64)>> {
        self.data.get(&id).map(|v| v.iter().map(|v| (v.0, v.3)).collect::<Vec<_>>())
    }
}
/// Толчка кривой координат центра объема жидкости в цистерне в системе координат судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterVolumeUnit {
    pub tank_id: usize,
    pub key: f64,
    pub value_x: f64,
    pub value_y: f64,
    pub value_z: f64,
}
///
impl std::fmt::Display for CenterVolumeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CenterVolumeData(tank_id:{}, key:{}, value_x:{}, value_y:{}, value_z:{} )",
            self.tank_id,
            self.key,
            self.value_x,
            self.value_y,
            self.value_z,
        )
    }
}
/// Массив кривых координат центра объема жидкости
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CenterVolumeArray {
    pub data: Vec<CenterVolumeUnit>,
}
#[allow(dead_code)]
impl CenterVolumeArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        let result: Self = serde_json::from_str(src)?;
        if result.data.len() <= 1 {
            return Err(Error::Parameter(format!("Error parse CenterVolumeArray: number of points must be greater or equal to 2")));
        }
        if let Some(v) = result.data.iter().find(|v| v.key < 0. ) {
            return Err(Error::Parameter(format!("Error parse CenterVolumeArray: key of immersion_area's points must be greater or equal to 0, {}", v)));
        }
        Ok(result)
    }
    /// Преобразование и возвращает данные в виде мапы индекс груза/вектор данных по кривой
    pub fn data(self) -> HashMap<usize, Vec<(f64, f64, f64, f64)>> {
        let mut map: HashMap<usize, Vec<(f64, f64, f64, f64)>> = HashMap::new();
        self.data.into_iter().for_each(|v| {
            if let Some(vector) = map.get_mut(&v.tank_id) {
                vector.push((v.key, v.value_x, v.value_y, v.value_z));
            } else {
                map.insert(v.tank_id, vec![(v.key, v.value_x, v.value_y, v.value_z)]);       
            }
        });
        map
    }    
}

/// Кривая момента инерции площади свободной поверхности жидкости
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FreeMomentInertiaData {
    data: HashMap<usize, Vec<(f64, f64, f64)>>,
}
///
#[allow(dead_code)]
impl FreeMomentInertiaData {
    ///
    pub fn new(data: HashMap<usize, Vec<(f64, f64, f64)>>) -> Self {
        Self{data}
    }
    /// Для заданного ID возвращает вектор значений по Х
    pub fn x(&self, id: usize) -> Option<Vec<(f64, f64)>> {
        self.data.get(&id).map(|v| v.iter().map(|v| (v.0, v.1)).collect::<Vec<_>>())
    }
    /// Для заданного ID возвращает вектор значений по Y
    pub fn y(&self, id: usize) -> Option<Vec<(f64, f64)>> {
        self.data.get(&id).map(|v| v.iter().map(|v| (v.0, v.2)).collect::<Vec<_>>())
    }
}
/// Точка кривой момента инерции площади свободной поверхности жидкости
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FreeMomentInertiaUnit {
    pub tank_id: usize,
    pub key: f64,
    pub value_x: f64,
    pub value_y: f64,
}
///
impl std::fmt::Display for FreeMomentInertiaUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FreeMomentInertiaData(tank_id:{}, key:{}, value_x:{}, value_y:{})",
            self.tank_id,
            self.key,
            self.value_x,
            self.value_y,
        )
    }
}
/// Массив кривых момента инерции площади свободной поверхности жидкости
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FreeMomentInertiaArray {
    pub data: Vec<FreeMomentInertiaUnit>,
}
#[allow(dead_code)]
impl FreeMomentInertiaArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        let result: Self = serde_json::from_str(src)?;
        if result.data.len() <= 1 {
            return Err(Error::Parameter(format!("Error parse FreeMomentInertiaArray: number of points must be greater or equal to 2")));
        }
        if let Some(v) = result.data.iter().find(|v| v.key < 0. ) {
            return Err(Error::Parameter(format!("Error parse FreeMomentInertiaArray: key of inertia's points must be greater or equal to 0, {}", v)));
        }
        Ok(result)
    }
    /// Преобразование и возвращает данные в виде мапы индекс груза/вектор данных по кривой
    pub fn data(self) -> HashMap<usize, Vec<(f64, f64, f64)>> {
        let mut map: HashMap<usize, Vec<(f64, f64, f64)>> = HashMap::new();
        self.data.into_iter().for_each(|v| {
            if let Some(vector) = map.get_mut(&v.tank_id) {
                vector.push((v.key, v.value_x, v.value_y));
            } else {
                map.insert(v.tank_id, vec![(v.key, v.value_x, v.value_y)]);       
            }
        });
        map
    }
}
