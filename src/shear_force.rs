use crate::{math::vec::SumAbove, total_force::TotalForce};

///срезающая сила, интегрирование путем вычисления суммы сверху, s_f[i] = s_f[i-1] + f[i], s_f[0] = 0
pub struct ShearForce<'a> {
    total_force: TotalForce<'a>,
}

impl<'a> ShearForce<'a> {
    ///
    pub fn new(total_force: TotalForce<'a>) -> Self {
        Self { total_force }
    }
    ///
    pub fn values(&self) -> Vec<f64> {
        self.total_force.values().sum_above()
    }
}