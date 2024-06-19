//! Распределение объема вытесненной воды по шпациям
use std::rc::Rc;
use crate::math::Bounds;
use super::displacement::Displacement;

///
/// Распределение объема вытесненной воды по шпациям
pub struct Volume {
    /// вектор разбиения на отрезки для эпюров
    bounds: Rc<Bounds>,
    /// отстояние центра тяжести ватерлинии по длине от миделя
    center_waterline_shift: f64,
    /// средняя осадка
    mean_draught: f64,
    /// водоизмещение судна
    displacement: Rc<Displacement>,
    /// дифферент судна
    trim: f64,//Trim,
}
///
impl Volume {
    /// Основной конструктор. Аргументы:  
    /// - bounds: вектор разбиения на отрезки для эпюров
    /// - center_waterline_shift: отстояние центра тяжести ватерлинии по длине от миделя, м
    /// - mean_draught: средняя осадка, м
    /// - displacement: водоизмещение судна, м^3
    /// - trim: дифферент судна, м
    pub fn new(    
        center_waterline_shift: f64, 
        mean_draught: f64,          
        displacement: Rc<Displacement>,   
        trim: f64,
        bounds: Rc<Bounds>,
    ) -> Self {
        Self {
            bounds,
            center_waterline_shift,
            mean_draught,
            displacement,
            trim,
        }
    }
}
///
impl IVolume for Volume {
    /// Распределение объема вытесненной воды по шпациям
    fn values(&mut self) -> Vec<f64> {
        // длинна судна
        let ship_length = self.bounds.length();
        // дифферент судна
        let trim = self.trim;//.value();
        //отстояние центра тяжести ватерлинии по длине от миделя
        let x_f = self.center_waterline_shift;
        //средняя осадка
        let d = self.mean_draught;
        //осадка на носовом перпендикуляре (6)
        let stern_draught = d + (0.5 - x_f/ship_length) * trim;
        //осадка на кормовом перпендикуляре (7)
        let bow_draught = d - (0.5 + x_f/ship_length) * trim;
        //изменение осадки
        let delta_draught = (stern_draught - bow_draught) / self.bounds.length();
        let result: Vec<f64> = self
            .bounds
            .iter()
            .map(|v| {
                self.displacement.value(
                    *v,
                    d + delta_draught * v.start(),
                    d + delta_draught * v.end(),
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
