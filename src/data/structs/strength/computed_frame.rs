//! Промежуточные структуры для serde_json для парсинга данных
//! разбиения корпуса для расчете эпюров
use serde::{Deserialize, Serialize};
use crate::data::structs::DataArray;
/// Данные по шпангоуту
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComputedFrameData {
    /// Индекс шпангоута
    pub index: i32,
    /// Начало шпангоута
    pub start_x: f64,
    /// Конец шпангоута
    pub end_x: f64,
}
//
impl std::fmt::Display for ComputedFrameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ComputedFrameData(index:{}, start_x:{} end_x:{} )",
            self.index, self.start_x, self.end_x,
        )
    }
}
pub type ComputedFrameDataArray = DataArray<ComputedFrameData>;
//
impl ComputedFrameDataArray {
    /// Преобразование и возвращает данные в виде вектора (индекс, начало, конец) шпации
    pub fn data(mut self) -> Vec<(f64, f64)> {
        let mut result: Vec<_> = self
            .data
            .iter_mut()
            .map(|v| (v.index, v.start_x, v.end_x))
            .collect();
        result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        result
            .into_iter()
            .map(|(_, start_x, end_x)| (start_x, end_x))
            .collect::<Vec<(f64, f64)>>()
    }
}
