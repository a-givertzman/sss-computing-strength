use crate::data::structs::WaterlineLengthArray;

#[allow(dead_code)]
pub(crate) fn waterline_length() -> WaterlineLengthArray {
    WaterlineLengthArray::from(vec![
        (0.5, 106.29),
        (1., 109.53),
        (1.5, 112.11),
        (2., 114.47),
        (2.5, 116.71),
        (3., 118.88),
        (3.5, 119.11),
        (4., 119.30),
        (4.5, 119.47),
        (5., 119.60),
        (5.5, 119.74),
        (6., 119.81),
        (6.5, 119.85),
        (6.8, 119.85)
    ])
}