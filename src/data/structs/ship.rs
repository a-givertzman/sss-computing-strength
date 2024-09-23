//! Промежуточные структуры для serde_json для парсинга данных судна
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{navigation_area_data::NavigationAreaData, serde_parser::IFromJson};

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
    /// Параметры района плавания судна
    pub navigation_area: NavigationAreaData,
    /// Тип надводного борта судна
    pub freeboard_type: String,
    /// Текст ошибки
    pub error: HashMap<String, String>,
}
///
impl IFromJson for Ship {
    ///
    fn error(&self) -> Option<&String> {
        self.error.values().next()
    }
}
///
impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ship(name:{}, ship_type:{}, icing_type:{}, icing_timber_type:{}, navigation_area:{}, freeboard_type:{})",
            self.name,
            self.ship_type,
            self.icing_type,
            self.icing_timber_type,
            self.navigation_area,
            self.freeboard_type,
        )
    }
}
