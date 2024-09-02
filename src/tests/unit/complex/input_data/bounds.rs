use crate::Bounds;

#[allow(dead_code)]
pub(crate) fn bounds(loa: f64, middle_x: f64, n: usize) -> Vec<(f64, f64)> {
    Bounds::from_n(loa, middle_x, n).unwrap().iter().map(|v| (v.start().unwrap(), v.end().unwrap())).collect::<Vec<(f64, f64)>>()
}