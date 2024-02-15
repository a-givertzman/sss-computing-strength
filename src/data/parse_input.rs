use serde::{Deserialize, Serialize};

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameData {
    pub offset_x: f64,
    pub immersion_area_curve: Vec<(f64, f64)>,
}

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameSpaceData {
    pub mass: f64,
    pub displacement_curve: Vec<(f64, f64)>,
}

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedData {
    pub gravity_const_g: f64,
    pub water_density: f64,
    pub mean_draught_curve: Vec<(f64, f64)>,
    pub trimming_moment_curve: Vec<(f64, f64)>,
    pub buoyancy_centre_curve: Vec<(f64, f64)>,
    pub frames: Vec<FrameData>,
    pub frame_spaces: Vec<FrameSpaceData>,
}


impl ParsedData {
    ///
    pub fn parse(src: &str) -> Option<Self> {
        serde_json::from_str(src).ok()?
    }
}


