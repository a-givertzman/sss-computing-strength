//! Структуры для ввода данных
use std::collections::HashSet;

use serde::{de::Error, de::Unexpected, Deserialize, Serialize};

pub type Result<T> = serde_json::Result<T>;

/// Данные по судну
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedShipData {
    /// длинна корпуса судна
    pub ship_length: f64,
    /// разбиение на шпации - количество
    pub n_parts: f64,
    /// плотность воды
    pub water_density: f64,
}
///
#[allow(dead_code)]
impl ParsedShipData {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: Self = serde_json::from_str(src)?;
        if result.ship_length <= 0. {
            return Err(Error::invalid_value(
                Unexpected::Float(result.ship_length),
                &"positive value of ship's length",
            ));
        }
        if result.n_parts == 0. {
            return Err(Error::invalid_value(
                Unexpected::Float(result.n_parts),
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

/// Кривая отстояния центра тяжести ватерлинии по длине от миделя  
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedCenterWaterline {
    /// Кривая отстояния центра тяжести ватерлинии по длине от миделя  
    pub center_waterline: Vec<(f64, f64)>,
}
///
#[allow(dead_code)]
impl ParsedCenterWaterline {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: Self = serde_json::from_str(src)?;
        if result.center_waterline.len() <= 1 {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(result.center_waterline.len() as u64),
                &"number of waterline's points greater or equal to 2",
            ));
        }
        Ok(result)
    }
}
/// Кривая продольного метацентрического радиуса
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedRadLong {
    /// Кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f64, f64)>,
}
///
#[allow(dead_code)]
impl ParsedRadLong {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: Self = serde_json::from_str(src)?;
        if result.rad_long.len() <= 1 {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(result.rad_long.len() as u64),
                &"number of rad_long's points greater or equal to 2",
            ));
        }
        Ok(result)
    }
}
/// Кривая средней осадки
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedMeanDraught {
    /// Кривая средней осадки
    pub mean_draught: Vec<(f64, f64)>,
}
///
#[allow(dead_code)]
impl ParsedMeanDraught {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: Self = serde_json::from_str(src)?;
        if result.mean_draught.len() <= 1 {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(result.mean_draught.len() as u64),
                &"number of mean_draught's points greater or equal to 2",
            ));
        }
        Ok(result)
    }
}
/// Кривая отстояния центра величины погруженной части судна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedCenterShift {
    /// кривая отстояния центра величины погруженной части судна
    pub center_shift: Vec<(f64, f64, f64, f64)>,
}
///
#[allow(dead_code)]
impl ParsedCenterShift {
    ///
    pub fn parse(src: &str) -> Result<Self> {
        let result: Self = serde_json::from_str(src)?;
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
        if let Some(frame) = result.frames.iter().find(|f| f.index >= result.frames.len()) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(frame.index as u64),
                &"index of frame lower then frames.len()",
            ));
        }
        let qnt_unique_index = result.frames.iter().map(|f| f.index ).collect::<HashSet<_>>().len();
        if result.frames.len() != qnt_unique_index {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(qnt_unique_index as u64),
                &"index of frame must be unique",
            ));
        }
        if let Some(frame) = result.frames.iter().find(|f| f.immersion_area.len() <= 1) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(frame.immersion_area.len() as u64),
                &"number of immersion_area's points greater or equal to 2",
            ));
        }
        Ok(result)
    }
}
/// Груз
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadSpaceData {
    /// Общая масса
    pub mass: f64,
    /// Границы груза
    pub bound: (f64, f64, f64, f64),
    /// Центер масс 
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
        if let Some(s) = result.load_space.iter().find(|s| s.bound.0 >= s.bound.1 || s.bound.2 >= s.bound.3 ) {
            return Err(Error::invalid_value(
                Unexpected::Other(&format!("Bound({}, {}, {}, {})", s.bound.0, s.bound.1, s.bound.2, s.bound.3)),
                &"Bound: x1 < x2, y1 < y2",
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
        if let Some(t) = result.tanks.iter().find(|t| t.bound.0 >= t.bound.1 || t.bound.2 >= t.bound.3 ) {
            return Err(Error::invalid_value(
                Unexpected::Other(&format!("Bound({}, {}, {}, {})", t.bound.0, t.bound.1, t.bound.2, t.bound.3)),
                &"wrong data for Bound",
            ));
        }
        if let Some(tank) = result.tanks.iter().find(|t| t.center.len() <= 1) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(tank.center.len() as u64),
                &"number of center's points greater or equal to 2",
            ));
        }
        if let Some(tank) = result.tanks.iter().find(|t| t.free_surf_inertia.len() <= 1) {
            return Err(Error::invalid_value(
                Unexpected::Unsigned(tank.free_surf_inertia.len() as u64),
                &"number of free_surf_inertia's points greater or equal to 2",
            ));
        }
        Ok(result)
    }
}


/*
/// Общая структура для ввода данных. Содержит все данные
/// для расчетов, при заполнении проверяет данные на корректность.
#[derive(Debug)]
pub struct ShipData {
    /// разбиение на шпации - количество
    pub n_parts: u32,
    /// плотность воды
    pub water_density: f32,
    /// длинна корпуса судна
    pub ship_length: f32,
    /// кривая отстояния центра тяжести ватерлинии по длине от миделя  
    pub center_waterline: Vec<(f32, f32)>,
    /// кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f32, f32)>,
    /// кривая средней осадки
    pub mean_draught: Vec<(f32, f32)>,
    /// кривая отстояния центра величины погруженной части судна
    pub center_shift: Vec<(f32, f32, f32, f32)>,
    /// Шпангоуты судна
    pub frames: Vec<super::frame::ShipData>,
    /// Нагрузка судна без жидких грузов
    pub load_space: Vec<super::load_space::LoadSpaceData>,
    /// Нагрузка судна, жидкие грузы
    pub tanks: Vec<super::tank::TankData>,
}
*/