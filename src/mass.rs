//! Нагрузка на корпус, масса всех грузов + масса конструкций и механизмов судна 
use std::rc::Rc;
use crate::{load::ILoad, math::{bound::Bound, mass_moment::MassMoment, position::Position, surface_moment::SurfaceMoment}};

/// Нагрузка на корпус судна, список всех грузов
#[derive(Clone)]
pub struct Mass {
    /// все грузы судна
    loads: Vec<Rc<Box<dyn ILoad>>>,
    /// Разбиение на отрезки для расчетов эпюров
    bounds: Vec<Bound>,
}
///
impl Mass {
    /// Основной конструктор 
    pub fn new (loads: Vec<Rc<Box<dyn ILoad>>>, bounds: Vec<Bound>) -> Self {
        Self { loads, bounds, }
    }
    /// Суммарный статический момент
    fn moment_mass (&self) -> MassMoment {
        self.loads.iter().map(|c| c.moment_mass() ).sum::<MassMoment>()
    }
    /// Суммарный момент свободной поверхности для жидких грузов
    fn moment_surface (&self) -> SurfaceMoment {
        self.loads.iter().map(|c| c.moment_surface() ).sum::<SurfaceMoment>()
    }
}
///
impl IMass for Mass {
    /// Суммарная масса всех грузов, полная масса судна
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
        self.moment_surface().y()/self.sum()
    }
}
///
#[doc(hidden)]
pub trait IMass {
    fn sum(&self) -> f64;
    fn values(&self) -> Vec<f64>;
    fn shift(&self) -> Position;
    fn delta_m_h(&self) -> f64;
}
// Заглушка для тестирования
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