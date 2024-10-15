//! Срезающая сила

use super::total_force::ITotalForce;
use crate::{math::vec::*, Error};

/// Срезающая сила, действующая на корпус судна
pub struct ShearForce {
    /// результирующая нагрузки по шпациям
    total_force: Box<dyn ITotalForce>,
}
//
impl ShearForce {
    /// Главный конструктор
    /// *total_force - результирующая нагрузки на шпацию
    pub fn new(total_force: impl ITotalForce + 'static) -> Self {
        Self {
            total_force: Box::new(total_force),
        }
    }
}
//
impl IShearForce for ShearForce {
    fn values(&mut self) -> Result<Vec<f64>, Error> {
        let result = self
            .total_force
            .values()?
            .sum_above()
            .into_iter()
            .map(|v| -v)
            .collect();
        log::trace!("\t ShearForce result:{:?}", result);
        Ok(result)
    }
}

#[doc(hidden)]
pub trait IShearForce {
    fn values(&mut self) -> Result<Vec<f64>, Error>;
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
    fn values(&mut self) -> Result<Vec<f64>, Error> {
        Ok(self.data.clone())
    }
}
