use std::collections::HashMap;

use crate::data::structs::loads::{CargoGeneralCategory, LoadCargo, LoadCargoArray};
///
impl From<Vec<(&str, f64, f64, f64, f64)>> for LoadCargoArray {
    fn from(src: Vec<(&str, f64, f64, f64, f64)>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(name, value, shift_z, bound_x1, bound_x2)| LoadCargo {
                    name: name.to_owned(),
                    mass: Option<f64>,
                    general_category: CargoGeneralCategory,
                    timber: bool,
                    bound_x1: f64,
                    bound_x2: f64,
                    bound_y1: Option<f64>,
                    bound_y2: Option<f64>,
                    bound_z1: Option<f64>,
                    bound_z2: Option<f64>,
                    mass_shift_x: Option<f64>,
                    mass_shift_y: Option<f64>,
                    mass_shift_z: Option<f64>,
                    horizontal_area: Option<f64>,
                    vertical_area: Option<f64>,
                    vertical_area_shift_x: Option<f64>,
                    vertical_area_shift_y: Option<f64>,
                    vertical_area_shift_z: Option<f64>,
                })
                .collect(),
            error: HashMap::new(),
        }
    }
}
///
#[allow(dead_code)]
pub(crate) fn cargo() -> LoadCargoArray {
    LoadCargoArray::from(vec![
        (0.0, 1.0),
        (1.0, 0.98),
        (1.5, 0.95),
        (2.0, 0.88),
        (2.5, 0.79),
        (3.0, 0.74),
        (3.5, 0.72),
        (4.0, 0.70),
    ])
}