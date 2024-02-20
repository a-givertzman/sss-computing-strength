use serde::{Deserialize, Serialize};

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedShipData {
    pub ship_length: f64,
    pub center_waterline: Vec<(f64, f64)>,
    pub rad_trans: Vec<(f64, f64)>,
    pub mean_draught: Vec<(f64, f64)>,
    pub center_shift: Vec<(f64, f64, f64, f64,)>,
}
///
impl ParsedShipData {
    ///
    pub fn parse(src: &str) -> Option<Self> {
        serde_json::from_str(src).ok()?
    }
}

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameData {
    pub index: usize,
    pub immersion_area: Vec<(f64, f64)>,
}

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedFramesData {
    pub frames: Vec<FrameData>,
}

impl ParsedFramesData {
    ///
    pub fn parse(src: &str) -> Option<Self> {
        serde_json::from_str(src).ok()?
    }
}


///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadSpaceData {
    pub mass: f64,
    pub bound: (f64, f64, f64, f64),
    pub center: (f64, f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TankData {
    pub density: f64,
    pub volume: f64,
    pub bound: (f64, f64, f64, f64),
    pub center: Vec<(f64, f64, f64, f64)>,
    pub free_surf_inertia: Vec<(f64, f64, f64)>,
}

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedLoadsData {
    pub load_space: Vec<LoadSpaceData>,
    pub tanks: Vec<TankData>,
}

impl ParsedLoadsData {
    ///
    pub fn parse(src: &str) -> Option<Self> {
        serde_json::from_str(src).ok()?
    }
}
