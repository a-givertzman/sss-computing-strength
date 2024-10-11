//! Промежуточные структуры для serde_json для парсинга данных
//! Высота борта на носовом перпендикуляре
//! относительно центра корпуса судна
use crate::Position;
use super::{DataArray, PointData};
/// Высота борта на носовом перпендикуляре судна относительно центра
pub type BowBoardDataArray = DataArray<PointData>;
//
impl BowBoardDataArray {
    /// Преобразование данных в массив
    pub fn bow_board_data(&self) -> Vec<BowBoardParsedData> {      
        self.data.iter().map(|v|BowBoardParsedData{name: v.name.clone(), pos: Position::new(v.x, v.y, v.z)}).collect()
    }  
}
//
#[derive(Debug, Clone, PartialEq)]
pub struct BowBoardParsedData {
    /// Название 
    pub name: String, 
    /// Координаты центра винта относительно центра корпуса судна, м
    pub pos: Position,
}
//
impl std::fmt::Display for BowBoardParsedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BowBoardParsedData(name:{} pos:{})",
            self.name, self.pos
        )
    }
}