//! Структуры для ввода данных
use serde::{Deserialize, Serialize};

/// Данные по корпусу судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedShipData {
    /// длинна корпуса судна
    pub ship_length: f64,
    /// кривая отстояния центра тяжести ватерлинии по длине от миделя  
    pub center_waterline: Vec<(f64, f64)>,
    /// кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f64, f64)>,
    /// кривая средней осадки
    pub mean_draught: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна
    pub center_shift: Vec<(f64, f64, f64, f64,)>,
}
///
#[allow(dead_code)]
impl ParsedShipData {
    ///
    pub fn parse(src: &str) -> Option<Self> {
        serde_json::from_str(src).ok()?
    }
}

/// Шпангоут
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameData {
    /// порядковый номер шпангоута
    pub index: usize,
    /// кривая погружаемой площади
    pub immersion_area: Vec<(f64, f64)>,
}

/// Шпангоуты судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedFramesData {
    /// массив шпангоутов судна
    pub frames: Vec<FrameData>,
}

#[allow(dead_code)]
impl ParsedFramesData {
    ///
    pub fn parse(src: &str) -> Option<Self> {
        serde_json::from_str(src).ok()?
    }
}


/// Груз
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadSpaceData {
    /// общая масса
    pub mass: f64,
    /// границы груза
    pub bound: (f64, f64, f64, f64),
    /// центер масс
    pub center: (f64, f64, f64),
}
/// Цистерна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TankData {
    /// плотность жидкости в цистерне 
    pub density: f64,
    /// объем жидкости в цистерне
    pub volume: f64,
    /// границы цистерны, (x1, x2, y1, y2)
    pub bound: (f64, f64, f64, f64),
    /// кривая координат центра объема жидкости в цистерне в системе координат судна
    /// (volume, x, y, z)
    pub center: Vec<(f64, f64, f64, f64)>,
    /// кривая момента инерции площади свободной поверхности жидкости
    /// (volume, x - поперечный, y - продольный)
    pub free_surf_inertia: Vec<(f64, f64, f64)>,
}

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedLoadsData {
    pub load_space: Vec<LoadSpaceData>,
}

#[allow(dead_code)]
impl ParsedLoadsData {
    ///
    pub fn parse(src: &str) -> Option<Self> {
        serde_json::from_str(src).ok()?
    }
}

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedTanksData {
    pub tanks: Vec<TankData>,
}

#[allow(dead_code)]
impl ParsedTanksData {
    ///
    pub fn parse(src: &str) -> Option<Self> {
        serde_json::from_str(src).ok()?
    }
}
