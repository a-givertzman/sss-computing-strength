use crate::{draught::Draught, mass::Mass, math::vec::*};

///распределение результирующей нагрузка на шпацию, ньютоны
pub struct TotalForce<'a> {
    mass: &'a Mass<'a>,
    draught: Draught<'a>,
    gravity_g: f64,
}

impl<'a> TotalForce<'a> {
    ///
    pub fn new(mass: &'a Mass, draught: Draught<'a>, gravity_g: f64) -> Self {
        Self {
            mass,
            draught,
            gravity_g,
        }
    }
    ///
    pub fn values(&self) -> Vec<f64> {
        assert!(self.mass.values().len() == self.draught.values().len(), "mass.len() {} == draught.len() {}", self.mass.values().len(), self.draught.values().len());
        let mut result = self.mass.values();
        result.sub_vec(&self.draught.values());
        result.mul_single(self.gravity_g);
        result
    }
}
