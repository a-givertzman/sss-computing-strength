//! Структуры для ввода данных
use serde::{de::Error, de::Unexpected, Deserialize, Serialize};

pub type Result<T> = serde_json::Result<T>;

/// Данные запроса на расчет
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedInputData {
    /// название проекта судна
    pub project_name: String,
    /// имя судна
    pub ship_name: String,
    /// разбиение на шпации - количество
    pub n_parts: u64,
    /// плотность воды
    pub water_density: f64,
}
///
#[allow(dead_code)]
impl ParsedInputData {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: ParsedInputData = serde_json::from_str(src)?;
        if result.project_name.len() == 0 {
            return Err(Error::invalid_value(
                Unexpected::Str(&result.project_name),
                &"project_name",
            ));
        }
        if result.ship_name.len() == 0 {
            return Err(Error::invalid_value(
                Unexpected::Str(&result.ship_name),
                &"ship_name",
            ));
        }
        if result.n_parts == 0 {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(result.n_parts),
                &"positive number of frames",
            ));
        }
        if result.water_density <= 0. {
            return Err(Error::invalid_value(
                Unexpected::Float(result.water_density),
                &"positive value of water density",
            ));
        }
        Ok(result)
    }
}

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
    pub center_shift: Vec<(f64, f64, f64, f64)>,
}
///
#[allow(dead_code)]
impl ParsedShipData {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: ParsedShipData = serde_json::from_str(src)?;
        if result.ship_length <= 0. {
            return Err(Error::invalid_value(
                Unexpected::Float(result.ship_length),
                &"positive value of ship's length",
            ));
        }
        if result.center_waterline.len() <= 1 {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(result.center_waterline.len() as u64),
                &"number of waterline's points greater or equal to 2",
            ));
        }
        if result.mean_draught.len() <= 1 {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(result.mean_draught.len() as u64),
                &"number of mean_draught's points greater or equal to 2",
            ));
        }
        if result.center_shift.len() <= 1 {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(result.center_shift.len() as u64),
                &"number of center_shift's points greater or equal to 2",
            ));
        }
        Ok(result)
    }
}
/// Шпангоут
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameData {
    /// порядковый номер шпангоута от кормы
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
///
#[allow(dead_code)]
impl ParsedFramesData {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: ParsedFramesData = serde_json::from_str(src)?;
        if result.frames.len() <= 1 {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(result.frames.len() as u64),
                &"number of frames greater or equal to 2",
            ));
        }
        if let Some(frame) = result.frames.iter().find(|f| f.index > result.frames.len()) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(frame.index as u64),
                &"index of frame lower or equal frames.len()",
            ));
        }
        if let Some(frame) = result.frames.iter().find(|f| f.immersion_area.len() == 0) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(frame.immersion_area.len() as u64),
                &"number of immersion_area's points greater to 0",
            ));
        }
        Ok(result)
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
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedLoadsData {
    pub load_space: Vec<LoadSpaceData>,
}
///
#[allow(dead_code)]
impl ParsedLoadsData {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: ParsedLoadsData = serde_json::from_str(src)?;
        if let Some(space) = result.load_space.iter().find(|s| s.mass < 0.) {
            return Err(Error::invalid_value(
                Unexpected::Float(space.mass),
                &"mass of load_space greater or equal to 0",
            ));
        }
        Ok(result)
    }
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
pub struct ParsedTanksData {
    pub tanks: Vec<TankData>,
}
///
#[allow(dead_code)]
impl ParsedTanksData {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: ParsedTanksData = serde_json::from_str(src)?;
        if let Some(tank) = result.tanks.iter().find(|t| t.density <= 0.) {
            return Err(Error::invalid_value(
                Unexpected::Float(tank.density),
                &"density of liquid in the tank greater to 0",
            ));
        }
        if let Some(tank) = result.tanks.iter().find(|t| t.volume < 0.) {
            return Err(Error::invalid_value(
                Unexpected::Float(tank.volume),
                &"volume of tank greater or equal to 0",
            ));
        }
        if let Some(tank) = result.tanks.iter().find(|t| t.center.len() == 0) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(tank.center.len() as u64),
                &"number of center's points greater to 0",
            ));
        }
        if let Some(tank) = result.tanks.iter().find(|t| t.free_surf_inertia.len() == 0) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(tank.free_surf_inertia.len() as u64),
                &"number of free_surf_inertia's points greater to 0",
            ));
        }
        Ok(result)
    }
}
