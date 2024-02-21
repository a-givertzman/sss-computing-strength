use crate::{
    displacement::Displacement,
    math::{bound::Bound, curve::Curve},
    trim::Trim,
};

///класс реализующий распределение осадки
pub struct Draught {
    trim: Trim, // дифферент судна
    displacement: Displacement,
    ship_length: f64,        // длинна судна
    bounds: Vec<Bound>,  // ссылка на вектор разбиения на отрезки для эпюров
    center_waterline_shift: Curve, // отстояние центра тяжести ватерлинии по длине от миделя
    mean_draught: Curve,     // средняя осадка
}

impl Draught {
    ///
    pub fn new(
        trim: Trim, // дифферент судна
        displacement: Displacement,
        ship_length: f64,        // длинна судна
        bounds: Vec<Bound>,  // ссылка на вектор разбиения на отрезки для эпюров
        center_waterline_shift: Curve, // отстояние центра тяжести ватерлинии по длине от миделя
        mean_draught: Curve,     // средняя осадка
    ) -> Self {
        Self {
            trim,
            displacement,
            ship_length,
            bounds,
            center_waterline_shift,
            mean_draught,
        }
    }
    ///распределение осадки, м
    pub fn values(&self) -> Vec<f64> {
        //отстояние центра тяжести ватерлинии по длине от миделя
        let x_f = self.center_waterline_shift.value(self.trim.volume());
        //средняя осадка
        let d = self.mean_draught.value(self.trim.volume());
        //дифферент судна
        let trim = self.trim.value();
        //осадка на носовом перпендикуляре
        //let stern_draught = d - (0.5 - x_f/self.ship_length)*trim;
        //осадка на кормовом перпендикуляре
        let bow_draught = d - (0.5 + x_f / self.ship_length) * trim;
        //let delta_draught = (bow_draught - stern_draught)/self.bounds.len() as f64;
        //self.bounds.iter().map(|v| self.displacement.value(*v, delta_draught*(v.center() + self.ship_length/2.)/self.ship_length)).collect()

        let trim_x_f_sl = x_f * trim / self.ship_length;
        let delta_draught = (-2. * trim_x_f_sl) / (self.bounds.len() as f64 * self.ship_length);
        let result = self.bounds
            .iter()
            .map(|v| { {              
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
