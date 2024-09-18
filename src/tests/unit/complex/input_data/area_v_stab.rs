use std::collections::HashMap;
use crate::data::structs::stability::*;

///
impl From<Vec<(f64, f64, f64, f64)>> for VerticalAreaArray {
    fn from(src: Vec<(f64, f64, f64, f64)>) -> Self {
        Self {
            data: src
                .into_iter()
                .map(|(draught, area, moment_x, moment_z)| VerticalArea {
                    draught,
                    area,
                    moment_x,
                    moment_z,
                })
                .collect(),
            error: HashMap::new(),
        }
    }
}
///
#[allow(dead_code)]
pub(crate) fn area_v_stab() -> VerticalAreaArray {
    VerticalAreaArray::from(vec![
        (0.5, 1136.2, 4333.19, 6547.68),
        (1., 1082.1, 4209.05, 6507.07),
        (1.4, 1037.9, 4131., 6454.),
        (1.5, 1026.71, 4115.29, 6437.78),
        (2., 970.1, 4063.37, 6338.67),
        (2.5, 912.3, 4060.44, 6208.58),
        (3., 853.4, 4112.04, 6046.62),
        (3.5, 793.9, 4188.28, 5853.27),
        (4., 734.3, 4257., 5629.69),
        (4.5, 674.57, 4319.85, 5375.99),
        (5., 614.9, 4377.05, 5092.36),
        (5.5, 555., 4429.38, 4778.34),
        (6., 495.05, 4478.34, 4433.42),
        (6.5, 435.14, 4525.81, 4058.9),
        (6.8, 399.18, 4554., 3819.81),
    ])
}
