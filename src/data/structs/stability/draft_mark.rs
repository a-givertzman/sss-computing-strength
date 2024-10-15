//! Промежуточные структуры для serde_json для парсинга данных
//! Координаты отметок заглубления на корпусе судна
//! относительно центра корпуса судна
use std::collections::HashMap;
use crate::Position;
use super::{DataArray, PointData};
/// Координаты отметок заглубления на корпусе судна
/// относительно центра
pub type DraftMarkDataArray = DataArray<PointData>;
//
impl DraftMarkDataArray {
    /// Преобразование данных в массив
    pub fn draft_data(&self) -> Vec<DraftMarkParsedData> {
        let mut map: HashMap<String, Vec<(f64, f64, f64)>> = HashMap::new();
        self.data.iter().for_each(|v| {
            if let Some(vector) = map.get_mut(&v.name) {
                vector.push((v.x, v.y, v.z));
            } else {
                map.insert(v.name.clone(), vec![(v.x, v.y, v.z)]);
            }
        });
        map.into_iter()
            .map(|v| DraftMarkParsedData {
                name: v.0,
                data: v.1.into_iter().map(Position::from).collect(),
            })
            .collect()
    }
}
//
#[derive(Debug, Clone, PartialEq)]
pub struct DraftMarkParsedData {
    /// Название
    pub name: String,
    /// Координаты центра винта относительно центра корпуса судна, м
    pub data: Vec<Position>,
}
//
impl std::fmt::Display for DraftMarkParsedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DraftMarkParsedData(name:{} data:{:?})",
            self.name, self.data
        )
    }
}
