//! Давление ветра p_v и добавка на порывистость m 
//! в зависимости от района плавания судна, Табл. 2.1.4.1
use crate::data::structs::DataArray;
use serde::{Deserialize, Serialize};

/// Давление ветра p_v и добавка на порывистость m 
/// в зависимости от района плавания судна, Табл. 2.1.4.1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NavigationAreaData {
    /// Район плавания суджна
    pub area: String,
    /// Предполагаемое давление ветра
    pub p_v: f64,
    /// Добавка на порывистость ветра
    pub m: f64,
}
///
impl std::fmt::Display for NavigationAreaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NavigationAreaData(area:{}, p_v:{}, m:{} )",
            self.area, self.p_v, self.m,
        )
    }
}
///
pub type NavigationAreaArray = DataArray<NavigationAreaData>;
///
impl NavigationAreaArray {
    /// Условия района плавания
    pub fn get_area(&self, area_name: &str) -> Option<(f64, f64)> {
        self.data
            .iter()
            .filter(|data| area_name.eq_ignore_ascii_case(&data.area))
            .map(|data| (data.p_v, data.m))
            .next()
    }
}
