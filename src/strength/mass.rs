//! Нагрузка на корпус судна
use std::{cell::RefCell, rc::Rc};

use crate::{
    math::*, ILoadMass, IParameters, IResults, LoadMass, LoadingType, ParameterID
};

use super::{IIcingMass, IWettingMass};

/// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
#[derive(Clone)]
pub struct Mass {
    /// Постоянная масса судна распределенная по шпациям
    loads_const: Rc<Vec<Rc<LoadMass>>>,
    /// Учет распределения обледенения судна
    icing_mass: Rc<dyn IIcingMass>,
    /// Учет намокания палубного груза - леса
    wetting_mass: Rc<dyn IWettingMass>, 
    /// Все грузы судна
    loads_variable: Rc<Vec<Rc<LoadMass>>>,
    /// Вектор разбиения на отрезки для эпюров
    bounds: Rc<Bounds>,
    /// Набор результатов расчетов для записи в БД
    results: Rc<dyn IResults>,
    parameters: Rc<dyn IParameters>,
    /// Суммарная масса
    sum: Rc<RefCell<Option<f64>>>,
    /// Распределение массы по вектору разбиения
    values: Rc<RefCell<Option<Vec<f64>>>>,
}
///
impl Mass {
    /// Аргументы конструктора:  
    /// * loads_const - постоянная масса судна распределенная по шпациям
    /// * icing_mass - Учет обледенения судна
    /// * wetting_mass - Учет намокания палубного груза - леса
    /// * loads_variable - грузы судна
    /// * bounds - вектор разбиения на отрезки для эпюров
    /// * results, parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        loads_const: Rc<Vec<Rc<LoadMass>>>,
        icing_mass: Rc<dyn IIcingMass>,
        wetting_mass: Rc<dyn IWettingMass>, 
        loads_variable: Rc<Vec<Rc<LoadMass>>>,
        bounds: Rc<Bounds>,
        results: Rc<dyn IResults>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            loads_const,
            icing_mass,
            wetting_mass,
            loads_variable,
            bounds,
            results,
            parameters,
            sum: Rc::new(RefCell::new(None)),
            values: Rc::new(RefCell::new(None)),
        }
    }
    ///
    fn calculate(&self) {
        *self.sum.borrow_mut() = Some(self.sum());
        *self.values.borrow_mut() = Some(self.values());
    }
    /// Суммарная масса
    fn sum(&self) -> f64 {
        let ballast = self
            .loads_variable
            .iter()
            .filter(|v| v.load_type() == LoadingType::Ballast)
            .map(|v| v.value(None))
            .sum::<f64>();
        let stores = self
            .loads_variable
            .iter()
            .filter(|v| v.load_type() == LoadingType::Store)
            .map(|v| v.value(None))
            .sum::<f64>();
        let cargo = self
            .loads_variable
            .iter()
            .filter(|v| v.load_type() == LoadingType::Cargo)
            .map(|v| v.value(None))
            .sum::<f64>();
        let deadweight = ballast + stores + cargo;
        let lightship = self.loads_const.iter().map(|v| v.value(None)).sum::<f64>();
        let icing = self.icing_mass.mass(None);
        let wetting = self.wetting_mass.mass(None);
        let mass_sum = deadweight + lightship + wetting;
        self.parameters.add(ParameterID::Displacement, mass_sum);
        self.parameters.add(ParameterID::MassBallast, ballast);
        self.parameters.add(ParameterID::MassStores, stores);
        self.parameters.add(ParameterID::MassCargo, cargo);
        self.parameters.add(ParameterID::MassDeadweight, deadweight);
        self.parameters.add(ParameterID::MassLightship, lightship);
        self.parameters.add(ParameterID::MassIcing, icing);
        self.parameters.add(ParameterID::MassWetting, wetting);
        log::info!("\t Mass sum:{:?} ", mass_sum);
        //    dbg!(ballast, stores, cargo, deadweight, lightship, icing);
        mass_sum
    }
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64> {
        let mut vec_hull = Vec::new();
        let mut vec_equipment = Vec::new();
        let mut vec_ballast = Vec::new();
        let mut vec_store = Vec::new();
        let mut vec_cargo = Vec::new();
        let mut vec_icing = Vec::new();
        let mut vec_wetting = Vec::new();
        let mut vec_sum = Vec::new();
        let res: Vec<f64> = self
            .bounds
            .iter()
            .map(|b| {
                let hull = self.loads_const
                    .iter()
                    .filter(|v| v.load_type() == LoadingType::Hull)
                    .map(|v| v.value(Some(*b)))
                    .sum::<f64>();
                vec_hull.push(hull);
                let equipment = self.loads_const
                    .iter()
                    .filter(|v| v.load_type() == LoadingType::Equipment)
                    .map(|v| v.value(Some(*b)))
                    .sum::<f64>();
                vec_equipment.push(equipment);
                let ballast = self.loads_variable
                    .iter()
                    .filter(|v| v.load_type() == LoadingType::Ballast)
                    .map(|v| v.value(Some(*b)))
                    .sum::<f64>();  
                vec_ballast.push(ballast);
                let store = self.loads_variable
                    .iter()
                    .filter(|v| v.load_type() == LoadingType::Store)
                    .map(|v| v.value(Some(*b)))
                    .sum::<f64>();  
                vec_store.push(store);
                let cargo = self.loads_variable
                    .iter()
                    .filter(|v| v.load_type() == LoadingType::Cargo)
                    .map(|v| v.value(Some(*b)))
                    .sum::<f64>();    
                vec_cargo.push(cargo);
                let icing = self.icing_mass.mass(Some(*b));
                vec_icing.push(icing);                
                let wetting = self.wetting_mass.mass(Some(*b));
                vec_wetting.push(wetting);
                self.loads_const
                    .iter()
                    .map(|v| v.value(Some(*b)))
                    .sum::<f64>()
                    + self
                        .loads_variable
                        .iter()
                        .map(|v| v.value(Some(*b)))
                        .sum::<f64>()
                    + icing
            })
            .collect();
        vec_hull.push(vec_hull.iter().sum());
        vec_equipment.push(vec_equipment.iter().sum());
        vec_ballast.push(vec_ballast.iter().sum());
        vec_store.push(vec_store.iter().sum());
        vec_cargo.push(vec_cargo.iter().sum());
        vec_icing.push(vec_icing.iter().sum());
        vec_wetting.push(vec_wetting.iter().sum());
        vec_sum.append(&mut res.clone());
        vec_sum.push(res.iter().sum());
        log::info!("\t Mass values:{:?} ", res);
        self.results.add("value_mass_hull".to_owned(), vec_hull);
        self.results.add("value_mass_equipment".to_owned(), vec_equipment);
        self.results.add("value_mass_ballast".to_owned(), vec_ballast);
        self.results.add("value_mass_store".to_owned(), vec_store);
        self.results.add("value_mass_cargo".to_owned(), vec_cargo);
        self.results.add("value_mass_icing".to_owned(), vec_icing);
        self.results.add("value_mass_wetting".to_owned(), vec_wetting);
        self.results.add("value_mass_sum".to_owned(), vec_sum);
        res
    }
}
///
impl IMass for Mass {
    /// Суммарная масса
    fn sum(&self) -> f64 {
        if self.sum.borrow().is_none() {
            self.calculate();
        }
        self.sum.borrow().clone().expect("Mass sum error: no value")
    }
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64> {
        if self.values.borrow().is_none() {      
            self.calculate();
        }
        self.values
            .borrow()
            .clone()
            .expect("Mass values error: no values")
    }
}

#[doc(hidden)]
pub trait IMass {
    /// Суммарная масса
    fn sum(&self) -> f64;
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeMass {
    sum: f64,
    values: Vec<f64>,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeMass {
    pub fn new(
        sum: f64,
        values: Vec<f64>,
    ) -> Self {
        Self {
            sum,
            values,
        }
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
}
