//! Класс для расчета дифферента и средней осадки в расчете прочности

use crate::{mass::IMass, math::Bounds, trim::ITrim, IVolume};

use super::{
    displacement::Displacement,
    volume::Volume,
};
use std::rc::Rc;

/// Класс для расчета дифферента и средней осадки в расчете прочности метором перебора
/// Используются только эпюра масс и Бонжан. Данные по остойчивости не используются.
pub struct Trim {
    /// длинна судна
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
    /// Осадка на миделе в ДП, м
    draught_mid: Option<f64>,
    /// Изменение осадки
    delta_draught: Option<f64>,
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
            draught_mid: None,
            delta_draught: None,
        }
    }
    /// Вычисление суммы площади и смещения центра методом трапеций
    /// * values - Vec(x, value)>
    /// * result - (delta_x, sum_s)
    fn calc_s_trap(&self, values: &Vec<(f64, f64)>) -> (f64, f64) {
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
    }
    /// Вычисление суммы площади и смещения центра
    /// * values - Vec(x, value)>
    /// * result - (delta_x, sum_s)
    fn calc_s(&self, values: &Vec<(f64, f64)>) -> (f64, f64) {
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
    /// Вычисление дифферента и средней осадки перебором
    fn calculate(&mut self) {
        let dx = self.bounds.iter().map(|v| v.center()).collect::<Vec<_>>();
        let mass_pairs = dx.clone().into_iter().zip(self.mass.values()).collect::<Vec<_>>();
        let (w_xg, w) = self.calc_s(&mass_pairs);
        let mut trim = 0.; // Дифферент
        let mut mean_draught = self.mean_draught;
        let (mut v_xc, mut volume) = (0., 0.);
        for _i in 0..50 {
            mean_draught = self.mean_draught;
            for _j in 0..50 {
                // Осадка на носовом перпендикуляре длины L в ДП dн, м (6)
                let draught_bow = mean_draught + (0.5 - self.center_waterline_shift/self.ship_length)*trim;
                // Осадка на кормовом перпендикуляре длины L в ДП dк, м (7)
                let draught_stern = mean_draught - (0.5 + self.center_waterline_shift/self.ship_length)*trim;
                // Осадка на миделе в ДП, м (8)
                let draught_mid = (draught_bow + draught_stern) / 2.;
                // Изменение осадки
                let delta_draught = (draught_stern - draught_bow) / self.ship_length;
                let volume_values = Volume::new(
                    Rc::clone(&self.displacement),
                    Box::new(FakeTrim::new()),
                    Rc::clone(&self.bounds),
                ).values();
                let volume_pairs = dx.clone().into_iter().zip(volume_values).collect::<Vec<_>>();
                (v_xc, volume) = self.calc_s(&volume_pairs);
                let delta_w = (w - volume*self.water_density)/w;              
                if delta_w.abs() <= 0.000000001 {
                    break;
                }         
                mean_draught = 0.001_f64.max(mean_draught + mean_draught*delta_w);   
//                dbg!(_j, trim, mean_draught, v_xc, volume*self.water_density, w, delta_w, );             
            }
            let delta_x = w_xg - v_xc;
//            dbg!(_i, trim, mean_draught, v_xc, w_xg, w, delta_x, );
            if delta_x.abs() <= 0.000000001 {
                break;
            }                 
            trim = trim + delta_x / 10.;
        }
        // Осадка на носовом перпендикуляре длины L в ДП dн, м (6)
        let draught_bow = mean_draught + (0.5 - self.center_waterline_shift/self.ship_length)*trim;
        // Осадка на кормовом перпендикуляре длины L в ДП dк, м (7)
        let draught_stern = mean_draught - (0.5 + self.center_waterline_shift/self.ship_length)*trim;
        // Осадка на миделе в ДП, м (8)
        let draught_mid = (draught_bow + draught_stern) / 2.;
        // Изменение осадки
        let delta_draught = (draught_stern - draught_bow) / self.ship_length;
        //
        self.draught_mid = Some(draught_mid);
        self.delta_draught = Some(delta_draught);
    }
}
/// 
impl ITrim for Trim {
    /// Вычисление дифферента и средней осадки перебором
    fn value(&mut self, pos_x: f64) -> f64 {
        if self.draught_mid.is_none() {
            self.calculate();
        }        
        self.draught_mid.expect("strength::Trim value error: no draught_mid!")
            + self.delta_draught.expect("strength::Trim value error: no draught_mid!") 
            * pos_x
    }
}
