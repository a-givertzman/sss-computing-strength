//! Промежуточные структуры для serde_json для парсинга данных груза
use crate::data::structs::DataArray;
use serde::{Deserialize, Serialize};


/// Тип груза
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize,)]
pub enum CompartmentType {
    #[serde(alias="ballast")]
    Ballast,
    #[serde(alias="store")]
    Store,
    #[serde(alias="cargo")]
    Cargo,
}
///
impl std::fmt::Display for CompartmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CompartmentType::Ballast => "Ballast",
                CompartmentType::Store => "Store", 
                CompartmentType::Cargo => "Cargo", 
            },
        )
    }
}
/// Нагрузка судна: цистерны и трюмы  
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompartmentData {
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
    /// Тип груза
    pub loading_type: CompartmentType,
}
///
impl std::fmt::Display for CompartmentData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CompartmentData(space_id:{} name:{} mass:{} density:{} volume:{} bound:({}, {}) \
             mass_shift:({}, {}, {}) m_f_s_y:{} m_f_s_x:{} loading_type:{})",
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
            self.loading_type,
        )
    }
}
/// Массив данных по грузам
pub type CompartmentArray = DataArray<CompartmentData>;
///
impl CompartmentArray {
    ///
    pub fn data(self) -> Vec<CompartmentData> {
        self.data
    }
}
/// Груз
#[derive(Debug)]
pub struct ParsedCompartmentData {
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
    /// Тип груза
    pub loading_type: CompartmentType,
}
///
impl std::fmt::Display for ParsedCompartmentData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CompartmentData(name:{}, mass:{} bound_x:{:?}, bound_y:{:?} bound_z:{:?} 
                mass_shift:({} {} {}) m_f_s_y:{:?}, m_f_s_x:{:?} 
                windage_area:{} windage_shift:(x:{}, z:{}) type:{})",
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
            self.loading_type,
        )
    }
}
