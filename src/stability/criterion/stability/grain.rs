//! Критерий крена от смещения зерна

use core::f64;
use std::{f64::consts::PI, rc::Rc};
use crate::{Error, IBulk, ILeverDiagram, IMass, IParameters, ParameterID};

/// Критерий крена от смещения зерна
pub struct Grain {
    /// Угол заливания отверстий
    flooding_angle: f64, 
    /// Все навалочные смещаемые грузы судна
    loads_bulk: Rc<Vec<Rc<dyn IBulk>>>,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    mass: Rc<dyn IMass>,  
    /// Диаграмма плеч статической и динамической остойчивости
    lever_diagram: Rc<dyn ILeverDiagram>, 
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>, 
    /// Угол крена от смещения зерна
    angle: Option<(f64, f64)>,   
    /// Остаточная площадь между кривой кренящих и
    /// кривой восстанавливающих плеч
    area: Option<f64>,   
}
/// 
impl Grain {
    /// Основной конструктор
    /// * flooding_angle - Угол заливания отверстий
    /// * loads_bulk - Все навалочные смещаемые грузы судна
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        flooding_angle: f64, 
        loads_bulk: Rc<Vec<Rc<dyn IBulk>>>,
        mass: Rc<dyn IMass>,  
        lever_diagram: Rc<dyn ILeverDiagram>, 
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            flooding_angle,
            loads_bulk,
            mass,
            lever_diagram,
            parameters,
            angle: None,
            area: None,            
        }
    }
    /// Расчет угла крена и остаточной площади между кривой кренящих и
    /// кривой восстанавливающих плеч
    fn calculate(&mut self) -> Result<(), Error> {
        let m_grain: f64 = self.loads_bulk.iter().map(|v| v.moment() ).sum();    
        let lambda_0 = m_grain/self.mass.sum()?; 
        // Первая точка апроксимирующей прямой
        let first_point_ab = (0.0f64, lambda_0); 
        // Вторая точка апроксимирующей прямой
        let second_point_ab = (40.0f64, 0.8*lambda_0);
        // Изменение апроксимирующей прямой на один градус угла крена
        let delta_ab = (second_point_ab.1 - first_point_ab.1)/(second_point_ab.0 - first_point_ab.0);
        let precision = 0.05; // Точность определения пересечения в градусах
        // Точка пересечения кривых. Проходим по кривой плечей и ищем точку пересечения как  
        // точку, в которой значение кривой плеч момента зерна меньше чем значение dso
        // Если точка отсутствует (момент от зерна слишком большй) то принимаем
        // за точку 90 градусов
        let max_i: f64 = 90./precision;
        let max_i = max_i.ceil() as usize;
        let first_angle = (0..=max_i).find(|i| {
            let i = *i as f64; 
            // значение угла крена в текущей точке
            let angle = i * precision;  
            // значение апроксимирующей прямой плеч момента зерна в текущей точке
            let lever_ab = lambda_0 + delta_ab * angle; 
            // значение восстанавливающего момента в текущей точке
            let lever_dso = self.lever_diagram.lever_moment(angle).unwrap_or(f64::MIN);
            // log::info!("\t Grain area first_angle i:{i} angle:{angle} lever_ab:{lever_ab} lever_dso:{lever_dso}");
            lever_dso >= lever_ab
        }).unwrap_or((90./precision) as usize) as f64 * precision;
        // угол соответствующий максимальной разности между ординатами двух кривых
        let mut angles: Vec<(f64, f64)> = (0..=max_i).map(|i| {
            let i = i as f64; 
            // значение угла крена в текущей точке
            let angle = i * precision;  
            // значение апроксимирующей прямой плеч момента зерна в текущей точке
            let lever_ab = lambda_0 + delta_ab * angle; 
            // значение восстанавливающего момента в текущей точке
            let lever_dso = self.lever_diagram.lever_moment(angle).unwrap_or(lever_ab);
            (angle, lever_dso - lever_ab)
        }).collect();
        angles.sort_by(|v1, v2| v1.1.partial_cmp(&v2.1).expect("Grain calculate cmp error"));
        let angle_delta_max = angles.last().unwrap_or(&(0., 0.)).0;
        let second_angle = self.flooding_angle.min(40.).min(angle_delta_max);
        self.angle = Some((first_angle, second_angle));
        // Площадь кривой восстанавливающих плеч
        let dso_area = self.lever_diagram.dso_area(first_angle, second_angle)?;
        // Площадь кривой кренящих плеч от смещения зерна
        let first_grain_lever = self.lever_diagram.lever_moment(first_angle)?;
        let second_grain_lever = lambda_0 + delta_ab * second_angle;
        let grain_area = (first_grain_lever + (second_grain_lever - first_grain_lever)/2.) * (second_angle - first_angle) * PI/180.;
        let result_area = dso_area - grain_area;
        self.area = Some(result_area);
   /*     log::info!("\t Grain area m_grain:{m_grain} lambda_0:{lambda_0} 
            first_point_ab:{:?} second_point_ab:{:?}
            first_angle:{first_angle} angle_delta_max:{angle_delta_max} second_angle:{second_angle} 
            delta_ab:{delta_ab} dso_area:{dso_area} first_grain_lever:{first_grain_lever} second_grain_lever:{second_grain_lever}
            grain_area:{grain_area} result_area:{result_area}", first_point_ab, second_point_ab);
    */  self.parameters.add(ParameterID::HeelingMomentDueToTheTransverseShiftOfGrain, m_grain);
        self.parameters.add(ParameterID::HeelingAngleWithMaximumDifference, angle_delta_max);
        Ok(())
    }
}
///
impl IGrain for Grain {
    /// Расчетный и максимально допустимый углы крена от смещения зерна 
    fn angle(&mut self) -> Result<(f64, f64), Error> {
        if self.angle.is_none() {
            self.calculate()?;
        }
        self.angle.ok_or(Error::FromString("Grain angle error!".to_string()))
    }
    /// Остаточная площадь между кривой кренящих и
    /// кривой восстанавливающих плеч
    fn area(&mut self) -> Result<f64, Error> {
        if self.area.is_none() {
            self.calculate()?;
        }
        self.area.ok_or(Error::FromString("Grain area error!".to_string()))
    }
}
#[doc(hidden)]
pub trait IGrain {
    /// Расчетный и максимально допустимый углы крена от смещения зерна 
    fn angle(&mut self) -> Result<(f64, f64), Error>;
    /// Остаточная площадь между кривой кренящих и
    /// кривой восстанавливающих плеч
    fn area(&mut self) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeGrain {
    angle: (f64, f64),
    area: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeGrain {
    pub fn new(
        angle: (f64, f64),
        area: f64,
    ) -> Self {
        Self {
            angle,
            area,
        }
    }
}
#[doc(hidden)]
impl IGrain for FakeGrain  {
    ///
    fn angle(&mut self) -> Result<(f64, f64), Error> {
        Ok(self.angle)
    }
    ///
    fn area(&mut self) -> Result<f64, Error> {
        Ok(self.area)
    }
}


