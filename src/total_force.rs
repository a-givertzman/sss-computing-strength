use crate::math::vec::{MultipleSingle, SubVec};


///распределение результирующей нагрузка на шпацию, ньютоны
pub struct TotalForce {
    mass_values: Vec<f64>,
    draught_values: Vec<f64>,
    gravity_g: f64,
}

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
