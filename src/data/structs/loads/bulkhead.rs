//! Промежуточные структуры для serde_json для парсинга данных зерновой перегородки
use serde::{Deserialize, Serialize};
use crate::data::structs::DataArray;

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

///
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
///
impl BulkheadArray {
    /// 
    pub fn data(self) -> Vec<LoadCargo> {
        self.data.into_iter().map(|v| LoadCargo{
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
        }).collect()
    }
}
/*
/// Груз
#[derive(Debug)]
pub struct ParsedCargoData {
    /// Название 
    pub name: String, 
    /// Общая масса, т
    pub mass: f64,
    /// Границы груза, м
    pub bound_x: (f64, f64),
    pub bound_y: Option<(f64, f64)>,
    pub bound_z: Option<(f64, f64)>,
    /// Центр масс, м
    pub mass_shift: Option<(f64, f64, f64)>, 
    /// Площадь горизонтальной поверхности, м^2
    pub horizontal_area: Option<f64>,
    /// Центр горизонтальной поверхности, м
    pub horizontal_area_shift: Option<(f64, f64, f64)>,
    /// Площадь парусности, м^2
    pub vertical_area: Option<f64>,
    /// Центр парусности, м
    pub vertical_area_shift: Option<(f64, f64, f64)>,
    /// Тип груза
    pub loading_type: CargoType,
}
///
impl std::fmt::Display for ParsedCargoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BulkheadData(name:{}, mass:{} bound_x:{:?}, bound_y:{:?} bound_z:{:?} mass_shift:({}, {}, {}) 
            horizontal_area:{} horizontal_area_shift:({}, {}, {})
            vertical_area:{} vertical_area_shift:({}, {}, {}) type:{}) ",
            self.name,
            self.mass,

            self.bound_x,
            self.bound_y,
            self.bound_z,
            self.mass_shift.unwrap_or((0.,0.,0.)).0,
            self.mass_shift.unwrap_or((0.,0.,0.)).1,
            self.mass_shift.unwrap_or((0.,0.,0.)).2,
            self.horizontal_area.unwrap_or(0.),
            self.horizontal_area_shift.unwrap_or((0.,0.,0.)).0,
            self.horizontal_area_shift.unwrap_or((0.,0.,0.)).1,
            self.horizontal_area_shift.unwrap_or((0.,0.,0.)).2,
            self.vertical_area.unwrap_or(0.),
            self.vertical_area_shift.unwrap_or((0.,0.,0.)).0,
            self.vertical_area_shift.unwrap_or((0.,0.,0.)).1,
            self.vertical_area_shift.unwrap_or((0.,0.,0.)).2,
            self.loading_type,
        )
    }
}
*/