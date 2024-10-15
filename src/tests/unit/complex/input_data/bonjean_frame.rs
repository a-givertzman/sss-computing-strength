use std::collections::HashMap;
use crate::data::structs::{FrameIndexData, FrameIndexDataArray};
//
impl From<Vec<(i32, f64)>> for FrameIndexDataArray {
    fn from(src: Vec<(i32, f64)>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(frame_index, pos_x)| FrameIndexData { frame_index, pos_x })
                .collect(),
            error: HashMap::new(),
        }
    }
}
//
#[allow(dead_code)]
pub(crate) fn bonjean_frame() -> FrameIndexDataArray {
    FrameIndexDataArray::from(vec![
        (0, -59.194),
        (1, -53.364),
        (2, -47.534),
        (3, -41.704),
        (4, -35.874),
        (5, -30.044),
        (6, -24.214),
        (7, -18.384),
        (8, -12.554),
        (9, -6.724),
        (10, -0.894),
        (11, 4.936),
        (12, 10.766),
        (13, 16.596),
        (14, 22.426),
        (15, 28.256),
        (16, 34.086),
        (17, 39.916),
        (18, 45.746),
        (19, 51.576),
        (20, 57.406),
    ])
}
