//! Распределение объема вытесненной воды по шпациям
use std::rc::Rc;

use crate::{displacement::Displacement, mass::IMass, math::*, trim::Trim};
///
/// Распределение объема вытесненной воды по шпациям
pub struct Draught {
    /// вектор разбиения на отрезки для эпюров
    bounds: Bounds,
    /// объемное водоизмещение
    mass: Rc<dyn IMass>,
    /// отстояние центра тяжести ватерлинии по длине от миделя
    center_waterline_shift: f64,
    /// средняя осадка
    mean_draught: f64,
    /// водоизмещение судна
    displacement: Displacement,
    /// дифферент судна
    trim: Trim,
}
///
impl Draught {
    /// Основной конструктор. Аргументы:  
    /// - bounds: вектор разбиения на отрезки для эпюров
    /// - mass: все грузы судна
    /// - center_waterline_shift: кривая отстояния центра тяжести ватерлинии по длине от миделя
    /// - mean_draught: кривая средняй осадки
    /// - displacement: класс водоизмещения судна
    /// - trim: класс дифферента судна
    pub fn new(    
        mass: Rc<dyn IMass>,           // все грузы судна
        center_waterline_shift: f64, // отстояние центра тяжести ватерлинии по длине от миделя
        mean_draught: f64,           // средняя осадка
        displacement: Displacement,    // водоизмещение судна
        trim: Trim,                    // дифферент судна
        bounds: Bounds,                // вектор разбиения на отрезки для эпюров
    ) -> Self {
        Self {
            bounds,
            mass,
            center_waterline_shift,
            mean_draught,
            displacement,
            trim,
        }
    }
}
///
impl IDraught for Draught {
    /// Распределение объема вытесненной воды по шпациям
    fn values(&self) -> Vec<f64> {
        // длинна судна
        let ship_length = self.bounds.length();
        // дифферент судна
        let trim = self.trim.value();
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
                let displacement = self.displacement.value(
                    *v,
                    d + delta_draught * v.start(),
                    d + delta_draught * v.end(),
                );
                displacement
            })
            .collect();
        log::info!("\t Draught ship_length:{ship_length} trim:{trim} x_f:{x_f} d:{d} stern_draught:{stern_draught} bow_draught:{bow_draught} delta_draught:{delta_draught} result:{:?}, res_sum:{}", result, result.iter().sum::<f64>());
        result
    }
}

#[doc(hidden)]
pub trait IDraught {
    fn values(&self) -> Vec<f64>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeDraught {
    data: Vec<f64>,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeDraught {
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }
}
#[doc(hidden)]
impl IDraught for FakeDraught {
    fn values(&self) -> Vec<f64> {
        self.data.clone()
    }
}
