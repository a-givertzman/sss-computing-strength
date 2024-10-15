use std::collections::HashMap;
use crate::data::structs::{multipler_s::MultiplerSData, MultiplerSArray, NavigationArea};
//
impl MultiplerSArray {
    pub fn from(src: Vec<(NavigationArea, i32, f64)>) -> Self {
        Self{data: src.into_iter().map(|(area, t, s)| MultiplerSData{area, t: t as f64, s} ).collect(), error: HashMap::new() }
    }
}
//
#[allow(dead_code)]
pub(crate) fn multipler_s() -> MultiplerSArray {
    MultiplerSArray::from(vec![
        (NavigationArea::R2, 5, 0.1),
        (NavigationArea::R2, 6, 0.093),
        (NavigationArea::R2, 7, 0.083),
        (NavigationArea::R2, 8, 0.073),
        (NavigationArea::R2, 10, 0.053),
        (NavigationArea::R2, 12, 0.04),
        (NavigationArea::R2, 14, 0.035),
        (NavigationArea::R2, 16, 0.035),
        (NavigationArea::R2, 18, 0.035),
        (NavigationArea::R2, 20, 0.035),
    ])
}