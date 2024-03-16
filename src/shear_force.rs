//! Срезающая сила
use crate::{math::SumAbove, total_force::ITotalForce};

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
    fn values(&mut self) -> Vec<f64> {
        let mut result = self.total_force.values().sum_above();
        // поправка
        let last_value = result.last().expect("ShearForce error: no result values!");
        let delta = *last_value/result.len() as f64;
        result.iter_mut().enumerate().for_each(|(i, v)| *v -= delta*(i as f64) );
        assert!(*result.last().expect("ShearForce error: no result values!") == 0., "ShearForce result.last {} == 0", result.last().expect("ShearForce error: no result values!"));
        log::info!("\t ShearForce result:{:?}", result);
        result
    }
}

#[doc(hidden)]
pub trait IShearForce {
    fn values(&mut self) -> Vec<f64>;
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
    fn values(&mut self) -> Vec<f64> {
        self.data.clone()
    }
}