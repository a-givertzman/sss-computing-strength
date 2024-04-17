//! Промежуточные структуры для serde_json для парсинга данных
//! Постоянные площади обледенения: горизонтальные поверхности 
//! корпуса судна для расчета остойчивости. 
use serde::{Deserialize, Serialize};

use super::DataArray;

/// Постоянные площади обледенения: горизонтальные поверхности 
/// корпуса судна для расчета остойчивости. Дополнительно содержат 
/// смещение центра поверхности.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HStabAreaData {
    /// Название 
    pub name: String, 
    /// Значение площади, м^2
    pub value: f64,
    /// Смещение центра
    pub shift_x: f64,
    pub shift_y: f64,
    pub shift_z: f64,  
}
///
impl std::fmt::Display for HStabAreaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HStabAreaData(value:{} shift:({} {} {}))",
            self.value, self.shift_x, self.shift_y, self.shift_z
        )
    }
}
///
pub type HStabAreaDataArray = DataArray<HStabAreaData>;
///
impl HStabAreaDataArray {
    /// Преобразование данных в массив
    pub fn data(self) -> Vec<HStabAreaData> {
        self.data
    }  
}

