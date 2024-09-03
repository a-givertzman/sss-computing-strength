use crate::data::structs::VolumeShiftArray;

#[allow(dead_code)]
pub(crate) fn volume_shift() -> VolumeShiftArray {
    VolumeShiftArray::from(vec![
        (0.5, 0.25),
        (1., 0.51),
        (1.5, 0.76),
        (2., 1.02),
        (2.5, 1.28),
        (3., 1.54),
        (3.5, 1.80),
        (4., 2.05),
        (4.5, 2.31),
        (5., 2.56),
        (5.5, 2.82),
        (6., 3.07),
        (6.5, 3.32),
        (6.8, 3.48)
    ])
}