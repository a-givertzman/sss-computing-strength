//! Результирующая нагрузка на шпацию
use std::rc::Rc;

use crate::{draught::IDraught, mass::IMass, math::{MultipleSingle, SubVec}};

/// Результирующей нагрузка на шпацию, вычисляется
/// суммированием силы выталкивания воды и суммарной  
/// нагрузки на судно
pub struct TotalForce {
    /// нагрузка на судно
    mass: Rc<dyn IMass>,
    /// Плотность воды
    water_density: f64,
    /// объем вытесненной воды
    draught: Box<dyn IDraught>,
    /// ускорение свободного падения
    gravity_g: f64,
}
///
impl TotalForce {
    ///
    pub fn new(mass: Rc<dyn IMass>, water_density: f64, draught: impl IDraught + 'static, gravity_g: f64) -> Self {
        assert!(gravity_g > 0., "gravity_g {gravity_g} > 0.");
        Self {
            mass,
            water_density,
            draught: Box::new(draught),
            gravity_g,
        }
    }
}
///
impl ITotalForce for TotalForce {
    ///
    fn values(&self) -> Vec<f64> {
        let mass_values = self.mass.values();
        let mut draught_values = self.draught.values();
        assert!(mass_values.len() == draught_values.len(), "mass.len() {} == draught.len() {}", mass_values.len(), draught_values.len());
        let mut result = mass_values.clone();
        draught_values.mul_single(self.water_density);
        result.sub_vec(&draught_values);
        result.mul_single(self.gravity_g);
        log::info!("\t TotalForce mass:{:?} draught:{:?} result:{:?}, mass_sum:{}, draught_mass_sum:{}", mass_values, draught_values, result, mass_values.iter().sum::<f64>(), draught_values.iter().sum::<f64>());
        result
    }
}

#[doc(hidden)]
pub trait ITotalForce {
    fn values(&self) -> Vec<f64>;
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
    fn values(&self) -> Vec<f64> {
        self.data.clone()
    }
}
