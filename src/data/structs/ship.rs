//! Промежуточные структуры для serde_json для парсинга данных судна
use super::{DataArray, NavigationArea, NavigationAreaData};
use crate::Error;
use serde::{Deserialize, Serialize};
/// Общие по судну и расчету
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ship {
    /// Имя судна
    pub name: String,
    /// Тип судна
    pub ship_type: String,
    /// Тип облединения корпуса судна
    pub icing_type: String,
    /// Тип облединения палубного груза - леса
    pub icing_timber_type: String,
    /// Район плавания судна
    pub area: String,
    /// Предполагаемое давление ветра
    pub p_v: f64,
    /// Добавка на порывистость ветра
    pub m: f64,
    /// Тип надводного борта судна
    pub freeboard_type: String,
}
//
pub type ShipArray = DataArray<Ship>;
//
impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ship(name:{}, ship_type:{}, icing_type:{}, icing_timber_type:{}, area:{}, p_v:{}, m:{}, freeboard_type:{})",
            self.name,
            self.ship_type,
            self.icing_type,
            self.icing_timber_type,
            self.area,
            self.p_v,
            self.m,
            self.freeboard_type,
        )
    }
}
//
impl Ship {
    pub fn navigation_area(&self) -> Result<NavigationAreaData, Error> {
        Ok(NavigationAreaData {
            area: NavigationArea::from_str(&self.area)?,
            p_v: self.p_v,
            m: self.m,
        })
    }
}
