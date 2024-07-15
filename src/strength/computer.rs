//! Класс для расчета прочности

use crate::{draught::Draught, math::Bounds, IResults, ITotalForce, IVolume};

use super::{
    bending_moment::BendingMoment, displacement::Displacement, shear_force::{IShearForce, ShearForce}, total_force::TotalForce, volume::Volume, IMass, Trim
};
use std::rc::Rc;

/// Класс для расчета прочности, вычисляет дифферент подбором
pub struct Computer {
    /// Ускорение свободного падения
    gravity_g: f64,
    /// Плотность воды
    water_density: f64,
    /// Длинна судна
    ship_length: f64,
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
    /// Набор результатов расчетов для записи в БД
    results: Rc<dyn IResults>, 
}
///
impl Computer {
    /// Основной конструктор  
    /// * gravity_g - Ускорение свободного падения  
    /// * water_density - Плотность воды  
    /// * ship_length - длинна судна
    /// * center_waterline_shift - Отстояние центра величины погруженной части судна  
    /// * mean_draught - Средняя осадка
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.  
    /// * displacement - Распределение осадки  
    /// * bounds - Вектор разбиения судна на отрезки  
    /// * results - Набор результатов расчетов для записи в БД  
    pub fn new(
        gravity_g: f64,                 
        water_density: f64,   
        ship_length: f64,           
        center_waterline_shift: f64,    
        mean_draught: f64,              
        mass: Rc<dyn IMass>, 
        displacement: Rc<Displacement>, 
        bounds: Rc<Bounds>,  
        results: Rc<dyn IResults>, 
    ) -> Self {
        Self {
            gravity_g,
            water_density,
            ship_length,
            center_waterline_shift,
            mean_draught,
            mass,
            displacement,
            bounds,
            results,
        }
    }
    /// Вычисление изгибающего момента и срезающей силы. Дифферент  
    /// подбирается перебором.
    pub fn calculate(&mut self) {
        let mut displacement_values;
        let mut total_force_values;
        let shear_force_values;
        let bending_moment_values;
        let mut volume = Volume::new(
            Rc::clone(&self.displacement),
            Box::new(Draught::new(
                self.ship_length,
                self.center_waterline_shift,
                Box::new(Trim::new(
                    self.ship_length, 
                    self.water_density,   
                    self.center_waterline_shift,
                    self.mean_draught,
                    Rc::clone(&self.mass),
                    Rc::clone(&self.displacement),
                    Rc::clone(&self.bounds),
                )),
                None,
            )),
            Rc::clone(&self.bounds),
        );
        displacement_values = volume.values();
        displacement_values.push(displacement_values.iter().sum());
        let mut total_force = TotalForce::new(
            Rc::clone(&self.mass),
            self.water_density,
            volume,
            self.gravity_g,
        );
        total_force_values = total_force.values();
        total_force_values.push(total_force_values.iter().sum());
        let mut shear_force = ShearForce::new(total_force);
        shear_force_values = shear_force.values();
        bending_moment_values = BendingMoment::new(Box::new(shear_force), self.bounds.delta()).values();  
        self.results.add("value_displacement".to_owned(), displacement_values);
        self.results.add("value_total_force".to_owned(), total_force_values);
        self.results.add("value_shear_force".to_owned(), shear_force_values);
        self.results.add("value_bending_moment".to_owned(), bending_moment_values);
    }

    /* 
    /// Вычисление изгибающего момента и срезающей силы. Дифферент  
    /// подбирается перебором.
    fn calculate(&mut self) {
        let mut trim = 0.; // Дифферент
        let mut delta = 1.; // Изменение дифферента
        let mut displacement_values = Vec::new();
        let mut total_force_values = Vec::new();
        let mut shear_force_values = Vec::new();
        let mut bending_moment_values = Vec::new();
        for _i in 0..30 {
            let mut volume = Volume::new(
                self.center_waterline_shift,
                self.mean_draught,
                Rc::clone(&self.displacement),
                trim,
                Rc::clone(&self.bounds),
            );
            displacement_values = volume.values();
            let mut total_force = TotalForce::new(
                Rc::clone(&self.mass),
                self.water_density,
                volume,
                self.gravity_g,
            );
            total_force_values = total_force.values();
            let mut shear_force = ShearForce::new(total_force);
            shear_force_values = shear_force.values();
            bending_moment_values = BendingMoment::new(Box::new(shear_force), self.bounds.delta()).values();
            // Последнее значение изгибающего момента в векторе.
            // Если корабль сбалансирован, должно равняться нулю
            let last_value = *bending_moment_values
                .last()
                .expect("BendingMoment values error: no last value");
            //         log::info!("Computing Trim: BendingMoment last value:{last_value} trim:{trim} i:{i} delta:{delta} ");
            if self.mass.sum() <= 1. || last_value.abs() < 0.1 {
                break;
            }
            trim -= last_value.signum() * delta;
            delta *= 0.5;
        }
        self.mass_values = Some(self.mass.values());
        self.displacement_values = displacement_values;
        self.total_force_values = total_force_values;
        self.shear_force = shear_force_values;
        self.bending_moment = bending_moment_values;
    } */
}
