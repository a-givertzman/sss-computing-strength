use crate::data::structs::BowAreaDataArray;

#[allow(dead_code)]
pub(crate) fn bow_area() -> BowAreaDataArray {
    BowAreaDataArray::from(vec![
        (0.5,	154.6),
        (1.,	147.3),
        (1.4,	141.2),
        (1.5,	139.6),
        (2.,	131.6),
        (2.5,	123.4),
        (3.,	115.),
        (3.5,	106.5),
        (4.,	98.),
        (4.5,	89.3),
        (5.,	80.5),
        (5.5,	71.7),
        (6.,	62.8),
        (6.5,	53.9),
        (6.8,	48.6),
    ])
}