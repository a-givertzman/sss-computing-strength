use crate::data::structs::LoadLineDataArray;

#[allow(dead_code)]
pub(crate) fn load_line() -> LoadLineDataArray {
    LoadLineDataArray::from(vec![
        ("Summer PS", 0., 6.7, 4.6),
        ("Summer SB", 0., -6.7, 4.708),
        ("Fresh water summer PS", 0.425, 6.7, 4.708),
        ("Fresh water summer SB", 0.425, -6.7, 4.708),
    ])
}
