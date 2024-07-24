//! Нагрузка на корпус судна
use std::{cell::RefCell, rc::Rc};

use crate::{math::*, Error, ILoadMass, IParameters, IResults, LoadMass, LoadingType, ParameterID};

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
    fn calculate(&self) -> Result<(), Error> {
        *self.sum.borrow_mut() = Some(self.sum()?);
        *self.values.borrow_mut() = Some(self.values()?);
        Ok(())
    }
    /// Суммарная масса
    fn sum(&self) -> Result<f64, Error> {
        let mut ballast = 0.;
        for v in self
            .loads_variable
            .iter()
            .filter(|v| v.load_type() == LoadingType::Ballast)
        {
            ballast += v.value(None)?;
        }
        let mut stores = 0.;
        for v in self
            .loads_variable
            .iter()
            .filter(|v| v.load_type() == LoadingType::Stores)
        {
            stores += v.value(None)?;
        }
        let mut cargo = 0.;
        for v in self
            .loads_variable
            .iter()
            .filter(|v| v.load_type() == LoadingType::Cargo)
        {
            cargo += v.value(None)?;
        }
        let deadweight = ballast + stores + cargo;
        let mut lightship = 0.;
        for v in self.loads_const.iter() {
            lightship += v.value(None)?;
        }
        let icing = self.icing_mass.mass(None)?;
        let wetting = self.wetting_mass.mass(None)?;
        let mass_sum = deadweight + lightship + wetting;
        self.parameters.add(ParameterID::Displacement, mass_sum);
        self.parameters.add(ParameterID::MassBallast, ballast);
        self.parameters.add(ParameterID::MassStores, stores);
        self.parameters.add(ParameterID::MassCargo, cargo);
        self.parameters.add(ParameterID::MassDeadweight, deadweight);
        self.parameters.add(ParameterID::MassLightship, lightship);
        self.parameters.add(ParameterID::MassIcing, icing);
        self.parameters.add(ParameterID::MassWetting, wetting);
        //    log::info!("\t Mass sum:{:?} ", mass_sum);
        //    dbg!(ballast, stores, cargo, deadweight, lightship, icing);
        Ok(mass_sum)
    }
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Result<Vec<f64>, Error> {
        let mut vec_hull = Vec::new();
        let mut vec_equipment = Vec::new();
        let mut vec_ballast = Vec::new();
        let mut vec_store = Vec::new();
        let mut vec_cargo = Vec::new();
        let mut vec_icing = Vec::new();
        let mut vec_wetting = Vec::new();
        let mut vec_sum = Vec::new();
        let mut res: Vec<f64> = Vec::new();
        for b in self.bounds.iter() {
            let mut hull = 0.;
            for v in self
                .loads_const
                .iter()
                .filter(|v| v.load_type() == LoadingType::Hull)
            {
                hull += v.value(Some(*b))?;
            }
            vec_hull.push(hull);
            let mut equipment = 0.;
            for v in self
                .loads_const
                .iter()
                .filter(|v| v.load_type() == LoadingType::Equipment)
            {
                equipment += v.value(Some(*b))?;
            }
            vec_equipment.push(equipment);
            let mut ballast = 0.;
            for v in self
                .loads_variable
                .iter()
                .filter(|v| v.load_type() == LoadingType::Ballast)
            {
                ballast += v.value(Some(*b))?;
            }
            vec_ballast.push(ballast);
            let mut store = 0.;
            for v in self
                .loads_variable
                .iter()
                .filter(|v| v.load_type() == LoadingType::Stores)
            {
                store += v.value(Some(*b))?;
            }
            vec_store.push(store);
            let mut cargo = 0.;
            for v in self
                .loads_variable
                .iter()
                .filter(|v| v.load_type() == LoadingType::Cargo)
            {
                cargo += v.value(Some(*b))?;
            }
            vec_cargo.push(cargo);
            let icing = self.icing_mass.mass(Some(*b))?;
            vec_icing.push(icing);
            let wetting = self.wetting_mass.mass(Some(*b))?;
            vec_wetting.push(wetting);

            res.push(hull + equipment + ballast + store + cargo + icing + wetting);
        }
        vec_hull.push(vec_hull.iter().sum());
        vec_equipment.push(vec_equipment.iter().sum());
        vec_ballast.push(vec_ballast.iter().sum());
        vec_store.push(vec_store.iter().sum());
        vec_cargo.push(vec_cargo.iter().sum());
        vec_icing.push(vec_icing.iter().sum());
        vec_wetting.push(vec_wetting.iter().sum());
        vec_sum.append(&mut res.clone());
        vec_sum.push(res.iter().sum());
    //    log::info!("\t Mass values:{:?} ", res);
        self.results.add("value_mass_hull".to_owned(), vec_hull);
        self.results
            .add("value_mass_equipment".to_owned(), vec_equipment);
        self.results
            .add("value_mass_ballast".to_owned(), vec_ballast);
        self.results.add("value_mass_store".to_owned(), vec_store);
        self.results.add("value_mass_cargo".to_owned(), vec_cargo);
        self.results.add("value_mass_icing".to_owned(), vec_icing);
        self.results
            .add("value_mass_wetting".to_owned(), vec_wetting);
        self.results.add("value_mass_sum".to_owned(), vec_sum);
        Ok(res)
    }
}
///
impl IMass for Mass {
    /// Суммарная масса
    fn sum(&self) -> Result<f64, Error> {
        if self.sum.borrow().is_none() {
            self.calculate()?;
        }
        Ok((*self.sum.borrow()).expect("Mass sum error: no value"))
    }
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Result<Vec<f64>, Error> {
        if self.values.borrow().is_none() {
            self.calculate()?;
        }
        Ok(self.values
            .borrow()
            .clone()
            .expect("Mass values error: no values"))
    }
}

#[doc(hidden)]
pub trait IMass {
    /// Суммарная масса
    fn sum(&self) -> Result<f64, Error>;
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Result<Vec<f64>, Error>;
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
    pub fn new(sum: f64, values: Vec<f64>) -> Self {
        Self { sum, values }
    }
}
#[doc(hidden)]
impl IMass for FakeMass {
    fn sum(&self) -> Result<f64, Error> {
        Ok(self.sum)
    }
    fn values(&self) -> Result<Vec<f64>, Error> {
        Ok(self.values.clone())
    }
}
