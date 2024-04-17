//! Класс для расчета прочности

use crate::{math::Bounds, mass::IMass,};

use super::{    bending_moment::BendingMoment,
    displacement::Displacement,
    volume::Volume,
    
    shear_force::{IShearForce, ShearForce},
    total_force::TotalForce,
};
use std::rc::Rc;

/// Класс для расчета прочности, вычисляет дифферент подбором
pub struct Computer {
    /// Ускорение свободного падения
    gravity_g: f64,
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
    /// Вычисленное распределение изгибающего момента
    bending_moment: Option<Vec<f64>>,
    /// Вычисленное распределение срезающей силы
    shear_force: Option<Vec<f64>>,
}
///
impl Computer {
    /// Основной конструктор
    pub fn new(
        gravity_g: f64,                 // Ускорение свободного падения
        water_density: f64,             // Плотность воды
        center_waterline_shift: f64,    // Отстояние центра величины погруженной части судна
        mean_draught: f64,              // Средняя осадка
        mass: Rc<dyn IMass>, // Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
        displacement: Rc<Displacement>, // Распределение осадки
        bounds: Rc<Bounds>,  // Вектор разбиения судна на отрезки
    ) -> Self {
        Self {
            gravity_g,
            water_density,
            center_waterline_shift,
            mean_draught,
            mass,
            displacement,
            bounds,
            bending_moment: None,
            shear_force: None,
        }
    }
    /// Вычисленное распределение изгибающего момента
    pub fn bending_moment(&mut self) -> Vec<f64> {
        if self.bending_moment.is_none() {
            self.calculate();
        }
        self.bending_moment
            .clone()
            .expect("Computer bending_moment error: no values")
    }
    /// Вычисленное распределение срезающей силы
    pub fn shear_force(&mut self) -> Vec<f64> {
        if self.shear_force.is_none() {
            self.calculate();
        }
        self.shear_force
            .clone()
            .expect("Computer shear_force error: no values")
    }
    /// Вычисление изгибающего момента и срезающей силы. Дифферент  
    /// подбирается перебором.
    fn calculate(&mut self) {
        let mut trim = 0.; // Дифферент
        let mut delta = 1.; // Изменение дифферента
        let mut shear_force_values = None;
        let mut bending_moment_values = None;
        for _i in 0..30 {
            let mut shear_force = ShearForce::new(TotalForce::new(
                Rc::clone(&self.mass),
                self.water_density,
                Volume::new(
                    self.center_waterline_shift,
                    self.mean_draught,
                    Rc::clone(&self.displacement),
                    trim,
                    Rc::clone(&self.bounds),
                ),
                self.gravity_g,
            ));
            shear_force_values = Some(shear_force.values());
            let tmp = BendingMoment::new(Box::new(shear_force), self.bounds.delta()).values();
            // Последнее значение изгибающего момента в векторе.
            // Если корабль сбалансирован, должно равняться нулю            
            let last_value = tmp.last().expect("BendingMoment values error: no last value").clone();
            bending_moment_values = Some(tmp);
   //         log::info!("Computing Trim: BendingMoment last value:{last_value} trim:{trim} i:{i} delta:{delta} ");
            if last_value.abs() < 0.1 {
                break;
            }
            trim -= last_value.signum() * delta;
            delta *= 0.5;
        }
        self.shear_force = shear_force_values;
        self.bending_moment = bending_moment_values;
    }
}
