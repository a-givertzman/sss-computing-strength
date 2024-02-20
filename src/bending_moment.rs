use crate::{math::vec::IntegralSum, shear_force::ShearForce};


///изгибающий момент, интегриральная сумма срезающей силы, m[i] = m[i-1] + s_f[i-1] + s_f[i], m[0] = 0
pub struct BendingMoment<'a> {
    shear_force: &'a ShearForce<'a>,
}

impl<'a> BendingMoment<'a> {
    ///
    pub fn new(shear_force: &'a ShearForce) -> Self {
        Self { shear_force }
    }
    ///
    pub fn values(&self) -> Vec<f64>  {
        self.shear_force.values().integral_sum()
    }
}