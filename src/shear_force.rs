//! Срезающая сила
use crate::{math::vec::SumAbove, total_force::ITotalForce};

/// Срезающая сила, вычисляется интегрированием  
/// путем вычисления суммы сверху результирующей нагрузки по шпациям:  
/// $SF_i = SF_{i-1} + TF_i, SF_0 = 0$

pub struct ShearForce {
    /// результирующая нагрузки по шпациям
    total_force: Box<dyn ITotalForce>,
}
///
impl ShearForce {
    ///
    pub fn new (total_force: impl ITotalForce + 'static) -> Self {
        Self { total_force: Box::new(total_force) }
    }
}
///
impl IShearForce for ShearForce {
    ///
    fn values(&self) -> Vec<f64> {
        let result = self.total_force.values().sum_above();
        log::debug!("\t ShearForce result:{:?}", result);
        result
    }
}

#[doc(hidden)]
pub trait IShearForce {
    fn values(&self) -> Vec<f64>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeShearForce {
    data: Vec<f64>,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeShearForce {
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }
}
#[doc(hidden)]
impl IShearForce for FakeShearForce {
    fn values(&self) -> Vec<f64> {
        self.data.clone()
    }
}