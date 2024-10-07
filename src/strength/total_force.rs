//! Результирующая нагрузка на шпацию
use std::rc::Rc;

use super::mass::*;
use super::volume::IVolume;
use crate::{math::*, Error};

/// Результирующая нагрузка на шпацию, сумма сил
/// действующая на каждую шпацию судна
pub struct TotalForce {
    /// нагрузка на судно
    mass: Rc<dyn IMass>,
    /// Плотность воды
    water_density: f64,
    /// объем вытесненной воды
    volume: Box<dyn IVolume>,
    /// ускорение свободного падения
    gravity_g: f64,
}
///
impl TotalForce {
    ///
    pub fn new(
        mass: Rc<dyn IMass>,
        water_density: f64,
        volume: impl IVolume + 'static,
        gravity_g: f64,
    ) -> Result<Self, Error> {
        if gravity_g <= 0. {
            return Err(Error::FromString(
                "TotalForce new error: gravity_g <= 0.".to_string(),
            ));
        }
        Ok(Self {
            mass,
            water_density,
            volume: Box::new(volume),
            gravity_g,
        })
    }
}
///
impl ITotalForce for TotalForce {
    ///
    fn values(&mut self) -> Result<Vec<f64>, Error> {
        let mass_values = self.mass.values()?;
        let mut volume_values = self.volume.values()?;
        if mass_values.len() != volume_values.len() {
            let error = Error::FromString(
                "TotalForce values error: mass_values.len() != volume_values.len()".to_owned(),
            );
            log::error!("{error}");
            return Err(error);
        }
        let mut result = mass_values.clone();
        volume_values.mul_single(self.water_density);
        result.sub_vec(&volume_values)?;
        result.mul_single(self.gravity_g);
        log::trace!(
            "\t TotalForce mass:{:?} volume:{:?} result:{:?}, mass_sum:{}, volume_mass_sum:{}",
            mass_values,
            volume_values,
            result,
            mass_values.iter().sum::<f64>(),
            volume_values.iter().sum::<f64>()
        );
        Ok(result)
    }
}

#[doc(hidden)]
pub trait ITotalForce {
    fn values(&mut self) -> Result<Vec<f64>, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeTotalForce {
    data: Vec<f64>,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeTotalForce {
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }
}
#[doc(hidden)]
impl ITotalForce for FakeTotalForce {
    fn values(&mut self) -> Result<Vec<f64>, Error> {
        Ok(self.data.clone())
    }
}
