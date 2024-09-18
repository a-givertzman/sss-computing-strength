use crate::data::structs::LoadLineDataArray;

#[allow(dead_code)]
pub(crate) fn load_line() -> LoadLineDataArray {
    LoadLineDataArray::from(vec![
        ("LL draft S PS", 0., -6.7, 4.6),
        ("LL draft S SB", 0., 6.7, 4.708),
        ("LL draft F PS", 0.425, -6.7, 4.708),
        ("LL draft F SB", 0.425, 6.7, 4.708),
    ])
}