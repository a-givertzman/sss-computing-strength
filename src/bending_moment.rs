//! Изгибающий момент
use crate::{math::IntegralSum, shear_force::IShearForce, MultipleSingle};

/// Изгибающий момент, интегриральная сумма срезающей  
/// силы, $M_i = M_{i-1} + SF_{i-1} + SF_i, M_0 = 0$
pub struct BendingMoment<'a> {
    /// массив значений средающей силы по шпациям
    shear_force: &'a mut dyn IShearForce,
    /// длинна элемента разбиения   
    delta_l: f64, 
}
///
impl<'a> BendingMoment<'a> {
    ///
    pub fn new(shear_force: &'a mut (dyn IShearForce + 'a), delta_l: f64 ) -> Self {
        Self { shear_force, delta_l }
    }
    ///
    pub fn values(&mut self) -> Vec<f64>  {
        let mut result = self.shear_force.values().integral_sum();
        result.mul_single(self.delta_l/2.);
        let last_value = result.last().expect("BendingMoment error: no result values!");
        let delta = *last_value/result.len() as f64;
        result.iter_mut().enumerate().for_each(|(i, v)| *v -= delta*(i as f64) );
        assert!(*result.last().expect("BendingMoment error: no result values!") == 0., "BendingMoment result.last {} == 0", result.last().expect("ShearForce error: no result values!"));
        log::info!("\t BendingMoment result:{:?}", result);  
        result
    }
}
