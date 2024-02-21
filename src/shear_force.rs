//! Срезающая сила
use crate::math::vec::SumAbove;

/// Срезающая сила, вычисляется интегрированием  
/// путем вычисления суммы сверху результирующей нагрузки по шпациям:  
/// $SF_i = SF_{i-1} + TF_i, SF_0 = 0$
pub struct ShearForce {
    /// массив значений результирующей нагрузки по шпациям
    total_force_values: Vec<f64>,
}

impl ShearForce {
    ///
    pub fn new(total_force_values: Vec<f64>) -> Self {
        Self { total_force_values }
    }
    ///
    pub fn values(&self) -> Vec<f64> {
        self.total_force_values.sum_above()
    }
}