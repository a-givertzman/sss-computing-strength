use std::collections::HashMap;

use crate::{
    data::structs::{ComputedFrameData, ComputedFrameDataArray},
    Bounds,
};

#[allow(dead_code)]
pub(crate) fn bounds(loa: f64, middle_x: f64, n: usize) -> ComputedFrameDataArray {
    ComputedFrameDataArray{data: Bounds::from_n(loa, middle_x, n)
        .unwrap()
        .iter()
        .enumerate()
        .map(|(index, v)| ComputedFrameData {
            index: index as i32,
            start_x: v.start().unwrap(),
            end_x: v.end().unwrap(),
        })
        .collect(),
        error: HashMap::new(),
    }
}
