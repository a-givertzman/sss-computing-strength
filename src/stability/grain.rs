/// Угол крена от смещения зерна

use std::rc::Rc;

use crate::{ICurve, IMetacentricHeight, IRollingAmplitude, IRollingPeriod};

/// Угол крена от смещения зерна
pub struct Grain {
    /// Осадка судна d
    d: f64,
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
    /// * loads_bulk - Все навалочные смещаемые грузы судна
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    pub fn new(
        d: f64,
        loads_bulk: Rc<Vec<Rc<dyn IBulk>>>,
        mass: Rc<dyn IMass>,  
        lever_diagram: Rc<dyn ILeverDiagram>, 
    ) -> Self {
        Self {
            d,
            loads_bulk,
            mass,
            lever_diagram,
        }
    }
}
///
impl IGrain for Grain {
    /// Расчет угла крена от смещения зерна
    fn angle(&self) -> f64 {
        let m_grain = self.loads_bulk.iter().map(|v| v.moment() ).sum();    
        let a_lever_moment = m_grain/self.mass.sum(); 
        let a_grain = self.lever_diagram.angle(a_lever_moment);   
        a_grain
    }
}
#[doc(hidden)]
pub trait IGrain {
    /// Расчет угла крена от смещения зерна
    fn angle(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeAccelleration {
    angle: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeAccelleration {
    pub fn new(
        angle: f64,
    ) -> Self {
        Self {
            angle,
        }
    }
}
#[doc(hidden)]
impl IGrain for FakeAccelleration {
    ///
    fn angle(&self) -> f64 {
        self.angle
    }
}


