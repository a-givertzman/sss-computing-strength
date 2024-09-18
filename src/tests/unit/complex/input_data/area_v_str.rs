use std::collections::HashMap;
use crate::data::structs::strength::*;

///
impl From<Vec<(&str, f64, f64, f64, f64)>> for VerticalAreaArray {
    fn from(src: Vec<(&str, f64, f64, f64, f64)>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(name, shift_z, value, bound_x1, bound_x2)| VerticalArea {
                    name: name.to_owned(),
                    value,
                    shift_z,
                    bound_x1,
                    bound_x2,
                })
                .collect(),
            error: HashMap::new(),
        }
    }
}
///
#[allow(dead_code)]
pub(crate) fn area_v_str() -> VerticalAreaArray {
    VerticalAreaArray::from(vec![
        ("Надводный борт",         4.13,  637.5,   -60.994,   59.356),
        ("Кормовая часть",         9.34,  38.39,   -53.194,   -47.794),
        ("Бак c фальшборотом",     8.23,  59.59,   37.006,    59.356),
        ("Комингсы",               7.47,  107.9,   -44.194,   36.356),
        ("Крышки люков",           8.51,  59.70,   33.106,    37.006),
        ("Кран для люковых крышек",12.14, 13.53,   -60.994,   59.356),
        ("Нос",                    12.58, 101.14,  37.656,    52.756),
        ("Корма",                  10.13, 18.04,   -60.994,   -53.194),
        ("Мачта носовая",          18.92, 0.86,    41.956,    52.156),
        ("Мачта кормовая",         18.22, 1.25,    -50.194,   -48.394)
    ])
}