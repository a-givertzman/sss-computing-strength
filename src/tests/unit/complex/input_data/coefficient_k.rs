use crate::data::structs::CoefficientKArray;

#[allow(dead_code)]
pub(crate) fn coefficient_k() -> CoefficientKArray {
    CoefficientKArray::from(vec![
        (0.0, 1.0),
        (1.0, 0.98),
        (1.5, 0.95),
        (2.0, 0.88),
        (2.5, 0.79),
        (3.0, 0.74),
        (3.5, 0.72),
        (4.0, 0.70),
    ])
}