use crate::math::vec::IntegralSum;


///изгибающий момент, интегриральная сумма срезающей силы, m[i] = m[i-1] + s_f[i-1] + s_f[i], m[0] = 0
pub struct BendingMoment<'a> {
    shear_force_values: &'a Vec<f64>,
}

impl<'a> BendingMoment<'a> {
    ///
    pub fn new(shear_force_values: &'a Vec<f64> ) -> Self {
        Self { shear_force_values }
    }
    ///
    pub fn values(&self) -> Vec<f64>  {
        self.shear_force_values.integral_sum()
    }
}