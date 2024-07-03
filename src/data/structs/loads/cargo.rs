//! Промежуточные структуры для serde_json для парсинга данных груза
use serde::{Deserialize, Serialize};
use crate::{data::structs::DataArray};

/// Тип груза
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize,)]
pub enum CargoType {
    #[serde(alias="store")]
    Store,
    #[serde(alias="cargo")]
    Cargo,
}
///
impl std::fmt::Display for CargoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CargoType::Store => "Store", 
                CargoType::Cargo => "Cargo", 
            },
        )
    }
}
/// Груз, конструкции корпуса, контейнер или другой твердый груз
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadCargoData {
    /// Имя груза
    pub name: String,
    /// Общая масса, т
    pub mass: Option<f64>,
    /// Диапазон по длинне, м
    pub bound_x1: f64,
    pub bound_x2: f64,
    /// Диапазон по ширине
    pub bound_y1: Option<f64>,
    pub bound_y2: Option<f64>,
    /// Диапазон по высоте
    pub bound_z1: Option<f64>,
    pub bound_z2: Option<f64>,
    /// Отстояние центра величины, м
    pub mass_shift_x: Option<f64>,
    pub mass_shift_y: Option<f64>,
    pub mass_shift_z: Option<f64>,
    /// Площадь горизонтальной поверхности, м^2
    pub horizontal_area: Option<f64>,
    /// Смещение центра площади горизонтальной поверхности, м
    pub horizontal_area_shift_x: Option<f64>,
    pub horizontal_area_shift_y: Option<f64>,
    pub horizontal_area_shift_z: Option<f64>,
    /// Площадь вертикальной поверхности, м^2
    pub vertical_area: Option<f64>,
    /// Смещение центра площади вертикальной поверхности, м
    pub vertical_area_shift_x: Option<f64>,
    pub vertical_area_shift_y: Option<f64>,
    pub vertical_area_shift_z: Option<f64>,
    /// Тип груза
    pub loading_type: CargoType,
}

///
impl std::fmt::Display for LoadCargoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadCargoData(name:{} mass:{} bound_x:({}, {}) bound_y:({}, {}) bound_z:({}, {}) 
            mass_shift:({}, {}, {}) horizontal_area:{} horizontal_area_shift:({}, {}, {})
            vertical_area:{} vertical_area_shift_y:({}, {}, {}) loading_type:{})",
            self.name,
            self.mass.unwrap_or(0.),
            self.bound_x1,
            self.bound_x2,
            self.bound_y1.unwrap_or(0.),
            self.bound_y2.unwrap_or(0.),
            self.bound_z1.unwrap_or(0.),
            self.bound_z2.unwrap_or(0.),
            self.mass_shift_x.unwrap_or(0.),
            self.mass_shift_y.unwrap_or(0.),
            self.mass_shift_z.unwrap_or(0.),
            self.horizontal_area.unwrap_or(0.),
            self.horizontal_area_shift_x.unwrap_or(0.),
            self.horizontal_area_shift_y.unwrap_or(0.),
            self.horizontal_area_shift_z.unwrap_or(0.),
            self.vertical_area.unwrap_or(0.),
            self.vertical_area_shift_x.unwrap_or(0.),
            self.vertical_area_shift_y.unwrap_or(0.),
            self.vertical_area_shift_z.unwrap_or(0.),
            self.loading_type,
        )
    }
}
/// Массив данных по грузам
pub type LoadCargoArray = DataArray<LoadCargoData>;
///
impl LoadCargoArray {
    /// 
    pub fn data(self) -> Vec<LoadCargoData> {
        self.data
    }
}

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
            "LoadCargoData(name:{}, mass:{} bound_x:{:?}, bound_y:{:?} bound_z:{:?} mass_shift:({}, {}, {}) 
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