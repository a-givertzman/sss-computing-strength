//! Изгибающий момент
use crate::{math::IntegralSum, shear_force::IShearForce};

/// Изгибающий момент, интегриральная сумма срезающей  
/// силы, $M_i = M_{i-1} + SF_{i-1} + SF_i, M_0 = 0$
pub struct BendingMoment<'a> {
    /// массив значений средающей силы по шпациям
    shear_force: &'a dyn IShearForce,
}
///
impl<'a> BendingMoment<'a> {
    ///
    pub fn new(shear_force: &'a impl IShearForce ) -> Self {
        Self { shear_force }
    }
    ///
    pub fn values(&self) -> Vec<f64>  {
        let result = self.shear_force.values().integral_sum();
        log::info!("\t BendingMoment result:{:?}", result);
        result
    }
}
