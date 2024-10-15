use std::collections::HashMap;
use crate::data::structs::{HStabArea, HStabAreaArray};
//
impl From<Vec<(&str, f64, f64, f64, f64)>> for HStabAreaArray {
    fn from(src: Vec<(&str, f64, f64, f64, f64)>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(name, value, shift_x, shift_y, shift_z)| HStabArea {
                    name: name.to_owned(),
                    value,
                    shift_x,
                    shift_y,
                    shift_z,
                })
                .collect(),
            error: HashMap::new(),
        }
    }
}
//
#[allow(dead_code)]
pub(crate) fn area_h_stab() -> HStabAreaArray {
    HStabAreaArray::from(vec![
        ("Палуба бака", 135.2, 48.9, 0., 9.5),
        ("Верхняя палуба", 423.4, -24.5, 0., 6.9),
        ("Крышки люков трюма", 872.7, -3.3, 0., 8.9),
        ("Комингсы", 37.8, -3.6, 0., 8.2),
        ("Палуба ходового мостика", 81.1, 42.0, 0., 12.4),
        ("Крыша рубки", 116.7, 47.4, 0., 16.0),
        ("Спасательная шлюпка", 13.9, -57.9, -4.5, 13.0),
        ("Посадочная площадка", 4.2, -57.9, -4.5, 6.8),
        ("Дежурная шлюпка", 6.6, 39.0, 0.0, 13.2),
        ("Дымовые трубы", 12.1, -51.0, 0.0, 15.6),
    ])
}
