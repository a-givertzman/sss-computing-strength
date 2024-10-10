use std::collections::HashMap;
use crate::data::structs::icing::*;
//
impl From<Vec<(&str, f64)>> for IcingArray {
    fn from(src: Vec<(&str, f64)>) -> Self {
        Self{data: src.into_iter().map(|(key, value)| IcingData{key: key.to_owned(), value} ).collect(), error: HashMap::new() }
    }
}
//
#[allow(dead_code)]
#[deny(clippy::approx_constant)]
pub(crate) fn icing() -> IcingArray {
    IcingArray::from(vec![
        ("icing_m_timber", 0.032),
        ("icing_m_v_full", 0.015),
        ("icing_m_v_half", 0.0075),
        ("icing_m_h_full", 0.03),
        ("icing_m_h_half", 0.015),
        ("icing_coef_v_area_full", 0.1),
        ("icing_coef_v_area_half", 0.075),
        ("icing_coef_v_area_zero", 0.05),
        ("icing_coef_v_moment_full", 0.2),
        ("icing_coef_v_moment_half", 0.15),
        ("icing_coef_v_moment_zero", 0.1),
    ])
}