use std::collections::HashMap;

use crate::data::structs::{ShipParametersArray, ShipData};

///
impl From<Vec<(&str, f64)>> for ShipParametersArray {
    fn from(src: Vec<(&str, f64)>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(key, value)| ShipData {
                    key: key.to_owned(),
                    value: value.to_owned(),
                })
                .collect(),
            error: HashMap::new(),
        }
    }
}
#[allow(dead_code)]
pub(crate) fn ship_parameters() -> ShipParametersArray {
    ShipParametersArray::from(vec![
        ("Wetting of deck timber", 10.),
        ("Ship operating speed", 16.),
        ("Water Density", 1.025),
        ("Keel area", 24.69),
        ("L.O.A", 119.95),
        ("LBP", 118.388),
        ("MouldedBreadth", 13.4),
        ("Moulded depth", 6.8),
        ("X midship from Fr0", 59.194),
        ("Number of Parts", 20.),
        ("Center of mass shift x", 1.05),
        ("Center of mass shift y", 0.),
        ("Center of mass shift z", 5.32),
        ("Total windage area for Dmin", 1037.90),
        ("Center of windage area shift x for Dmin", 3.98),
        ("Center of windage area shift z for Dmin", 6.22),
        ("Minimum draft", 1.40),
        ("Minimum operating draft", 3.05),
        ("Length middle from stern", 60.593),
        ("Overall height up to non-removable parts", 16.8),
        ("Draught corresponding to summer load line", 4.6),
        ("Calculated minimum bow height", 4.113),
        ("Calculated minimum bow area", 84.),
        ("Maximum aft trim", -1.7),
        ("Maximum forward trim", 0.7),
        ("DWT", 4886.4),
      ])
}
