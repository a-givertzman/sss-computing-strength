//! Срезающая сила

use super::total_force::ITotalForce;
use crate::math::vec::*;

/// Срезающая сила, действующая на корпус судна
pub struct ShearForce {
    /// результирующая нагрузки по шпациям
    total_force: Box<dyn ITotalForce>,
}
///
impl ShearForce {
    ///
    pub fn new(total_force: impl ITotalForce + 'static) -> Self {
        Self {
            total_force: Box::new(total_force),
        }
    }
}
///
impl IShearForce for ShearForce {
    ///
    fn values(&mut self) -> Vec<f64> {
        let result = self
            .total_force
            .values()
            .sum_above()
            .into_iter()
            .map(|v| -v)
            .collect();
        //     log::info!("\t ShearForce result:{:?}", result);
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
