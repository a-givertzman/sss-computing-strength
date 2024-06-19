//! Промежуточные структуры для serde_json для парсинга данных шпангоутов
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::data::structs::DataArray;

/// Данные по шпангоуту
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameIndexData {
    /// Индекс шпангоута
    pub frame_index: i32,
    /// Координата по оси Х относительно нулевого шпангоута
    pub pos_x: f64,
}
///
impl std::fmt::Display for FrameIndexData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FrameIndexData(frame_index:{},pos_x:{} )",
            self.frame_index, self.pos_x,
        )
    }
}
pub type FrameIndexDataArray = DataArray<FrameIndexData>;
///
impl FrameIndexDataArray {
    /// Преобразование и возвращает данные в виде мапы индекс/данные шпангоута
    pub fn data(self) -> HashMap<i32, f64> {
        self.data.into_iter().map(|v| (v.frame_index, v.pos_x)).collect()
    }
}
