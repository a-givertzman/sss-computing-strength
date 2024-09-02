use crate::data::structs::CoefficientKThetaArray;

#[allow(dead_code)]
pub(crate) fn coefficient_k_theta() -> CoefficientKThetaArray {
    CoefficientKThetaArray::from(vec![
        (2.5, 1.),
        (3.0, 1.08),
        (3.5, 1.11),
        (4.0, 1.11),
        (4.5, 1.2),
        (5.0, 1.3),
        (5.5, 1.45),
        (6.0, 1.56),
        (6.5, 1.61),
    ])
}