//! Промежуточные структуры для serde_json для парсинга данных
//! Координаты отметок заглубления на корпусе судна 
//! относительно центра корпуса судна
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::DataArray;

/// Координаты отметок заглубления на корпусе судна 
/// относительно центра
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DraftMarkData {
    /// Название 
    pub name: String, 
    /// Координаты относительно центра корпуса судна, м
    pub z: f64,
    pub x: f64,
    pub y: f64,  
}
///
impl std::fmt::Display for DraftMarkData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DraftMarkData(name:{} pos:(x:{} y:{} z:{}))",
            self.name, self.x, self.y, self.z
        )
    }
}
///
pub type DraftMarkDataArray = DataArray<DraftMarkData>;
///
impl DraftMarkDataArray {
    /// Преобразование данных в массив
    pub fn data(&self) -> HashMap<String, Vec<(f64, f64, f64)>> {      
        let mut map: HashMap<String, Vec<(f64, f64, f64)>> = HashMap::new();
        self.data.iter().for_each(|v| {
            if let Some(vector) = map.get_mut(&v.name) {
                vector.push((v.x, v.y, v.z));
            } else {
                map.insert(v.name.clone(), vec![(v.x, v.y, v.z)]);       
            }
        });
        map
    }  
}

