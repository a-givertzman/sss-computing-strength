//! Промежуточные структуры для serde_json для парсинга данных груза
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::DataArray;

/// Груз, конструкции корпуса, контейнер или другой твердый груз
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadSpaceData {
    /// ID груза
    pub space_id: usize,
    /// Имя груза
    pub name: String,
    /// Общая масса, т
    pub mass: Option<f64>,
    /// Плотность t/m^3
    pub density: Option<f64>,
    /// Объем m^3
    pub volume: Option<f64>,
    /// Диапазон по длинне
    pub bound_x1: f64,
    pub bound_x2: f64,
    /// Тип задания диапазона 
    /// (физ. шпангоуты или метры)
    pub bound_type: String,  
    /// Отстояние центра величины, м
    pub mass_shift_x: Option<f64>,
    pub mass_shift_y: Option<f64>,
    pub mass_shift_z: Option<f64>,
    /// Момент инерции площади ВЛ, м4
    pub m_f_s_y: Option<f64>,
    pub m_f_s_x: Option<f64>,
}

///
impl std::fmt::Display for LoadSpaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(space_id:{} name:{} mass:{} density:{} volume:{} bound_x1:{} bound_x2:{} mass_shift_x:{} mass_shift_y:{} mass_shift_z:{} m_f_s_y:{} m_f_s_x:{})",
            self.space_id,
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
        )
    }
}
/// Массив данных по грузам
pub type LoadSpaceArray = DataArray<LoadSpaceData>;
///
impl LoadSpaceArray {
    /// 
    pub fn data(self) -> Vec<LoadSpaceData> {
        self.data
    }
}

/// Груз
#[derive(Debug)]
pub struct ParsedLoadSpaceData {
    /// Название 
    pub name: String, 
    /// Общая масса, т
    pub mass: f64,
    /// Плотность 
    pub density: Option<f64>, 
    /// Объем m^3
    pub volume: Option<f64>,
    /// Границы груза
    pub bound_x: (f64, f64),
    pub bound_y: Option<(f64, f64)>,
    pub bound_z: Option<(f64, f64)>,
    /// Центр масс
    pub mass_shift: Option<(f64, f64, f64)>,
    /// Продольный момент свободной поверхности жидкости
    pub m_f_s_y: Option<f64>,
    /// Поперечный момент инерции свободной поверхности жидкости в цистерне
    pub m_f_s_x: Option<f64>,    
    /// Площадь парусности
    pub windage_area: Option<f64>,
    /// Центр парусности
    pub windage_shift: Option<(f64, f64)>,
}
///
impl std::fmt::Display for ParsedLoadSpaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(name:{}, mass:{} bound_x:{:?}, bound_y:{:?} bound_z:{:?} mass_shift:({} {} {}) m_f_s_y:{:?}, m_f_s_x:{:?} windage_area:{} windage_shift:(x:{}, z:{}))",
            self.name,
            self.mass,
            self.bound_x,
            self.bound_y,
            self.bound_z,
            self.mass_shift.unwrap_or((0.,0.,0.)).0,
            self.mass_shift.unwrap_or((0.,0.,0.)).1,
            self.mass_shift.unwrap_or((0.,0.,0.)).2,
            self.m_f_s_y,
            self.m_f_s_x,
            self.windage_area.unwrap_or(0.),
            self.windage_shift.unwrap_or((0.,0.)).0,
            self.windage_shift.unwrap_or((0.,0.)).1,
        )
    }
}