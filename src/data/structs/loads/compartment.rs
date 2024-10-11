//! Промежуточные структуры для serde_json для парсинга данных груза
use crate::data::structs::DataArray;
use serde::{Deserialize, Serialize};
use super::{CargoGeneralCategory, MatterType};
/// Помещения судна: цистерны и трюмы  
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompartmentData {
    /// Имя груза
    pub name: String,
    /// Общая масса, т
    pub mass: Option<f64>,
    /// Плотность t/m^3
    pub density: Option<f64>,
    /// Объем m^3
    pub volume: Option<f64>,
    /// Диапазон по длинне, м
    pub bound_x1: f64,
    pub bound_x2: f64,
    /// Отстояние центра величины, м
    pub mass_shift_x: Option<f64>,
    pub mass_shift_y: Option<f64>,
    pub mass_shift_z: Option<f64>,
    /// Момент инерции площади ВЛ, м4
    pub m_f_s_y: Option<f64>,
    pub m_f_s_x: Option<f64>,
    /// Кренящий момент от смещения сыпучего груза, м4
    pub grain_moment: Option<f64>,
    /// Классификация груза
    pub general_category: CargoGeneralCategory,
    /// Физический тип груза судна
    pub matter_type: MatterType,
}
//
impl std::fmt::Display for CompartmentData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CompartmentData(name:{} mass:{} density:{} volume:{} bound:({}, {}) \
             mass_shift:({}, {}, {}) m_f_s_y:{} m_f_s_x:{} grain_moment:{} general_category:{} matter_type:{})",
            self.name,
            self.mass.unwrap_or(0.),
            self.density.unwrap_or(0.),
            self.volume.unwrap_or(0.),
            self.bound_x1,
            self.bound_x2,
            self.mass_shift_x.unwrap_or(0.),
            self.mass_shift_y.unwrap_or(0.),
            self.mass_shift_z.unwrap_or(0.),
            self.m_f_s_y.unwrap_or(0.),
            self.m_f_s_x.unwrap_or(0.),
            self.grain_moment.unwrap_or(0.),
            self.general_category,
            self.matter_type,
        )
    }
}
/// Массив данных по грузам
pub type CompartmentArray = DataArray<CompartmentData>;
//
impl CompartmentArray {
    pub fn data(self) -> Vec<CompartmentData> {
        self.data
    }
}
