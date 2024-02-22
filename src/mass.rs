//! Нагрузка на корпус судна
use std::rc::Rc;

use crate::{load::ILoad, math::{bound::Bound, mass_moment::MassMoment, position::Position, surface_moment::SurfaceMoment}};

/// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
#[derive(Clone)]
pub struct Mass {
    /// все грузы судна
    loads: Vec<Rc<Box<dyn ILoad>>>,
    /// ссылка на вектор разбиения на отрезки для эпюров
    bounds: Vec<Bound>,
}

impl Mass {
    /// Аргументы конструктора:  
    /// * loads - вектор абстрактных грузов
    /// * bounds - ссылка на вектор разбиения на отрезки для эпюров
    pub fn new (loads: Vec<Rc<Box<dyn ILoad>>>, bounds: Vec<Bound>) -> Self {
        Self { loads, bounds, }
    }
    /// Суммарная масса
    pub fn sum(&self) -> f64 {
        self.loads.iter().map(|v| v.mass(None)).sum::<f64>()
    }    
    /// Распределение массы по вектору разбиения
    pub fn values(&self) -> Vec<f64> {
        self.bounds.iter().map(|b| 
            self.loads.iter().map(|v| v.mass(Some(*b))).sum::<f64>()).collect()
    }
    /// Отстояние центра масс
    pub fn shift(&self) -> Position {
        self.moment_mass().to_pos(self.sum())
    }
    /// Поправка к продольной метацентрической высоте на влияние свободной поверхности жидкости в цистернах 
    pub fn delta_m_h(&self) -> f64 {
        self.moment_surface().y()/self.sum()
    }
    ///Суммарный статический момент
    fn moment_mass (&self) -> MassMoment {
        self.loads.iter().map(|c| c.moment_mass() ).sum::<MassMoment>()
    }
    /// Суммарный момент свободной поверхности
    fn moment_surface (&self) -> SurfaceMoment {
        self.loads.iter().map(|c| c.moment_surface() ).sum::<SurfaceMoment>()
    }
}