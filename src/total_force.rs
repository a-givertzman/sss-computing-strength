//! Результирующая нагрузка на шпацию
use crate::{
    draught::IDraught,
    mass::IMass,
    math::vec::{MultipleSingle, SubVec},
};
use std::rc::Rc;
///
/// Результирующая нагрузка на шпацию. Вычисляется
/// суммированием сил действующих на шпацию.
pub struct TotalForce {
    /// Распределение всей нагрузки на судно
    mass: Rc<dyn IMass>,
    /// Распределение массы вытесненной воды
    draught: Box<dyn IDraught>,
    /// Ускорение свободного падения
    gravity_g: f64,
}
///
impl TotalForce {
    /// Основной конструктор
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
    /// Распределение результирующей силы. Вычисляется как сумма массы выталкивания воды и
    /// суммарной массы грузов, приходящихся на шпацию помноженное на ускорение свободного
    /// падения.
    fn values(&self) -> Vec<f64> {
        let mut mass_values = self.mass.values();
        let draught_values = self.draught.values();
        assert!(
            mass_values.len() == draught_values.len(),
            "mass.len() {} == draught.len() {}",
            mass_values.len(),
            draught_values.len()
        );
        mass_values.sub_vec(&draught_values);
        mass_values.mul_single(self.gravity_g);
        log::debug!("\t TotalForce result:{:?}", mass_values);
        mass_values
    }
}
///
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
