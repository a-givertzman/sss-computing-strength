use crate::data::structs::EntryAngleDataArray;

#[allow(dead_code)]
pub(crate) fn entry_angle() -> EntryAngleDataArray {
    EntryAngleDataArray::from(vec![
        (2.3, 36.3),
        (2.5, 34.2),
        (3.0, 29.8),
        (3.5, 25.8),
        (4.0, 21.9),
        (4.6, 16.7),
    ])
}