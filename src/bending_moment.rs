//! Изгибающий момент
use crate::math::vec::IntegralSum;

/// Изгибающий момент, интегриральная сумма срезающей  
/// силы, $M_i = M_{i-1} + SF_{i-1} + SF_i, M_0 = 0$
pub struct BendingMoment<'a> {
    /// массив значений средающей силы по шпациям
    shear_force_values: &'a Vec<f64>,
}
///
impl<'a> BendingMoment<'a> {
    ///
    pub fn new(shear_force_values: &'a Vec<f64> ) -> Self {
        Self { shear_force_values }
    }
    ///
    pub fn values(&self) -> Vec<f64>  {
        self.shear_force_values.integral_sum()
    }
}