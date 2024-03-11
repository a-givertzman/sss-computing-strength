//! Нагрузка на корпус судна
use std::rc::Rc;

use crate::{load::ILoad, math::*};

/// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
#[derive(Clone)]
pub struct Mass {
    /// все грузы судна
    loads: Vec<Rc<Box<dyn ILoad>>>,
    /// ссылка на вектор разбиения на отрезки для эпюров
    bounds: Bounds,
}

impl Mass {
    /// Аргументы конструктора:  
    /// * loads - вектор абстрактных грузов
    /// * bounds - ссылка на вектор разбиения на отрезки для эпюров
    pub fn new (loads: Vec<Rc<Box<dyn ILoad>>>, bounds: Bounds) -> Self {
        Self { loads, bounds, }
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

impl IMass for Mass {
    /// Суммарная масса
    fn sum(&self) -> f64 {
        self.loads.iter().map(|v| v.mass(None)).sum::<f64>()
    }    
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64> {
        self.bounds.iter().map(|b| 
            self.loads.iter().map(|v| v.mass(Some(*b))).sum::<f64>()).collect()
    }
    /// Отстояние центра масс
    fn shift(&self) -> Position {
        self.moment_mass().to_pos(self.sum())
    }
    /// Поправка к продольной метацентрической высоте на влияние свободной поверхности жидкости в цистернах 
    fn delta_m_h(&self) -> f64 {
        assert!(self.sum() > 0., "Mass delta_m_h sum > 0");
        self.moment_surface().y()/self.sum()
    }
}

#[doc(hidden)]
pub trait IMass {
    fn sum(&self) -> f64;
    fn values(&self) -> Vec<f64>;
    fn shift(&self) -> Position;
    fn delta_m_h(&self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeMass {
    sum: f64,
    values: Vec<f64>,
    shift: Position,
    delta_m_h: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeMass {
    pub fn new( 
        sum: f64,
        values: Vec<f64>,
        shift: Position,
        delta_m_h: f64,
    ) -> Self {
        Self { sum, values, shift, delta_m_h, }
    }
}
#[doc(hidden)]
impl IMass for FakeMass {
    fn sum(&self) -> f64 {
        self.sum
    }    
    fn values(&self) -> Vec<f64> {
        self.values.clone()
    }
    fn shift(&self) -> Position {
        self.shift.clone()
    }
    fn delta_m_h(&self) -> f64 {
        self.delta_m_h
    }
}