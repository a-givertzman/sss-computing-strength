//! Нагрузка на корпус судна
use std::{cell::RefCell, rc::Rc};

use crate::{icing::IIcingMass, math::*, IParameters, LoadMass, LoadingType, ParameterID, Parameters};

use super::load::ILoadMass;

/// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
#[derive(Clone)]
pub struct Mass {
    /// Постоянная масса судна распределенная по шпациям
    loads_const: Rc<Vec<Rc<dyn ILoadMass>>>, 
    /// Смещение постоянный массы судна
    shift_const: Position,
    /// Учет распределения обледенения судна
    icing_mass: Rc<dyn IIcingMass>,
    /// Все грузы судна
    loads_variable: Rc<Vec<Rc<LoadMass>>>, 
    /// Вектор разбиения на отрезки для эпюров
    bounds: Rc<Bounds>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>, 
    /// Суммарный статический момент
    moment_mass: Rc<RefCell<Option<Moment>>>,
    /// Суммарная масса
    sum: Rc<RefCell<Option<f64>>>,
    /// Распределение массы по вектору разбиения
    values: Rc<RefCell<Option<Vec<f64>>>>,
    /// Отстояние центра масс
    shift: Rc<RefCell<Option<Position>>>,
}
///
impl Mass {
    /// Аргументы конструктора:  
    /// * loads_const - постоянная масса судна распределенная по шпациям
    /// * shift_const - смещение постоянный массы судна
    /// * icing_mass - Учет обледенения судна
    /// * loads_variable - грузы судна
    /// * bounds - вектор разбиения на отрезки для эпюров
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        loads_const: Rc<Vec<Rc<dyn ILoadMass>>>, 
        shift_const: Position,
        icing_mass: Rc<dyn IIcingMass>,
        loads_variable: Rc<Vec<Rc<LoadMass>>>, 
        bounds: Rc<Bounds>,
        parameters: Rc<dyn IParameters>, 
    ) -> Self {
        Self {
            loads_const,
            shift_const,
            icing_mass,
            loads_variable,
            bounds,
            parameters,
            moment_mass: Rc::new(RefCell::new(None)),
            sum: Rc::new(RefCell::new(None)),
            values: Rc::new(RefCell::new(None)),
            shift: Rc::new(RefCell::new(None)),
        }
    }
}
///
impl IMass for Mass {
    /// Суммарная масса
    fn sum(&self) -> f64 {
        if self.sum.borrow().is_none() {
            let ballast = self.loads_variable.iter().filter(|v| v.load_type() == LoadingType::Ballast ).map(|v| v.value(None)).sum::<f64>();
            let stores = self.loads_variable.iter().filter(|v| v.load_type() == LoadingType::Store ).map(|v| v.value(None)).sum::<f64>();
            let cargo = self.loads_variable.iter().filter(|v| v.load_type() == LoadingType::Cargo ).map(|v| v.value(None)).sum::<f64>();
            let deadweight = ballast + stores + cargo;
            let lightship = self.loads_const.iter().map(|v| v.value(None)).sum::<f64>();
            let icing = self.icing_mass.mass(None);
            let wetting = 0.; //TODO
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
            *self.sum.borrow_mut() = Some(mass_sum);
        }
        self.sum.borrow().clone().expect("Mass sum error: no value")
    }
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64> {
        if self.values.borrow().is_none() {
            let res: Vec<f64> = self
                .bounds
                .iter()
                .map(|b| {
                    self.loads_const
                        .iter()
                        .map(|v| v.value(Some(*b)))
                        .sum::<f64>()
                        + self
                            .loads_variable
                            .iter()
                            .map(|v| v.value(Some(*b)))
                            .sum::<f64>()
                        + self.icing_mass.mass(Some(*b))
                })
                .collect();
            log::info!("\t Mass values:{:?} ", res);
            *self.values.borrow_mut() = Some(res);
        }
        self.values
            .borrow()
            .clone()
            .expect("Mass values error: no values")
    }
    /// Отстояние центра масс
    fn shift(&self) -> Position {
        if self.shift.borrow().is_none() {
            let res = self.moment_mass().to_pos(self.sum());
            log::info!("\t Mass shift:{res} ");
            self.parameters.add(ParameterID::CenterMassX, res.x());
            self.parameters.add(ParameterID::CenterMassZ, res.z());
            *self.shift.borrow_mut() = Some(res);
        }
        self.shift
            .borrow()
            .clone()
            .expect("Mass shift error: no value")
    }
    /// Суммарный статический момент. Для постоянной массы и для запасов считается по
    /// заданным значениям смещения центра масс
    fn moment_mass(&self) -> Moment {
        if self.moment_mass.borrow().is_none() {
            let res = self
                .loads_const
                .iter()
                .map(|c| {
                    Moment::from_pos(self.shift_const.clone(), c.value(None))
                })
                .sum::<Moment>()
                + self
                    .loads_variable
                    .iter()
                    .map(|c| c.moment())
                    .sum::<Moment>()
                + self.icing_mass.moment();
            log::info!("\t Mass moment_mass:{res} ");
            *self.moment_mass.borrow_mut() = Some(res);
        }
        self.moment_mass
            .borrow()
            .clone()
            .expect("Mass moment_mass error: no value")
    }
}

#[doc(hidden)]
pub trait IMass {
    /// Суммарная масса
    fn sum(&self) -> f64;
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64>;
    /// Отстояние центра масс
    fn shift(&self) -> Position;
    /// Поправка к продольной метацентрической высоте на  
    /// влияние свободной поверхности жидкости в цистернах
  //  fn delta_m_h(&self) -> DeltaMH;
    /// Суммарный статический момент. Для постоянной массы и для запасов считается по
    /// заданным значениям смещения центра масс
    fn moment_mass(&self) -> Moment;
    // Суммарный момент свободной поверхности
 //   fn moment_surface(&self) -> SurfaceMoment;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeMass {
    sum: f64,
    values: Vec<f64>,
    shift: Position,
    moment_mass: Moment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeMass {
    pub fn new(
        sum: f64,
        values: Vec<f64>,
        shift: Position,
        moment_mass: Moment,
    ) -> Self {
        Self {
            sum,
            values,
            shift,
            moment_mass,
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
    fn shift(&self) -> Position {
        self.shift.clone()
    }
 /*   fn delta_m_h(&self) -> DeltaMH {
        self.delta_m_h.clone()
    }*/
    fn moment_mass(&self) -> Moment {
        self.moment_mass.clone()
    }
  /*  fn moment_surface(&self) -> SurfaceMoment {
        self.moment_surface.clone()
    }*/
}
