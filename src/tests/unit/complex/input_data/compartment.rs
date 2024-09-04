use std::collections::HashMap;

use crate::data::structs::loads::{CargoGeneralCategory, CompartmentArray, CompartmentData, MatterType};
///
impl From<Vec<(  
    &str,
    Option::<f64>,
    Option::<f64>,
    Option::<f64>,
    f64,
    f64,
    Option::<f64>,
    Option::<f64>,
    Option::<f64>,
    Option::<f64>,
    Option::<f64>,
    Option::<f64>,
    CargoGeneralCategory,
    MatterType
)>> for CompartmentArray {
    fn from(src: Vec<(&str,
        Option::<f64>,
        Option::<f64>,
        Option::<f64>,
        f64,
        f64,
        Option::<f64>,
        Option::<f64>,
        Option::<f64>,
        Option::<f64>,
        Option::<f64>,
        Option::<f64>,
        CargoGeneralCategory,
        MatterType
    )>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(                    
                    name,                    
                    mass,
                    density,
                    volume,
                    bound_x1,
                    bound_x2,
                    mass_shift_x,
                    mass_shift_y,
                    mass_shift_z,
                    m_f_s_y,
                    m_f_s_x,
                    grain_moment,
                    general_category,
                    matter_type,
                )| CompartmentData {
                    name: name.to_owned(),
                    mass,
                    density,
                    volume,
                    bound_x1,
                    bound_x2,
                    mass_shift_x,
                    mass_shift_y,
                    mass_shift_z,
                    m_f_s_y,
                    m_f_s_x,
                    grain_moment,
                    general_category,
                    matter_type,
                })
                .collect(),
            error: HashMap::new(),
        }
    }
}
///
#[allow(dead_code)]
pub(crate) fn cargo() -> CompartmentArray {
    CompartmentArray::from(vec![
        name                   |        mass        | matter_type | general_category | density |       volume       | bound_x1 | bound_x2 | mass_shift_x |    mass_shift_y    |   mass_shift_z    |      m_f_s_y       |      m_f_s_x      | grain_moment
        -----------------------------------------+--------------------+-------------+------------------+---------+--------------------+----------+----------+--------------+--------------------+-------------------+--------------------+-------------------+--------------
         Цистерна питьевой воды 2 ЛБ             |               3.71 | liquid      | stores           |       1 |               3.71 |  -24.744 |   42.556 |        41.44 |             -1.325 |              5.02 |                  0 |                 0 |
         Цистерна расходного топлива 1 ЛБ        | 5.5040000000000004 | liquid      | stores           |    0.86 |                6.4 |  -55.594 |  -53.194 |      -54.394 |             -1.725 |             5.299 |                0.4 |               1.4 |
         Цистерна расходного топлива 2 ЛБ        | 5.5040000000000004 | liquid      | stores           |    0.86 |                6.4 |  -55.594 |  -53.194 |      -54.394 |             -2.975 |             5.299 |                0.4 |               1.4 |
         Цистерна запасного топлива 1 Пр.Б       |            34.9762 | liquid      | stores           |    0.86 |              40.67 |  -45.994 |  -44.194 |      -45.146 |              2.802 |             3.835 |                  0 |                 0 |
         Цистерна запасного топлива 2 ЛБ         |            35.0966 | liquid      | stores           |    0.86 |              40.81 |  -45.994 |  -44.194 |      -45.248 |             -2.796 |             3.835 |                  0 |                 0 |
         Цистерна запасного топлива 3 Пр.Б       |              53.42 | liquid      | stores           |    0.86 | 62.116279069767444 |  -57.394 |  -53.194 |      -55.012 |  3.265875968992248 |             2.994 | 12.663565891459932 | 4.973643410847633 |
         Цистерна запасного топлива 3 ЛБ         |              53.42 | liquid      | stores           |    0.86 | 62.116279069767444 |  -57.394 |  -53.194 |      -55.012 | -3.265875968992248 |             2.994 | 12.663565891459932 | 4.973643410847633 |
         Цистерна расходного топлива АДГ Пр.Б    |             1.0578 | liquid      | stores           |    0.86 |               1.23 |   39.556 |   40.756 |       39.965 |              2.022 |            11.552 |                  0 |               0.1 |
         Цистерна пресной воды 1 ДП (носовая)    |              45.09 | liquid      | stores           |       1 |              45.09 |   41.356 |   43.756 |       42.556 |              -1.25 |              2.31 |                  0 |                 0 |
         Цистерна пресной воды 2 Пр.Б (кормовая) |               3.94 | liquid      | stores           |       1 |               3.94 |  -53.194 |  -51.994 |      -52.594 |              5.034 |             5.139 |                  0 |                 0 |
         Цистерна питьевой воды 1 ЛБ             |               3.71 | liquid      | stores           |       1 |               3.71 |   40.156 |   42.556 |        41.44 |             -0.575 |              5.02 |                  0 |                 0 |
         Цистерна запасного масла ДГ             |              2.493 | liquid      | stores           |     0.9 |               2.77 |  -53.194 |  -51.994 |      -52.594 |              1.575 |               5.3 |                0.1 |               0.2 |
         Цистерна мочевины ДП                    | 32.839999999999996 | liquid      | stores           |    1.32 | 24.878787878787875 |  -53.194 |  -50.794 |      -51.994 |                  0 | 2.633939393939394 |               14.8 |               4.8 |
         Экипаж и багаж                          |                1.8 | solid       | stores           |         |                    |    44.31 |    46.31 |        45.31 |              -4.46 |               7.7 |                    |                   |
         Провизия                                |                1.2 | solid       | stores           |         |                    |    40.21 |    42.21 |        41.21 |                4.6 |              8.15 |                    |                   |
         Цистерна запасного масла ГД             |              2.493 | liquid      | stores           |     0.9 |               2.77 |  -53.194 |  -51.994 |      -52.594 |              0.525 |               5.3 |                0.1 |               0.2 |
         
        // (name, mass, volume_max, density, bound_x1, bound_x2, category_id)
              ("Цистерна расходного топлива 1 ЛБ",   5.51, 6.4,  0.86, -55.594,  -53.194,  4),
              ("Цистерна расходного топлива 2 ЛБ",   5.51, 6.4,  0.86, -55.594,  -53.194, 4),
              ("Цистерна запасного топлива 1 Пр.Б",  34.98, 40.67, 0.86, -45.994, -44.194, 4),
              ("Цистерна запасного топлива 2 ЛБ",    35.10, 40.81, 0.86, -45.994, -44.194, 4),
              ("Цистерна запасного топлива 3 Пр.Б",  53.42, 62.12, 0.86, -57.394,  -53.194, 4),
              ("Цистерна запасного топлива 3 ЛБ",    53.42, 62.12, 0.86, -57.394,  -53.194, 4),
              ("Цистерна переливная ДП",             0., 16.37, 0.86, -48.994, -45.994, 4),
              ("Цистерна расходного топлива АДГ Пр.Б",     1.06, 1.24,  0.86, 39.556, 40.756, 4),
              ("Цистерна пресной воды 1 ДП (носовая)",     45.09, 45.09, 1.,   41.356, 43.756, 7),
              ("Цистерна пресной воды 2 Пр.Б (кормовая)",  3.94, 3.94,  1.,    -53.194, -51.994, 7),
              ("Цистерна питьевой воды 1 ЛБ",  3.71, 3.71, 1.,   40.156, 42.556, 7),
              ("Цистерна питьевой воды 2 ЛБ",  3.71, 3.71, 1.,   -24.744, 42.556, 7),
              ("Цистерна запасного масла ГД",  2.49, 2.77, 0.9, -53.194, -51.994, 5),
              ("Цистерна запасного масла ДГ",  2.49, 2.77, 0.9, -53.194, -51.994, 5),
              ("Цистерна отработанного масла ЛБ", 0., 4.25, 0.9, -53.194, -48.994, 5),
              ("Цистерна мочевины ДП",         32.84, 28.22, 1.32, -53.194, -50.794,  6),
              ("Цистерна нефтеостатков ДП",    0., 8.5,  0.86, -53.194, -48.994, 8),
              ("Шламовая цистерна Пр.Б",       0., 2.12,  0.86, -53.194, -48.994, 8),
              ("Сточная цистерна 1 ДП",        0., 15.35, 0.86, 49.756, 51.556, 8),
              ("Сточная цистерна 2 ЛБ",        0., 0.7,  0.86, -53.194, -51.994, 8),
            
      //        (name, mass, bound_x1, bound_x2, mass_shift_x, mass_shift_y, mass_shift_z, category_id)
              ("Экипаж и багаж",     1.80, 44.31,  46.31, 45.31,  -4.46,  7.70, 9),
              ("Провизия",           0,    40.21,  42.21, 41.21,  4.60,   8.15, 9),
            
     //         (ship_id, space_id, name, active, mass, bound_x1, bound_x2, mass_shift_x, mass_shift_y, mass_shift_z, category_id)

              ("Палубный груз", TRUE, 0., -44.194, 37.656,  0,  0,  10.3, 10),
    ])
}