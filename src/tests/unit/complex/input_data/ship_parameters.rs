use std::collections::HashMap;

use crate::data::structs::{ShipArray, ShipData};

///
impl From<Vec<(&str, &str, &str, &str)>> for ShipArray {
    fn from(src: Vec<(&str, &str, &str, &str)>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(key, value, value_type, _)| ShipData {
                    key: key.to_owned(),
                    value: value.to_owned(),
                    value_type: value_type.to_owned(),
                })
                .collect(),
            error: HashMap::new(),
        }
    }
}
#[allow(dead_code)]
pub(crate) fn ship_parameters() -> ShipArray {
    ShipArray::from(vec![
        ("Name of ship", "Belogorodskaya ARK-20231", "text", ""),
        ("Type of icing", "none", "text", ""),
        ("Type of icing timber", "full", "text", ""),
        ("Wetting of deck timber", "10", "real", ""),
        ("Navigation area", "R2", "text", ""),
        ("Type of ship", "general dry cargo ship", "text", ""),
        ("Ship operating speed", "16", "real", "m/s"),
        ("Water Density", "1.025", "real", "g/ml"),
        ("Keel area", "24.69", "real", "m^2"),
        ("L.O.A", "119.95", "real", "m"),
        ("LBP", "118.388", "real", "m"),
        ("MouldedBreadth", "13.4", "real", "m"),
        ("Moulded depth", "6.8", "real", "m"),
        ("X midship from Fr0", "59.194", "real", "m"),
        ("Number of Parts", "20", "int", ""),
        ("Center of mass shift x", "1.05", "real", "m"),
        ("Center of mass shift y", "0", "real", "m"),
        ("Center of mass shift z", "5.32", "real", "m"),
        ("Total windage area for Dmin", "1037.90", "real", "m^2"),
        ("Center of windage area shift x for Dmin", "3.98", "real", "m"),
        ("Center of windage area shift z for Dmin", "6.22", "real", "m"),
        ("Minimum draft", "1.40", "real", "m"),
        ("Minimum operating draft", "3.05", "real", "m"),
        ("Length middle from stern", "60.593", "real", "m"),
        ("Overall height up to non-removable parts", "16.8", "real", "m"),
        ("Draught corresponding to summer load line", "4.6", "real", "m"),
        ("Calculated minimum bow height", "4.113", "real", "m"),
        ("freeboardType", "B", "text", ""),
        ("Calculated minimum bow area", "84", "real", "m^2"),
        ("Minimum allowable trim", "-1.7", "real", "m"),
        ("Maximum allowable trim", "0.7", "real", "m"),
      ])
}
