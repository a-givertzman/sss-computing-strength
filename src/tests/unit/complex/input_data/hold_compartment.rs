//!  Груз в трюмах судна

use std::collections::HashMap;

use crate::data::structs::loads::{CargoGeneralCategory, CompartmentArray, CompartmentData, MatterType};

/// Пустые трюмы
pub(crate) fn hold_compartment_empty() -> CompartmentArray {
    CompartmentArray {
        data: Vec::new(),
        error: HashMap::new(),
    }
}
/// Груз зерна
pub(crate) fn hold_compartment_grain() -> CompartmentArray {
    CompartmentArray {
        data: vec![
            CompartmentData {
                name: "Трюм 51-151 шп.".to_string(),
                mass: Some(3938.696),
                density: Some(0.8),
                volume: Some(4923.37),
                bound_x1: -27.344,
                bound_x2: 37.656,
                mass_shift_x: Some(4.925048030109457),
                mass_shift_y: Some(0.),
                mass_shift_z: Some(4.515987880252754),
                m_f_s_y: None,
                m_f_s_x: None,
                grain_moment: Some(1320.1),
                general_category: CargoGeneralCategory::Cargo,
                matter_type: MatterType::Bulk,
            },
            CompartmentData {
                name: "Трюм 25-51 шп.".to_string(),
                mass: Some(472.8),
                density: Some(0.8),
                volume: Some(591.),
                bound_x1: -44.194,
                bound_x2: -27.344,
                mass_shift_x: Some(-35.829),
                mass_shift_y: Some(-0.003),
                mass_shift_z: Some(2.512672413793103),
                m_f_s_y: None,
                m_f_s_x: None,
                grain_moment: Some(1354.2499310344826),
                general_category: CargoGeneralCategory::Cargo,
                matter_type: MatterType::Bulk,
            },
        ],
        error: HashMap::new(),
    }
}

/*
pub(crate) fn hold_compartment_grain() -> CompartmentArray {
    CompartmentArray {
        data: vec![
            CompartmentData {
                name: "Трюм 113-151 шп.".to_string(),
                mass: Some(1479.),
                density: Some(0.8),
                volume: Some(1848.75),
                bound_x1: 12.956,
                bound_x2: 37.656,
                mass_shift_x: Some(25.08),
                mass_shift_y: Some(0.),
                mass_shift_z: Some(4.501),
                m_f_s_y: None,
                m_f_s_x: None,
                grain_moment: Some(389.2),
                general_category: CargoGeneralCategory::Cargo,
                matter_type: MatterType::Bulk,
            },
            CompartmentData {
                name: "Трюм 51-113 шп.".to_string(),
                mass: Some(2459.696),
                density: Some(0.8),
                volume: Some(3074.62),
                bound_x1: -27.344,
                bound_x2: 12.956,
                mass_shift_x: Some(-7.194),
                mass_shift_y: Some(0.),
                mass_shift_z: Some(4.525),
                m_f_s_y: None,
                m_f_s_x: None,
                grain_moment: Some(930.9),
                general_category: CargoGeneralCategory::Cargo,
                matter_type: MatterType::Bulk,
            },
            CompartmentData {
                name: "Трюм 25-51 шп.".to_string(),
                mass: Some(472.8),
                density: Some(0.8),
                volume: Some(591.),
                bound_x1: -44.194,
                bound_x2: -27.344,
                mass_shift_x: Some(-35.829),
                mass_shift_y: Some(-0.003),
                mass_shift_z: Some(2.512672413793103),
                m_f_s_y: None,
                m_f_s_x: None,
                grain_moment: Some(1354.2499310344826),
                general_category: CargoGeneralCategory::Cargo,
                matter_type: MatterType::Bulk,
            },
        ],
        error: HashMap::new(),
    }
}
*/