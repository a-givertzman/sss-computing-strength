use crate::math::vec::SumAbove;

///срезающая сила, интегрирование путем вычисления суммы сверху, s_f[i] = s_f[i-1] + f[i], s_f[0] = 0
pub struct ShearForce {
    total_force_values: Vec<f64>,
}

impl ShearForce {
    ///
    pub fn new(total_force_values: Vec<f64>) -> Self {
        Self { total_force_values }
    }
    ///
    pub fn values(&self) -> Vec<f64> {
//        dbg!(&self.total_force.values().sum_above());
        self.total_force_values.sum_above()
    }
}