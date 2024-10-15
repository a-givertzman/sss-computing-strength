//! Давление ветра p_v и добавка на порывистость m
//! в зависимости от района плавания судна, Табл. 2.1.4.1
use serde::{Deserialize, Serialize};
use super::NavigationArea;
/// Давление ветра p_v и добавка на порывистость m 
/// в зависимости от района плавания судна, Табл. 2.1.4.1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NavigationAreaData {
    /// Район плавания судна
    pub area: NavigationArea,
    /// Предполагаемое давление ветра
    pub p_v: f64,
    /// Добавка на порывистость ветра
    pub m: f64,
}
//
impl std::fmt::Display for NavigationAreaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NavigationAreaData(area:{}, p_v:{}, m:{} )",
            self.area, self.p_v, self.m,
        )
    }
}
