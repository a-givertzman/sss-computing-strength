//! Распределение осадки по шпациям
use crate::{
    displacement::Displacement,
    math::{bound::Bound, curve::Curve},
    trim::Trim,
};
///
/// Распределение осадки по шпациям
pub struct Draught {
    /// дифферент судна
    trim: f64,
    /// водоизмещение судна
    displacement: Displacement,
    /// длинна судна
    ship_length: f64,
    /// вектор разбиения на отрезки для эпюров
    bounds: Vec<Bound>,
    /// отстояние центра тяжести ватерлинии по длине от миделя
    center_waterline_shift: f64,
    /// средняя осадка
    mean_draught: f64,
}
///
impl Draught {
    /// Основной конструктор
    pub fn new(
        ship_length: f64,            // длинна судна
        bounds: Vec<Bound>,          // вектор разбиения на отрезки для эпюров
        center_waterline_shift: f64, // отстояние центра тяжести ватерлинии по длине от миделя
        mean_draught: f64,           // средняя осадка
        displacement: Displacement,  // водоизмещение судна
        trim: f64,                   // дифферент судна
    ) -> Self {
        Self {
            ship_length,
            bounds,
            center_waterline_shift,
            mean_draught,
            displacement,
            trim,
        }
    }
    /// Конструктор, использующий класс Trim диффеерента судна для вычисления 
    /// значений.  Аргументы:  
    /// - ship_length: длинна судна
    /// - volume: объемное водоизмещение
    /// - bounds: вектор разбиения на отрезки для эпюров
    /// - center_waterline_shift: кривая отстояния центра тяжести ватерлинии по длине от миделя
    /// - mean_draught: кривая средняй осадки
    /// - displacement: класс водоизмещения судна
    /// - trim: класс дифферента судна
    pub fn from_trim(
        ship_length: f64,              // длинна судна
        volume: f64,                   //объемное водоизмещение
        bounds: Vec<Bound>,            // ссылка на вектор разбиения на отрезки для эпюров
        center_waterline_shift: Curve, // отстояние центра тяжести ватерлинии по длине от миделя
        mean_draught: Curve,           // средняя осадка
        displacement: Displacement,
        trim: Trim, // дифферент судна
    ) -> Self {
        Self::new(
            ship_length,
            bounds,
            center_waterline_shift.value(volume),
            mean_draught.value(volume),
            displacement,
            trim.value(),
        )
    }
    /// Распределение осадки по шпациям
    pub fn values(&self) -> Vec<f64> {
        //отстояние центра тяжести ватерлинии по длине от миделя
        let x_f = self.center_waterline_shift;
        //средняя осадка
        let d = self.mean_draught;
        //осадка на носовом перпендикуляре
        //let stern_draught = d - (0.5 - x_f/self.ship_length)*trim;
        //осадка на кормовом перпендикуляре
        let bow_draught = d - (0.5 + x_f / self.ship_length) * self.trim;
        //let delta_draught = (bow_draught - stern_draught)/self.bounds.len() as f64;
        //self.bounds.iter().map(|v| self.displacement.value(*v, delta_draught*(v.center() + self.ship_length/2.)/self.ship_length)).collect()
        let trim_x_f_sl = x_f * self.trim / self.ship_length;
        let delta_draught = (-2. * trim_x_f_sl) / (self.bounds.len() as f64 * self.ship_length);
        let result = self
            .bounds
            .iter()
            .map(|v| {
                {
                    let displacement = self.displacement.value(
                        *v,
                        bow_draught + delta_draught * (v.center() + self.ship_length / 2.),
                    );
                    //         dbg!(displacement);
                    displacement
                }
            })
            .collect();

        //       dbg!(&x_f, &d, &trim, &bow_draught, &trim_x_f_sl, &delta_draught, &result);
        result
    }
}
