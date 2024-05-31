//! Промежуточные структуры для serde_json для парсинга данных шпангоутов
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::DataArray;

/// Данные по шпангоуту
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameAreaData {
    /// Индекс шпангоута
    pub frame_index: i32,
    /// Осадка
    pub draft: f64,
    /// Погруженная площадь
    pub area: f64,
}
///
impl std::fmt::Display for FrameAreaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FrameAreaData(frame_index:{}, draft:{}, area:{} )",
            self.frame_index, self.draft, self.area,
        )
    }
}
pub type FrameAreaDataArray = DataArray<FrameAreaData>;
///
impl FrameAreaDataArray {
    /// Преобразование и возвращает данные в виде мапы индекс/данные шпангоута
    pub fn data(self) -> HashMap<i32, Vec<(f64, f64)>> {
        let mut map: HashMap<i32, Vec<(f64, f64)>> = HashMap::new();
        self.data.into_iter().for_each(|v| {
            if let Some(vector) = map.get_mut(&v.frame_index) {
                vector.push((v.draft, v.area));
            } else {
                map.insert(v.frame_index, vec![(v.draft, v.area)]);       
            }
        });
        map
    }
}
/// Шпангоут
#[derive(Debug, PartialEq, PartialOrd)]
pub struct ParsedFrameData {
    /// Координата по х относительно кормы
    pub x: f64,
    /// кривая погружаемой площади
    pub immersion_area: Vec<(f64, f64)>,
}
///
impl std::fmt::Display for ParsedFrameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParsedFrameData(x:{}, immersion_area.len:{} )",
            self.x,
            self.immersion_area.len(),
        )
    }
}
