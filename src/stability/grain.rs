//! Критерий крена от смещения зерна

use std::{f64::consts::PI, rc::Rc};
use crate::{ILeverDiagram, IBulk, IMass};

/// Критерий крена от смещения зерна
pub struct Grain {
    /// Осадка судна d
    d: f64,
    /// Угол заливания отверстий
    flooding_angle: f64, 
    /// Все навалочные смещаемые грузы судна
    loads_bulk: Rc<Vec<Rc<dyn IBulk>>>,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    mass: Rc<dyn IMass>,  
    /// Диаграмма плеч статической и динамической остойчивости
    lever_diagram: Rc<dyn ILeverDiagram>, 
}
/// 
impl Grain {
    /// Основной конструктор
    /// * d - Осадка судна d
    /// * flooding_angle - Угол заливания отверстий
    /// * loads_bulk - Все навалочные смещаемые грузы судна
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    pub fn new(
        d: f64,
        flooding_angle: f64, 
        loads_bulk: Rc<Vec<Rc<dyn IBulk>>>,
        mass: Rc<dyn IMass>,  
        lever_diagram: Rc<dyn ILeverDiagram>, 
    ) -> Self {
        Self {
            d,
            flooding_angle,
            loads_bulk,
            mass,
            lever_diagram,
        }
    }
}
///
impl IGrain for Grain {
    /// Остаточная площадь между кривой кренящих и
    /// кривой восстанавливающих плеч
    fn area(&self) -> f64 {
        let m_grain: f64 = self.loads_bulk.iter().map(|v| v.moment() ).sum();    
        let lambda_0 = m_grain/self.mass.sum(); 
        // Первая точка апроксимирующей прямой
        let first_point_ab = (0.0f64, lambda_0); 
        // Вторая точка апроксимирующей прямой
        let second_point_ab = (40.0f64, 0.8*lambda_0);
        // Изменение апроксимирующей прямой на один градус угла крена
        let delta_ab = (second_point_ab.1 - first_point_ab.1)/(second_point_ab.0 - first_point_ab.0);
        let precision = 0.1; // Точность определения пересечения в градусах
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
            let lever_ab = delta_ab * angle; 
            // значение апроксимирующей прямой плеч момента зерна в текущей точке
            let lever_dso = self.lever_diagram.lever_moment(angle);
            lever_dso >= lever_ab
        }).unwrap_or((90./precision) as usize) as f64 * precision;
        let second_angle = self.flooding_angle.min(40.);
        if first_angle >= second_angle {
            return 0.;
        }
        // Площадь кривой восстанавливающих плеч
        let dso_area = self.lever_diagram.dso_area(first_angle, second_angle);
        // Площадь кривой кренящих плеч от смещения зерна
        let first_grain_lever = self.lever_diagram.lever_moment(first_angle);
        let second_grain_lever = delta_ab * second_angle;
        let grain_area = ((second_grain_lever - first_grain_lever)/2.) * (second_angle - first_angle) * PI/180.;
        let result_area = dso_area - grain_area;
        log::info!("\t Grain area m_grain:{m_grain} lambda_0:{lambda_0} first_angle:{first_angle} 
            second_angle:{second_angle} dso_area:{dso_area} grain_area:{grain_area} result_area:{result_area}");
        result_area
    }
}
#[doc(hidden)]
pub trait IGrain {
    /// Остаточная площадь между кривой кренящих и
    /// кривой восстанавливающих плеч
    fn area(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeAccelleration {
    area: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeAccelleration {
    pub fn new(
        area: f64,
    ) -> Self {
        Self {
            area,
        }
    }
}
#[doc(hidden)]
impl IGrain for FakeAccelleration {
    ///
    fn area(&self) -> f64 {
        self.area
    }
}


