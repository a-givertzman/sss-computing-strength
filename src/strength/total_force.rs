//! Результирующая нагрузка на шпацию
use std::rc::Rc;

use crate::math::*;
use super::mass::*;
use super::volume::IVolume;

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
    pub fn new(mass: Rc<dyn IMass>, water_density: f64, volume: impl IVolume + 'static, gravity_g: f64) -> Self {
        assert!(gravity_g > 0., "gravity_g {gravity_g} > 0.");
        Self {
            mass,
            water_density,
            volume: Box::new(volume),
            gravity_g,
        }
    }
}
/// 
impl ITotalForce for TotalForce {
    ///
    fn values(&mut self) -> Vec<f64> {
        let mass_values = self.mass.values();
        let mut volume_values = self.volume.values();
        assert!(mass_values.len() == volume_values.len(), "mass.len() {} == volume.len() {}", mass_values.len(), volume_values.len());
        let mut result = mass_values.clone();
        volume_values.mul_single(self.water_density);
        result.sub_vec(&volume_values);
        result.mul_single(self.gravity_g);
  //      log::info!("\t TotalForce mass:{:?} volume:{:?} result:{:?}, mass_sum:{}, volume_mass_sum:{}", mass_values, volume_values, result, mass_values.iter().sum::<f64>(), volume_values.iter().sum::<f64>());
        result
    }
}

#[doc(hidden)]
pub trait ITotalForce {
    fn values(&mut self) -> Vec<f64>;
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
    fn values(&mut self) -> Vec<f64> {
        self.data.clone()
    }
}
