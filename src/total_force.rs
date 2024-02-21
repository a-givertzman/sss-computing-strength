//! Результирующая нагрузка на шпацию
use crate::math::vec::{MultipleSingle, SubVec};

/// Результирующей нагрузка на шпацию, вычисляется
/// суммированием силы выталкивания воды и суммарной  
/// нагрузки на судно
pub struct TotalForce {
    /// массив массы нагрузки на судно по шпациям
    mass_values: Vec<f64>,
    /// массив значений вытесненной массы воды по шпациям
    draught_values: Vec<f64>,
    /// ускорение свободного падения
    gravity_g: f64,
}
///
impl TotalForce {
    ///
    pub fn new(mass_values: Vec<f64>, draught_values: Vec<f64>, gravity_g: f64) -> Self {
        Self {
            mass_values,
            draught_values,
            gravity_g,
        }
    }
    ///
    pub fn values(&self) -> Vec<f64> {
        assert!(self.mass_values.len() == self.draught_values.len(), "mass.len() {} == draught.len() {}", self.mass_values.len(), self.draught_values.len());
        let mut result = self.mass_values.clone();
        result.sub_vec(&self.draught_values);
        result.mul_single(self.gravity_g);
        result
    }
}
