//! Промежуточные структуры для serde_json для парсинга данных шпангоутов
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::DataArray;

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
pub type FrameDataArray = DataArray<FrameData>;
///
impl FrameDataArray {
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
pub type FrameAreaArray = DataArray<FrameAreaUnit>;
///
impl DataArray<FrameAreaUnit> {
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

