use std::collections::HashMap;

use crate::data::structs::{navigation_area_data::NavigationAreaData, NavigationArea, NavigationAreaArray};

///
impl From<Vec<(NavigationArea, f64, f64)>> for NavigationAreaArray {
    fn from(src: Vec<(NavigationArea, f64, f64)>) -> Self {
        Self{data: src.into_iter().map(|(area, p_v, m)| NavigationAreaData{area, p_v, m} ).collect(), error: HashMap::new() }
    }
}
///
#[allow(dead_code)]
pub(crate) fn navigation_area() -> NavigationAreaArray {
    NavigationAreaArray::from(vec![
        (NavigationArea::Unlimited, 504., 0.5),
        (NavigationArea::R1, 353., 0.5),
        (NavigationArea::R2, 252., 0.52),
        (NavigationArea::R2Rsn, 252., 0.52),
        (NavigationArea::R2Rsn45, 166., 0.54),
        (NavigationArea::R3Rsn, 119., 0.55)
    ])
}