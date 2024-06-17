//! Класс для расчета дифферента в расчете прочности

use crate::{mass::IMass, math::Bounds, IVolume};

use super::{
    displacement::Displacement,
    volume::Volume,
};
use std::rc::Rc;

/// Класс для расчета дифферента в расчете прочности метором перебора
/// Используются только эпюра масс и Бонжан. Данные по остойчивости не используются.
pub struct Trim {
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
    /// * water_density - Плотность воды
    /// * center_waterline_shift - Отстояние центра величины погруженной части судна
    /// * mean_draught - Средняя осадка
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * displacement - Распределение осадки
    /// * bounds - Вектор разбиения судна на отрезки
    pub fn new(
        water_density: f64,   
        center_waterline_shift: f64,
        mean_draught: f64,
        mass: Rc<dyn IMass>, 
        displacement: Rc<Displacement>, 
        bounds: Rc<Bounds>, 
    ) -> Self {
        Self {
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
    fn calc_s(&self, values: &Vec<(f64, f64)>) -> (f64, f64) {
        let l = self.bounds.delta();
        let mut sum_s = 0.;
        let mut xc = 0.;
        for i in 0..values.len()-1 {
            let x_i1 = values[i].0;
            let x_i2 = values[i+1].0;
            let y_i1 = values[i].1; 
            let y_i2 = values[i+1].1;             
            let x_ci = x_i1 + (l / 3.) * ((2. * y_i2 + y_i1) / (y_i2 + y_i1));
            let s_i = ((y_i2 + y_i1) / 2.) * (x_i2 - x_i1);
            sum_s += s_i;
            xc += s_i * x_ci;
        }
        xc /= sum_s;
        (xc, sum_s)
    }
}
/// 
impl ITrim for Trim {
    /// Вычисление дифферента перебором
    fn value(&mut self) -> f64 {
        let dx = self.bounds.iter().map(|v| v.center()).collect::<Vec<_>>();
        let mass_pairs = dx.clone().into_iter().zip(self.mass.values()).collect::<Vec<_>>();
        let (w_xg, w) = self.calc_s(&mass_pairs);
        let mut trim = 0.; // Дифферент
        let mut mean_draught = self.mean_draught;
        for _i in 0..30 {
            let mut delta_w;
            let mut volume_values= Vec::new();
            for _j in 0..30 {
                volume_values = Volume::new(
                    self.center_waterline_shift,
                    mean_draught,
                    Rc::clone(&self.displacement),
                    trim,
                    Rc::clone(&self.bounds),
                ).values();
                delta_w = (w - volume_values.iter().sum::<f64>()/self.water_density)/w;                
                if delta_w.abs() <= 0.0001 {
                    break;
                }
                mean_draught += self.mean_draught*delta_w;                
            }
            let volume_pairs = dx.clone().into_iter().zip(volume_values).collect::<Vec<_>>();
            let (v_xc, _) = self.calc_s(&volume_pairs);
            let delta_x = w_xg - v_xc;
            if delta_x.abs() <= 0.002 {
                break;
            }     
            trim = trim + (w_xg - v_xc) / 2.;
        }
        trim
    }
}

#[doc(hidden)]
pub trait ITrim {
    /// Вычисление дифферента перебором
    fn value(&mut self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeTrim {
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeTrim {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}
#[doc(hidden)]
impl ITrim for FakeTrim {
    fn value(&mut self) -> f64 {
        self.value
    }
}

