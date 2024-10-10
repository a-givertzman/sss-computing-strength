//! Момент массы судна
use std::{cell::RefCell, rc::Rc};

use crate::{math::*, Error, ILoadMass, IMass, IParameters, LoadMass, ParameterID};

use super::{IIcingMoment, IWettingMoment};

/// Момент массы судна: сумма моментов конструкции, груз, экипаж и т.п. для расчета остойчивости
#[derive(Clone)]
pub struct ShipMoment {
    /// Масса судна
    mass: Rc<dyn IMass>,
    /// Постоянная масса судна распределенная по шпациям
    loads_const: Rc<Vec<Rc<LoadMass>>>,
    /// Смещение постоянный массы судна
    shift_const: Position,
    /// Все грузы судна
    loads_variable: Rc<Vec<Rc<LoadMass>>>,
    /// Учет обледенения судна
    icing_moment: Rc<dyn IIcingMoment>,
    /// Учет намокания палубного груза - леса
    wetting_moment: Rc<dyn IWettingMoment>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
    /// Суммарный статический момент
    moment_mass: Rc<RefCell<Option<Moment>>>,
    /// Отстояние центра масс
    shift: Rc<RefCell<Option<Position>>>,
}
//
impl ShipMoment {
    /// Аргументы конструктора:  
    /// * mass - Масса судна
    /// * loads_const - постоянная масса судна распределенная по шпациям
    /// * shift_const - смещение постоянный массы судна
    /// * icing_mass, icing_moment - Учет обледенения судна
    /// * wetting_mass, wetting_moment - Учет намокания палубного груза - леса
    /// * loads_variable - грузы судна
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        mass: Rc<dyn IMass>,
        loads_const: Rc<Vec<Rc<LoadMass>>>,
        shift_const: Position,
        icing_moment: Rc<dyn IIcingMoment>,
        wetting_moment: Rc<dyn IWettingMoment>,
        loads_variable: Rc<Vec<Rc<LoadMass>>>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            mass,
            loads_const,
            shift_const,
            icing_moment,
            wetting_moment,
            loads_variable,
            parameters,
            moment_mass: Rc::new(RefCell::new(None)),
            shift: Rc::new(RefCell::new(None)),
        }
    }
    //
    fn calculate(&self) -> Result<(), Error> {
        *self.shift.borrow_mut() = Some(self.shift()?);
        *self.moment_mass.borrow_mut() = Some(self.moment_mass()?);
        Ok(())
    }
    /// Отстояние центра масс
    fn shift(&self) -> Result<Position, Error> {
        let res = self.moment_mass()?.to_pos(self.mass.sum()?);
        //   log::info!("\t Mass shift:{res} ");
        self.parameters.add(ParameterID::CenterMassX, res.x());
        self.parameters.add(ParameterID::CenterMassZ, res.z());
        Ok(res)
    }
    /// Суммарный статический момент. Для постоянной массы и для запасов считается по
    /// заданным значениям смещения центра масс
    fn moment_mass(&self) -> Result<Moment, Error> {
        let mut moment_sum = self
            .loads_variable
            .iter()
            .map(|c| c.moment())
            .sum::<Moment>()
            + self.wetting_moment.moment()
            + self.icing_moment.moment()?;
        for v in self.loads_const.iter() {
            moment_sum += Moment::from_pos(self.shift_const.clone(), v.value(&Bound::Full)?);
        }
        //log::info!("\t Mass moment_mass:{res} ");
        Ok(moment_sum)
    }
}
//
impl IShipMoment for ShipMoment {
    /// Отстояние центра масс
    fn shift(&self) -> Result<Position, Error> {
        if self.shift.borrow().is_none() {
            self.calculate()?;
        }
        Ok(self.shift
            .borrow()
            .clone()
            .expect("Mass shift error: no value"))
    }
}

#[doc(hidden)]
pub trait IShipMoment {
    /// Отстояние центра масс
    fn shift(&self) -> Result<Position, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeShipMoment {
    shift: Position,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeShipMoment {
    pub fn new(shift: Position) -> Self {
        Self { shift }
    }
}
#[doc(hidden)]
impl IShipMoment for FakeShipMoment {
    fn shift(&self) -> Result<Position, Error> {
        Ok(self.shift.clone())
    }
}
