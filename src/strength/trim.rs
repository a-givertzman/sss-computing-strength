//! Класс для расчета дифферента и средней осадки в расчете прочности

use crate::{draught::Draught, math::Bounds, trim::{FakeTrim, ITrim}, IVolume, MultipleSingle};

use super::{
    displacement::Displacement,
    volume::Volume, IMass,
};
use std::rc::Rc;

/// Класс для расчета дифферента и средней осадки в расчете прочности метором перебора
/// Используются только эпюра масс и Бонжан. Данные по остойчивости не используются.
pub struct Trim {
    /// Длинна судна
    ship_length: f64,
    /// Плотность воды
    water_density: f64,
    /// Отстояние центра величины погруженной части судна
    center_waterline_shift: f64,
    /// Средняя осадка
    mean_draught: f64,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    mass: Rc<dyn IMass>,
    /// Распределение осадки
    displacement: Rc<Displacement>,
    /// Вектор разбиения судна на отрезки
    bounds: Rc<Bounds>,
}
///
impl Trim {
    /// Основной конструктор
    /// * ship_length - длинна судна
    /// * water_density - Плотность воды
    /// * center_waterline_shift - Отстояние центра величины погруженной части судна
    /// * mean_draught - Средняя осадка
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * displacement - Распределение осадки
    /// * bounds - Вектор разбиения судна на отрезки
    pub fn new(
        ship_length: f64, 
        water_density: f64,   
        center_waterline_shift: f64,
        mean_draught: f64,
        mass: Rc<dyn IMass>, 
        displacement: Rc<Displacement>, 
        bounds: Rc<Bounds>, 
    ) -> Self {
        Self {
            ship_length,
            water_density,
            center_waterline_shift,
            mean_draught,
            mass,
            displacement,
            bounds,
        }
    }
    /// Вычисление суммы площади и смещения центра методом трапеций
    /// * values - Vec(x, value)>
    /// * result - (delta_x, sum_s)
   /* fn calc_s_trap(&self, values: &Vec<(f64, f64)>) -> (f64, f64) {
        let l = self.bounds.delta();
        let mut sum_s = 0.;
        let mut xc = 0.;
        for i in 0..values.len()-1 {
            let x_i1 = values[i].0;
        //    let x_i2 = values[i+1].0;
            let y_i1 = values[i].1/l; 
            let y_i2 = values[i+1].1/l;             
            let x_ci = x_i1 + (l / 3.) * ((2. * y_i2 + y_i1) / (y_i2 + y_i1));
            let s_i = ((y_i2 + y_i1) / 2.) * l;
            sum_s += s_i;
            xc += s_i * x_ci;
        }
        xc /= sum_s;
        (xc, sum_s)
    }*/
    /// Вычисление суммы площади и смещения центра
    /// * values - Vec(x, value)>
    /// * result - (delta_x, sum_s)
    pub fn calc_s(values: &Vec<(f64, f64)>) -> (f64, f64) {
        let mut sum_s = 0.;
        let mut xc = 0.;
        for i in 0..values.len() {
            let x_i = values[i].0;
            let y_i = values[i].1; 
            sum_s += y_i;
            xc += y_i * x_i;
        }
        if sum_s > 0. {
            xc /= sum_s;
        } else {
            xc = 0.;
            sum_s = 0.;
        };
        (xc, sum_s)
    }
}
/// 
impl ITrim for Trim {
    /// Вычисление средней осадки и дифферента
    fn value(&self) -> (f64, f64) {
        let dx = self.bounds.iter().map(|v| v.center()).collect::<Vec<_>>();
        let mass_pairs = dx.clone().into_iter().zip(self.mass.values()).collect::<Vec<_>>();
        let (w_xg, w) = Trim::calc_s(&mass_pairs);
        let mut trim = 0.; // Дифферент
        let mut mean_draught: f64 = self.mean_draught;
        let (mut v_xc, mut volume_mass) = (0., 0.);
        for _i in 0..50 {
            mean_draught = self.mean_draught;
            for _j in 0..50 {
                let mut volume_values = Volume::new(
                    Rc::clone(&self.displacement),
                    Box::new(Draught::new(
                        self.ship_length,           
                        self.center_waterline_shift,    
                        Box::new(FakeTrim::new(mean_draught, trim)),               
                        None,                       
                    )),
                    Rc::clone(&self.bounds),
                ).values();
                volume_values.mul_single(self.water_density);
    //            dbg!(&volume_values);
                let volume_pairs = dx.clone().into_iter().zip(volume_values).collect::<Vec<_>>();
                (v_xc, volume_mass) = Trim::calc_s(&volume_pairs);
                let delta_w = (w - volume_mass)/w;              
                if delta_w.abs() <= 0.000000001 {
                    break;
                }         
                mean_draught = 0.001_f64.max(mean_draught + mean_draught*delta_w);   
    //            dbg!(_j, trim, mean_draught, v_xc, volume_mass, w, delta_w, );             
            }
            let delta_x = w_xg - v_xc;
   //         dbg!(_i, trim, mean_draught, v_xc, w_xg, w, delta_x, );
            if delta_x.abs() <= 0.000000001 {
    //            dbg!("delta_x.abs() <= 0.000000001");
                break;
            }                 
            trim = trim + delta_x / 10.;
        }
        (mean_draught, trim)
    }
}

