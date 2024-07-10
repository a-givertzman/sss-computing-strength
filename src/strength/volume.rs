//! Распределение объема вытесненной воды по шпациям
use std::rc::Rc;
use crate::{draught::IDraught, math::Bounds};
use super::displacement::Displacement;

///
/// Распределение объема вытесненной воды по шпациям
pub struct Volume {
    /// вектор разбиения на отрезки для эпюров
    bounds: Rc<Bounds>,
    /// водоизмещение судна
    displacement: Rc<Displacement>,
    /// Осадка судна
    draught: Box<dyn IDraught>,
}
///
impl Volume {
    /// Основной конструктор. Аргументы:  
    /// * bounds - Вектор разбиения на отрезки для эпюров
    /// * displacement - Водоизмещение судна, м^3
    /// * draught - Осадка судна
    pub fn new(  
        displacement: Rc<Displacement>,   
        draught: Box<dyn IDraught>,
        bounds: Rc<Bounds>,
    ) -> Self {
        Self {
            bounds,
            draught,            
            displacement,
        }
    }
}
///
impl IVolume for Volume {
    /// Распределение объема вытесненной воды по шпациям
    fn values(&mut self) -> Vec<f64> {
        let result: Vec<f64> = self
            .bounds
            .iter()
            .map(|v| {
                self.displacement.value(
                    *v,
                    self.draught.value(v.start()),
                    self.draught.value(v.end()),
                )
            })
            .collect();
  //      log::info!("\t Volume ship_length:{ship_length} trim:{trim} x_f:{x_f} d:{d} stern_draught:{stern_draught} bow_draught:{bow_draught} delta_draught:{delta_draught} result:{:?}, res_sum:{}", result, result.iter().sum::<f64>());
//            log::info!("\t Volume ship_length:{ship_length} trim:{trim} x_f:{x_f} d:{d} stern_draught:{stern_draught} bow_draught:{bow_draught} delta_draught:{delta_draught} res_sum:{}", result.iter().sum::<f64>());
        result
    }
}

#[doc(hidden)]
pub trait IVolume {
    fn values(&mut self) -> Vec<f64>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeVolume {
    data: Vec<f64>,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeVolume {
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }
}
#[doc(hidden)]
impl IVolume for FakeVolume {
    fn values(&mut self) -> Vec<f64> {
        self.data.clone()
    }
}
