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
        log::info!("\t BendingMoment result:{:?}", result); 
        let last_value = result.last().expect("BendingMoment error: no result values!");
        let delta = *last_value/((result.len()-1) as f64);
        result.iter_mut().enumerate().for_each(|(i, v)| *v -= delta*(i as f64) );
        log::info!("\t BendingMoment result_fixed:{:?}", result); 
        assert!(*result.last().expect("BendingMoment error: no result values!") == 0., "BendingMoment result.last {} == 0", result.last().expect("ShearForce error: no result values!")); 
        result
    }
}
