//! Промежуточные структуры для serde_json для парсинга данных контейнеров
use super::{CargoGeneralCategory, LoadCargo};
use crate::data::structs::DataArray;
use serde::{Deserialize, Serialize};
/// Контейнеры, груз без привязки к помещению, всегда твердый
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Container {
    /// ИД груза
    pub id: i32,
    /// Общая масса, т
    pub mass: f64,
    /// Классификация груза
    pub general_category: CargoGeneralCategory,
    /// Диапазон по длинне, м
    pub bound_x1: f64,
    pub bound_x2: f64,
    /// Диапазон по ширине
    pub bound_y1: f64,
    pub bound_y2: f64,
    /// Диапазон по высоте
    pub bound_z1: f64,
    pub bound_z2: f64,
    /// Отстояние центра величины, м
    pub mass_shift_x: f64,
    pub mass_shift_y: f64,
    pub mass_shift_z: f64,
    /// Признак нахождения контейнера на палубе
    pub is_on_deck: bool,
}
//
impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Container(id:{} mass:{} general_category:{} 
            bound_x:({}, {}) bound_y:({}, {}) bound_z:({}, {}) is_on_deck:{} )",
            self.id,
            self.mass,
            self.general_category,
            self.bound_x1,
            self.bound_x2,
            self.bound_y1,
            self.bound_y2,
            self.bound_z1,
            self.bound_z2,
            self.is_on_deck,
        )
    }
}
/// Массив данных по грузам
pub type ContainerArray = DataArray<Container>;
//
impl ContainerArray {
    //
    pub fn data(self) -> Vec<LoadCargo> {
        self.data
            .into_iter()
            .map(|v| {
                let (
                    horizontal_area,
                    vertical_area,
                    vertical_area_shift_x,
                    vertical_area_shift_y,
                    vertical_area_shift_z,
                ) = if v.is_on_deck {
                    (
                        Some((v.bound_x2 - v.bound_x1) * (v.bound_y2 - v.bound_y1)),
                        Some((v.bound_x2 - v.bound_x1) * (v.bound_z2 - v.bound_z1)),
                        Some((v.bound_x1 + v.bound_x2) / 2.),
                        Some((v.bound_y1 + v.bound_y2) / 2.),
                        Some((v.bound_z1 + v.bound_z2) / 2.),
                    )
                } else {
                    (None, None, None, None, None)
                };
                LoadCargo {
                    name: format!("container_{}", v.id),
                    mass: Some(v.mass),
                    general_category: v.general_category,
                    timber: false,
                    bound_x1: v.bound_x1,
                    bound_x2: v.bound_x2,
                    bound_y1: Some(v.bound_y1),
                    bound_y2: Some(v.bound_y2),
                    bound_z1: Some(v.bound_z1),
                    bound_z2: Some(v.bound_z2),
                    mass_shift_x: Some(v.mass_shift_x),
                    mass_shift_y: Some(v.mass_shift_y),
                    mass_shift_z: Some(v.mass_shift_z),
                    horizontal_area,
                    vertical_area,
                    vertical_area_shift_x,
                    vertical_area_shift_y,
                    vertical_area_shift_z,
                }
            })
            .collect()
    }
}
