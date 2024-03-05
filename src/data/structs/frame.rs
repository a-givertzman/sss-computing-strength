//! Промежуточные структуры для serde_json для парсинга данных шпангоутов
use std::collections::HashMap;

use crate::error::Error;
use serde::{Deserialize, Serialize};
/// Данные по шпангоуту
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameData {
    /// Индекс шпангоута
    pub index: usize,
    /// Параметр в виде текста
    pub key: String,
    /// Величина параметра
    pub value: f64,
}
///
impl std::fmt::Display for FrameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FrameData(index:{}, key:{}, value:{} )",
            self.index, self.key, self.value,
        )
    }
}
/// Массив данных по шпангоутам
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameDataArray {
    pub data: Vec<FrameData>,
}
///
#[allow(dead_code)]
impl FrameDataArray {
    ///
    pub fn parse(src: &str) -> Result<Self, Error> {
        let result: Self = serde_json::from_str(src)?;
        /*        if result.data.len() <= 1 {
            return Err(Error::Parameter(format!("Error parse FrameDataArray: number of frames must be greater or equal to 2")));
        }
        if let Some(frame) = result.data.iter().find(|f| f.index >= result.data.len()) {
            return Err(Error::Parameter(format!("Error parse FrameDataArray: index of frame lower then frames.len(), {}", frame)));
        }
        let qnt_unique_index = result.data.iter().map(|f| f.index ).collect::<HashSet<_>>().len();
        if result.data.len() != qnt_unique_index {
            return Err(Error::Parameter(format!("Error parse FrameDataArray: index of frame must be unique")));
        }
        if let Some(frame) = result.data.iter().find(|f| f.value < 0. ) {
            return Err(Error::Parameter(format!("Error parse FrameDataArray: value of frame must be greater or equal to 0, {}", frame)));
        }*/
        Ok(result)
    }
    /// Преобразование и возвращает данные в виде мапы индекс/данные шпангоута
    pub fn data(self) -> HashMap<usize, HashMap<String, f64>> {
        let mut map: HashMap<usize, HashMap<String, f64>> = HashMap::new();
        self.data.into_iter().for_each(|v| {
            if let Some(sub_map) = map.get_mut(&v.index) {
                sub_map.insert(v.key, v.value);
            } else {
                map.insert(v.index, HashMap::from([(v.key, v.value)]));       
            }
        });
        map
    }
}
/// Кривая погружаемой площади шпангоута от осадки
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameAreaData {
    data: HashMap<usize, Vec<(f64, f64)>>,
}
///
#[allow(dead_code)]
impl FrameAreaData {
    ///
    pub fn new(data: HashMap<usize, Vec<(f64, f64)>>) -> Self {
        Self{data}
    }
    /// Для заданного ID возвращает вектор значений
    pub fn get(&self, id: usize) -> Option<Vec<(f64, f64)>> {
        self.data.get(&id).map(|v| v.iter().map(|v| (v.0, v.1)).collect::<Vec<_>>())
    }
}
/// Точка для кривой погружаемой площади шпангоута от осадки
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameAreaUnit {
    /// Индекс шпангоута
    pub frame_index: usize, 
    /// Осадка в точке
    pub key: f64,
    /// Погруженная площадь шпангоута
    pub value: f64,
}
///
impl std::fmt::Display for FrameAreaUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FrameAreaData(frame_index:{}, key:{}, value:{})",
            self.frame_index, self.key, self.value,
        )
    }
}
/// Кривая погружаемой площади шпангоута от осадки
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameAreaArray {
    pub data: Vec<FrameAreaUnit>,
}
#[allow(dead_code)]
impl FrameAreaArray {
    /// Парсинг данных из json строки
    pub fn parse(src: &str) -> Result<Self, Error> {
        let result: Self = serde_json::from_str(src)?;
        if result.data.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error parse FrameAreaArray: number of frames area must be greater or equal to 2"
            )));
        }
        if let Some(frame) = result.data.iter().find(|f| f.key < 0.) {
            return Err(Error::Parameter(format!("Error parse FrameAreaArray: key of immersion_area's points must be greater or equal to 0, {}", frame)));
        }
        if let Some(frame) = result.data.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!("Error parse FrameAreaArray: value of immersion_area's points must be greater or equal to 0, {}", frame)));
        }
        Ok(result)
    }
    /// Преобразование и возвращает данные в виде мапы индекс фрейма/кривая площади
    pub fn data(&self) -> HashMap<usize, Vec<(f64, f64)>> {
        let mut map: HashMap<usize, Vec<(f64, f64)>> = HashMap::new();
        self.data.iter().for_each(|v| {
            if let Some(x) = map.get_mut(&v.frame_index) {
                x.push((v.key, v.value));
            } else {
                map.insert(v.frame_index, vec![(v.key, v.value)]);
            }
        });
        map
    }
}
