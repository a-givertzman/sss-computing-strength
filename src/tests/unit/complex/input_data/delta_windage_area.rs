use crate::data::structs::DeltaWindageAreaDataArray;

#[allow(dead_code)]
pub(crate) fn delta_windage_area() -> DeltaWindageAreaDataArray {
    DeltaWindageAreaDataArray::from(vec![
        (0.5, -98.3),
        (1., -44.2),
        (1.5, 11.186),
        (2., 67.8),
        (2.5, 125.6),
        (3., 184.5),
        (3.5, 244.),
        (4., 303.6),
        (4.5, 363.334),
        (5., 423.),
        (5.5, 482.9),
        (6., 542.846),
        (6.5, 602.762),
        (6.8, 638.716),
    ])
}
