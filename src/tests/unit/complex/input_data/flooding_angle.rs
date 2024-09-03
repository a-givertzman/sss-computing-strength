use crate::data::structs::FloodingAngleDataArray;

#[allow(dead_code)]
pub(crate) fn flooding_angle() -> FloodingAngleDataArray {
    FloodingAngleDataArray::from(vec![
        (2.3, 88.9),
        (2.5, 84.2),
        (3.0, 72.9),
        (3.5, 61.7),
        (4.0, 52.2),
        (4.6, 44.0),
    ])
}