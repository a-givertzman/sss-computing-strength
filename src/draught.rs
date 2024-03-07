//! Распределение массы вытесненной воды по шпациям
use std::rc::Rc;

use crate::{displacement::Displacement, mass::IMass, math::*, trim::Trim};
///
/// Распределение массы вытесненной воды по шпациям
pub struct Draught {
    /// плотность окружающей воды
    water_density: f64,
    /// вектор разбиения на отрезки для эпюров
    bounds: Bounds,
    /// объемное водоизмещение
    mass: Rc<dyn IMass>,
    /// отстояние центра тяжести ватерлинии по длине от миделя
    center_waterline_shift: Curve,
    /// средняя осадка
    mean_draught: Curve,
    /// водоизмещение судна
    displacement: Displacement,
    /// дифферент судна
    trim: Trim,
}
///
impl Draught {
    /// Основной конструктор. Аргументы:  
    /// - water_density: плотность окружающей воды    
    /// - bounds: вектор разбиения на отрезки для эпюров
    /// - mass: все грузы судна
    /// - center_waterline_shift: кривая отстояния центра тяжести ватерлинии по длине от миделя
    /// - mean_draught: кривая средняй осадки
    /// - displacement: класс водоизмещения судна
    /// - trim: класс дифферента судна
    pub fn new(
        water_density: f64,            // плотность окружающей воды        
        mass: Rc<dyn IMass>,           // все грузы судна
        center_waterline_shift: Curve, // отстояние центра тяжести ватерлинии по длине от миделя
        mean_draught: Curve,           // средняя осадка
        displacement: Displacement,    // водоизмещение судна
        trim: Trim,                    // дифферент судна
        bounds: Bounds,                // вектор разбиения на отрезки для эпюров
    ) -> Self {
        Self {
            water_density,
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
    /// Распределение массы вытесненной воды по шпациям
    fn values(&self) -> Vec<f64> {
        // длинна судна
        let ship_length = self.bounds.length();
        // дифферент судна
        let trim = self.trim.value();
        //объемное водоизмещение
        let volume = self.mass.sum() / self.water_density;
        //отстояние центра тяжести ватерлинии по длине от миделя
        let x_f = self.center_waterline_shift.value(volume);
        //средняя осадка
        let d = self.mean_draught.value(volume);
        //осадка на носовом перпендикуляре
        //let stern_draught = d - (0.5 - x_f/self.ship_length)*trim;
        //осадка на кормовом перпендикуляре
        let bow_draught = d - (0.5 + x_f / ship_length) * trim;
        //let delta_draught = (bow_draught - stern_draught)/self.bounds.len() as f64;
        //self.bounds.iter().map(|v| self.displacement.value(*v, delta_draught*(v.center() + self.ship_length/2.)/self.ship_length)).collect()
        let trim_x_f_sl = x_f * trim / ship_length;
        let delta_draught = (-2. * trim_x_f_sl) / (self.bounds.qnt() as f64 * ship_length);
        let result = self
            .bounds
            .iter()
            .map(|v| {
                let displacement = self.displacement.value(
                    *v,
                    bow_draught + delta_draught * (v.center() + ship_length / 2.),
                );
                displacement * self.water_density
            })
            .collect();
        log::debug!("\t Draught trim:{trim} volume:{volume} x_f:{x_f} d:{d} bow_draught:{bow_draught} trim_x_f_sl:{trim_x_f_sl} delta_draught:{delta_draught} result:{:?}", result);
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
