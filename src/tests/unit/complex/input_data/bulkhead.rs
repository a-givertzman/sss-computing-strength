use std::collections::HashMap;

use crate::data::structs::loads::{Bulkhead, BulkheadArray, CargoGeneralCategory};

///
#[allow(dead_code)]
pub(crate) fn bulkhead() -> BulkheadArray {
    BulkheadArray {
        data: vec![
            Bulkhead {
                name: "Зерновая переборка №1".to_string(),
                mass: Some(12.),
                general_category: CargoGeneralCategory::Bulkhead,
                bound_x1: -42.749,
                bound_x2: -42.359,
                mass_shift_x: Some(-42.57),
                mass_shift_y: Some(0.),
                mass_shift_z: Some(4.53),
            },
            Bulkhead {
                name: "Зерновая переборка №2".to_string(),
                mass: Some(12.),
                general_category: CargoGeneralCategory::Bulkhead,
                bound_x1: -43.204,
                bound_x2: -42.814,
                mass_shift_x: Some(-42.96),
                mass_shift_y: Some(0.),
                mass_shift_z: Some(4.53),
            },
        ],
        error: HashMap::new(),
    }
}
