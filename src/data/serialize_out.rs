use serde::{Deserialize, Serialize};

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutData {
    pub shear_force: Vec<(f64, f64)>,
    pub bending_moment: Vec<(f64, f64)>,
}


impl OutData {
    ///
    pub fn serialize(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }
}


