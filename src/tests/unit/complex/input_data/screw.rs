use std::collections::HashMap;
use crate::data::structs::{ScrewData, ScrewDataArray};
//
impl From<Vec<(&str, f64, f64, f64, f64)>> for ScrewDataArray {
    fn from(src: Vec<(&str, f64, f64, f64, f64)>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(name, x, y, z, d)| ScrewData {
                    name: name.to_owned(),
                    x,
                    y,
                    z,
                    d,
                })
                .collect(),
            error: HashMap::new(),
        }
    }
}
//
#[allow(dead_code)]
pub(crate) fn screw() -> ScrewDataArray {
    ScrewDataArray::from(vec![
        ("Screw SB", 0., 3.575, 1.72, 2.4),
        ("Screw PS", 0., -3.575, 1.72, 2.4),
    ])
}
