use crate::{draught::Draught, mass::Mass};

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
        assert_eq!(self.mass.values().len(), self.draught.values().len());
        self.mass
            .values()
            .iter()
            .zip(self.draught.values().iter())
            .map(|(m, bf)| (m - bf) * self.gravity_g)
            .collect()
    }
}