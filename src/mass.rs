use std::rc::Rc;

use crate::{load::ILoad, math::{bound::Bound, moment::Moment, position::Position}};

///класс, инкапсулирующий все грузы судна
pub struct Mass<'a> {
    /// вектор абстрактных грузов
    loads: Vec<Rc<Box<dyn ILoad>>>,
    /// ссылка на вектор разбиения на отрезки для эпюров
    bounds: &'a Vec<Bound>,
}

impl<'a> Mass<'a> {
    /// аргументы  
    /// * loads - вектор абстрактных грузов
    /// * bounds - ссылка на вектор разбиения на отрезки для эпюров
    pub fn new<'b: 'a>(loads: Vec<Rc<Box<dyn ILoad>>>, bounds: &'a Vec<Bound>) -> Self {
        Self { loads, bounds, }
    }
    ///суммарная масса
    pub fn sum(&self) -> f64 {
        self.loads.iter().map(|v| v.mass(None)).sum::<f64>()    
    }    
    ///распределение массы по вектору разбиения
    pub fn values(&self) -> Vec<f64> {
        self.bounds.into_iter().map(|&b| 
            self.loads.iter().map(|v| v.mass(Some(b))).sum::<f64>()).collect()
    }
    ///отстояние центра масс
    pub fn shift(&self) -> Position {
        self.moment().to_pos(self.sum())
    }
    ///суммарный статический момент
    pub fn moment (&self) -> Moment {
        self.loads.iter().map(|c| c.moment() ).sum::<Moment>()
    }
}