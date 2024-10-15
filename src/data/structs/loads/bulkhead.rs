//! Промежуточные структуры для serde_json для парсинга данных зерновой перегородки
use crate::data::structs::DataArray;
use serde::{Deserialize, Serialize};
use super::{CargoGeneralCategory, LoadCargo};
/// Зерновая перегородка
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bulkhead {
    /// Имя
    pub name: String,
    /// Общая масса, т
    pub mass: Option<f64>,
    /// Классификация
    pub general_category: CargoGeneralCategory,
    /// Диапазон по длинне, м
    pub bound_x1: f64,
    pub bound_x2: f64,
    /// Отстояние центра величины, м
    pub mass_shift_x: Option<f64>,
    pub mass_shift_y: Option<f64>,
    pub mass_shift_z: Option<f64>,
}
//
impl std::fmt::Display for Bulkhead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bulkhead(name:{} mass:{} loading_type:{} bound_x:({}, {}) mass_shift:({}, {}, {}) )",
            self.name,
            self.mass.unwrap_or(0.),
            self.general_category,
            self.bound_x1,
            self.bound_x2,
            self.mass_shift_x.unwrap_or(0.),
            self.mass_shift_y.unwrap_or(0.),
            self.mass_shift_z.unwrap_or(0.),
        )
    }
}
/// Массив данных по грузам
pub type BulkheadArray = DataArray<Bulkhead>;
//
impl BulkheadArray {
    //
    pub fn data(self) -> Vec<LoadCargo> {
        self.data
            .into_iter()
            .map(|v| LoadCargo {
                name: v.name,
                mass: v.mass,
                general_category: v.general_category,
                timber: false,
                bound_x1: v.bound_x1,
                bound_x2: v.bound_x2,
                bound_y1: None,
                bound_y2: None,
                bound_z1: None,
                bound_z2: None,
                mass_shift_x: v.mass_shift_x,
                mass_shift_y: v.mass_shift_y,
                mass_shift_z: v.mass_shift_z,
                horizontal_area: None,
                vertical_area: None,
                vertical_area_shift_x: None,
                vertical_area_shift_y: None,
                vertical_area_shift_z: None,
            })
            .collect()
    }
}
