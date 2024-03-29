//! Изгибающий момент

use crate::math::vec::*;

use super::shear_force::IShearForce;


/// Изгибающий момент, действующий на корпус судна
pub struct BendingMoment {
    /// массив значений средающей силы по шпациям
    shear_force: Box<dyn IShearForce>,
    /// длинна элемента разбиения   
    delta: f64, 
}
///
impl BendingMoment {
    ///
    pub fn new(shear_force: Box<dyn IShearForce>, delta: f64 ) -> Self {
        Self { shear_force, delta }
    }
    ///
    pub fn values(&mut self) -> Vec<f64>  {
        let mut result = self.shear_force.values().integral_sum();
        result.mul_single(self.delta/2.);
  //      log::info!("\t BendingMoment result:{:?}", result); 
/*        let last_value = result.last().expect("BendingMoment error: no result values!");
        let delta = *last_value/((result.len()-1) as f64);
        result.iter_mut().enumerate().for_each(|(i, v)| *v -= delta*(i as f64) );
        log::info!("\t BendingMoment result_fixed:{:?}", result); 
        assert!(*result.last().expect("BendingMoment error: no result values!") == 0., "BendingMoment result.last {} == 0", result.last().expect("ShearForce error: no result values!")); 
  */      result
    }
}
