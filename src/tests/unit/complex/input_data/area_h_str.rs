use std::collections::HashMap;
use crate::data::structs::{HStrArea, HStrAreaArray};
//
impl From<Vec<(&str, f64, f64, f64)>> for HStrAreaArray {
    fn from(src: Vec<(&str, f64, f64, f64)>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(name, value, bound_x1, bound_x2)| HStrArea {
                    name: name.to_owned(),
                    value,
                    bound_x1,
                    bound_x2,
                })
                .collect(),
            error: HashMap::new(),
        }
    }
}
//
#[allow(dead_code)]
pub(crate) fn area_h_str() -> HStrAreaArray {
    HStrAreaArray::from(vec![
        ("Палуба бака 17 шпация", 31.52, 35.5164, 41.4358),
        ("Палуба бака 18 шпация", 18.01, 41.4358, 47.3554),
        ("Палуба бака 19 шпация", 25.05, 47.3554, 53.2746),
        ("Палуба бака 20 шпация", 60.62, 53.2746, 59.194),
        ("Верхняя палуба", 1334., -60.994, 38.956),
        ("Палуба ходового мостика", 197.8, 37.006, 52.156),
        ("Спасательная шлюпка", 13.9, -60.394, -53.194),
        ("Посадочная площадка", 4.2, -60.394, -56.194),
        ("Дежурная шлюпка", 6.6, 38.306, 40.156),
        ("Дымовые трубы", 12.1, -51.994, -49.594),
    ])
}
