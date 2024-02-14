use serde::{Deserialize, Serialize};
use serde_json::Result;

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameData {
    offset_x: f64,
    immersion_area_curve: Vec<(f64, f64)>,
}

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameSpaceData {
    mass: f64,
    displacement_curve: Vec<(f64, f64)>,
}

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedData {
    gravity_const_g: f64,
    water_density: f64,
    mean_draught_curve: Vec<(f64, f64)>,
    trimming_moment_curve: Vec<(f64, f64)>,
    buoyancy_centre_curve: Vec<(f64, f64)>,
    frames: Vec<FrameData>,
    frame_spaces: Vec<FrameSpaceData>,
}


impl ParsedData {
    ///
    pub fn parse(src: &str) -> Option<Self> {
        serde_json::from_str(src).ok()?
    }
}


