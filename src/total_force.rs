//! Результирующая нагрузка на шпацию
use std::rc::Rc;

use crate::{draught::IDraught, mass::IMass, math::vec::{MultipleSingle, SubVec}};

/// Результирующей нагрузка на шпацию, вычисляется
/// суммированием силы выталкивания воды и суммарной  
/// нагрузки на судно
pub struct TotalForce {
    /// нагрузка на судно
    mass: Rc<dyn IMass>,
    /// масса вытесненной воды
    draught: Box<dyn IDraught>,
    /// ускорение свободного падения
    gravity_g: f64,
}
///
impl TotalForce {
    ///
    pub fn new(mass: Rc<dyn IMass>, draught: impl IDraught + 'static, gravity_g: f64) -> Self {
        assert!(gravity_g > 0., "gravity_g {gravity_g} > 0.");
        Self {
            mass,
            draught: Box::new(draught),
            gravity_g,
        }
    }
}
///
impl ITotalForce for TotalForce {
    ///
    fn values(&self) -> Vec<f64> {
        let mut mass_values = self.mass.values();
        let draught_values = self.draught.values();
        assert!(mass_values.len() == draught_values.len(), "mass.len() {} == draught.len() {}", mass_values.len(), draught_values.len());
        mass_values.sub_vec(&draught_values);
        mass_values.mul_single(self.gravity_g);
        log::debug!("\t TotalForce result:{:?}", mass_values);
        mass_values
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
