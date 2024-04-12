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
    pub fn parse(src: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(src)?)
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
    pub fn parse(src: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(src)?)
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
    pub fn parse(src: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(src)?)
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

/// Цистерна
#[derive(Debug)]
pub struct ParsedTankData {
    /// плотность жидкости в цистерне
    pub density: f64,
    /// объем жидкости в цистерне
    pub volume: f64,
    /// границы цистерны, (x1, x2)
    pub bound: (f64, f64),
    /// кривая координат центра объема жидкости в цистерне в системе координат судна по x
    pub center_x: Vec<(f64, f64)>,
    /// кривая координат центра объема жидкости в цистерне в системе координат судна по y
    pub center_y: Vec<(f64, f64)>,
    /// кривая координат центра объема жидкости в цистерне в системе координат судна по z
    pub center_z: Vec<(f64, f64)>,
    /// кривая момента инерции площади свободной поверхности жидкости volume, x - поперечный
    pub free_surf_inertia_x: Vec<(f64, f64)>,
    /// кривая момента инерции площади свободной поверхности жидкостиvolume, y - продольный
    pub free_surf_inertia_y: Vec<(f64, f64)>,
}
///
impl std::fmt::Display for ParsedTankData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(density:{}, volume:{}, bound:(x1:{}, x2:{}), center_x.len:{}, center_y.len:{}, center_z.len:{}, inertia_x.len:{}, inertia_y.len:{}) )",
            self.density,
            self.volume,
            self.bound.0,
            self.bound.1,
            self.center_x.len(),
            self.center_y.len(),
            self.center_z.len(),
            self.free_surf_inertia_x.len(),
            self.free_surf_inertia_y.len(),
        )
    }
}