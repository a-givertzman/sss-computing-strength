//! Промежуточные структуры для serde_json для парсинга данных
//! Координаты осадок судна относительно центра корпуса судна
use crate::Position;
use super::{DataArray, PointData};
/// Координаты осадок судна относительно центра
pub type LoadLineDataArray = DataArray<PointData>;
//
impl LoadLineDataArray {
    /// Преобразование данных в массив
    pub fn load_line_data(&self) -> Vec<LoadLineParsedData> {
        self.data
            .iter()
            .map(|v| LoadLineParsedData {
                name: v.name.clone(),
                pos: Position::new(v.x, v.y, v.z),
            })
            .collect()
    }
}
//
#[derive(Debug, Clone, PartialEq)]
pub struct LoadLineParsedData {
    /// Название
    pub name: String,
    /// Координаты осадок судна относительно центра корпуса судна, м
    pub pos: Position,
}
//
impl std::fmt::Display for LoadLineParsedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LoadLineParsedData(name:{} pos:{})", self.name, self.pos)
    }
}
