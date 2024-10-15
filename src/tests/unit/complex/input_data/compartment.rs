use std::collections::HashMap;
use crate::data::structs::loads::{
    CargoGeneralCategory, CompartmentArray, CompartmentData, MatterType,
};
//
impl
    From<
        Vec<(
            &str,
            f64,
            MatterType,
            CargoGeneralCategory,
            Option<f64>,
            Option<f64>,
            f64,
            f64,
            f64,
            f64,
            f64,
            Option<f64>,
            Option<f64>,
        )>,
    > for CompartmentArray
{
    fn from(
        src: Vec<(
            &str,
            f64,
            MatterType,
            CargoGeneralCategory,
            Option<f64>,
            Option<f64>,
            f64,
            f64,
            f64,
            f64,
            f64,
            Option<f64>,
            Option<f64>,
        )>,
    ) -> Self {
        Self {
            data: src
                .into_iter()
                .map(
                    |(
                        name,
                        mass,
                        matter_type,
                        general_category,
                        density,
                        volume,
                        bound_x1,
                        bound_x2,
                        mass_shift_x,
                        mass_shift_y,
                        mass_shift_z,
                        m_f_s_y,
                        m_f_s_x,
                    )| CompartmentData {
                        name: name.to_owned(),
                        mass: Some(mass),
                        density,
                        volume,
                        bound_x1,
                        bound_x2,
                        mass_shift_x: Some(mass_shift_x),
                        mass_shift_y: Some(mass_shift_y),
                        mass_shift_z: Some(mass_shift_z),
                        m_f_s_y,
                        m_f_s_x,
                        grain_moment: None,
                        general_category,
                        matter_type,                   
                    },
                )
                .collect(),
            error: HashMap::new(),
        }
    }
}
//
#[allow(dead_code)]
pub fn compartment_100_sea() -> CompartmentArray {                                                                                                                                                                                                         
    CompartmentArray::from(vec![                                                                                                                                                                                                                
    //      name,                                     mass,     matter_type,        general_category,               density,volume,         bound_x1, bound_x2, mass_shift_x,mass_shift_y,mass_shift_z,m_f_s_y,  m_f_s_x,     grain_moment
            ("Цистерна питьевой воды 2 ЛБ",             3.71,     MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.),   Some(3.71),          -24.744,   42.556,  41.44,     -1.325,       5.02,          Some(0.),             Some(0.),          ),
            ("Цистерна расходного топлива 1 ЛБ",        5.504,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(6.4),           -55.594,  -53.194,  -54.394,   -1.725,       5.299,         Some(0.4),            Some(1.4),         ),
            ("Цистерна расходного топлива 2 ЛБ",        5.504,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(6.4),           -55.594,  -53.194,  -54.394,   -2.975,       5.299,         Some(0.4),            Some(1.4),         ),
            ("Цистерна запасного топлива 1 Пр.Б",       34.9762,  MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(40.67),         -45.994,  -44.194,  -45.146,    2.802,       3.835,         Some(0.),             Some(0.),          ),
            ("Цистерна запасного топлива 2 ЛБ",         35.0966,  MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(40.81),         -45.994,  -44.194,  -45.248,   -2.796,       3.835,         Some(0.),             Some(0.),          ),
            ("Цистерна запасного топлива 3 Пр.Б",       53.42,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(62.11627907),   -57.394,  -53.194,  -55.012,    3.265875969, 2.994,         Some(12.66356589146), Some(4.973643411), ),
            ("Цистерна запасного топлива 3 ЛБ",         53.42,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(62.11627907),   -57.394,  -53.194,  -55.012,   -3.265875969, 2.994,         Some(12.66356589146), Some(4.973643411), ),
            ("Цистерна расходного топлива АДГ Пр.Б",    1.0578,   MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(1.23),           39.556,   40.756,   39.965,    2.022,       11.552,        Some(0.),             Some(0.1),         ),
            ("Цистерна пресной воды 1 ДП (носовая)",    45.09,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.),   Some(45.09),          41.356,   43.756,   42.556,   -1.25,        2.31,          Some(0.),             Some(0.),          ),
            ("Цистерна пресной воды 2 Пр.Б (кормовая)", 3.94,     MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.),   Some(3.94),          -53.194,  -51.994,  -52.594,    5.034,       5.139,         Some(0.),             Some(0.),          ),
            ("Цистерна питьевой воды 1 ЛБ",             3.71,     MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.),   Some(3.71),           40.156,   42.556,   41.44,    -0.575,       5.02,          Some(0.),             Some(0.),          ),
            ("Цистерна запасного масла ДГ",             2.493,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.9),  Some(2.77),          -53.194,  -51.994,  -52.594,    1.575,       5.3,           Some(0.1),            Some(0.2),         ),
            ("Цистерна мочевины ДП",                    32.84,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.32), Some(24.8787878788), -53.194,  -50.794,  -51.994,    0.,          2.63393939394, Some(14.8),           Some(4.8),         ),
            ("Экипаж и багаж",                          1.8,      MatterType::Solid,  CargoGeneralCategory::Stores,   None,       None,                 44.31,    46.31,    45.31,    -4.46,        7.7,           None,                 None,              ),
            ("Провизия",                                1.2,      MatterType::Solid,  CargoGeneralCategory::Stores,   None,       None,                 40.21,    42.21,    41.21,     4.6,         8.15,          None,                 None,              ),
            ("Цистерна запасного масла ГД",             2.493,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.9),  Some(2.77),          -53.194,  -51.994,  -52.594,    0.525,       5.3,           Some(0.1),            Some(0.2),         ),    
        ])
}
//
#[allow(dead_code)]
pub fn compartment_100_sea_grain() -> CompartmentArray {       
    CompartmentArray::from(vec![                                                                                                                                                                                                                
        //      name,                                     mass,     matter_type,        general_category,                 density,    volume,              bound_x1, bound_x2, mass_shift_x,mass_shift_y,mass_shift_z, m_f_s_y,             m_f_s_x,     
                ("Цистерна питьевой воды 2 ЛБ",             3.71,     MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.),   Some(3.71),          -24.744,   42.556,  41.44,     -1.325,       5.02,          Some(0.),             Some(0.),          ),
                ("Цистерна расходного топлива 1 ЛБ",        5.504,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(6.4),           -55.594,  -53.194,  -54.394,   -1.725,       5.299,         Some(0.4),            Some(1.4),         ),
                ("Цистерна расходного топлива 2 ЛБ",        5.504,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(6.4),           -55.594,  -53.194,  -54.394,   -2.975,       5.299,         Some(0.4),            Some(1.4),         ),
                ("Цистерна запасного топлива 1 Пр.Б",       34.9762,  MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(40.67),         -45.994,  -44.194,  -45.146,    2.802,       3.835,         Some(0.),             Some(0.),          ),
                ("Цистерна запасного топлива 2 ЛБ",         35.0966,  MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(40.81),         -45.994,  -44.194,  -45.248,   -2.796,       3.835,         Some(0.),             Some(0.),          ),
                ("Цистерна запасного топлива 3 Пр.Б",       53.42,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(62.11627907),   -57.394,  -53.194,  -55.012,    3.265875969, 2.994,         Some(12.66356589146), Some(4.973643411), ),
                ("Цистерна запасного топлива 3 ЛБ",         53.42,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(62.11627907),   -57.394,  -53.194,  -55.012,   -3.265875969, 2.994,         Some(12.66356589146), Some(4.973643411), ),
                ("Цистерна расходного топлива АДГ Пр.Б",    1.0578,   MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.86), Some(1.23),           39.556,   40.756,   39.965,    2.022,       11.552,        Some(0.),             Some(0.1),         ),
                ("Цистерна пресной воды 1 ДП (носовая)",    45.09,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.),   Some(45.09),          41.356,   43.756,   42.556,   -1.25,        2.31,          Some(0.),             Some(0.),          ),
                ("Цистерна пресной воды 2 Пр.Б (кормовая)", 3.94,     MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.),   Some(3.94),          -53.194,  -51.994,  -52.594,    5.034,       5.139,         Some(0.),             Some(0.),          ),
                ("Цистерна питьевой воды 1 ЛБ",             3.71,     MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.),   Some(3.71),           40.156,   42.556,   41.44,    -0.575,       5.02,          Some(0.),             Some(0.),          ),
                ("Цистерна запасного масла ДГ",             2.493,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.9),  Some(2.77),          -53.194,  -51.994,  -52.594,    1.575,       5.3,           Some(0.1),            Some(0.2),         ),
                ("Цистерна мочевины ДП",                    32.84,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(1.32), Some(24.8787878788), -53.194,  -50.794,  -51.994,    0.,          2.63393939394, Some(14.8),           Some(4.8),         ),
                ("Экипаж и багаж",                          1.8,      MatterType::Solid,  CargoGeneralCategory::Stores,   None,       None,                 44.31,    46.31,    45.31,    -4.46,        7.7,           None,                 None,              ),
                ("Провизия",                                1.2,      MatterType::Solid,  CargoGeneralCategory::Stores,   None,       None,                 40.21,    42.21,    41.21,     4.6,         8.15,          None,                 None,              ),
                ("Цистерна запасного масла ГД",             2.493,    MatterType::Liquid, CargoGeneralCategory::Stores,   Some(0.9),  Some(2.77),          -53.194,  -51.994,  -52.594,    0.525,       5.3,           Some(0.1),            Some(0.2),         ),    
                ("Балласт 5 ДД",                            159.8,    MatterType::Liquid, CargoGeneralCategory::Ballast,  Some(1.025),Some(155.90243902439),-22.144, -3.294,   -12.719,    0.,          0.409854942234,Some(0.),             Some(0.),          ),
    ])
}

