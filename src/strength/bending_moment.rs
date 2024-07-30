//! Изгибающий момент

use crate::{math::vec::*, Error};

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
    pub fn new(shear_force: Box<dyn IShearForce>, delta: f64) -> Self {
        Self { shear_force, delta }
    }
    ///
    pub fn values(&mut self) -> Result<Vec<f64>, Error> {
        let mut result: Vec<f64> = self
            .shear_force
            .values()?
            .integral_sum()
            .into_iter()
            .map(|v| -v)
            .collect();
        result.mul_single(self.delta / 2.);
        //      log::info!("\t BendingMoment result:{:?}", result);
        Ok(result)
    }
}
