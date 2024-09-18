use std::collections::HashMap;

use crate::data::structs::loads::{CargoGeneralCategory, LoadCargo, LoadCargoArray};

///
#[allow(dead_code)]
pub(crate) fn bulkhead() -> LoadCargoArray {
    LoadCargoArray {
        data: vec![
            LoadCargo {
                name: "Зерновая переборка №1".to_string(),
                mass: Some(12.),
                general_category: CargoGeneralCategory::Bulkhead,
                timber: false,
                bound_x1: -42.749,
                bound_x2: -42.359,
                bound_y1: None,
                bound_y2: None,
                bound_z1: None,
                bound_z2: None,
                mass_shift_x: Some(-42.57),
                mass_shift_y: Some(0.),
                mass_shift_z: Some(4.53),
                horizontal_area: None,
                vertical_area: None,
                vertical_area_shift_x: None,
                vertical_area_shift_y: None,
                vertical_area_shift_z: None,
            },
            LoadCargo {
                name: "Зерновая переборка №2".to_string(),
                mass: Some(12.),
                general_category: CargoGeneralCategory::Bulkhead,
                timber: false,
                bound_x1: -43.204,
                bound_x2: -42.814,
                bound_y1: None,
                bound_y2: None,
                bound_z1: None,
                bound_z2: None,
                mass_shift_x: Some(-42.96),
                mass_shift_y: Some(0.),
                mass_shift_z: Some(4.53),
                horizontal_area: None,
                vertical_area: None,
                vertical_area_shift_x: None,
                vertical_area_shift_y: None,
                vertical_area_shift_z: None,
            },
        ],
        error: HashMap::new(),
    }
}
