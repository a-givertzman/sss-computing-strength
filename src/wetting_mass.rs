//! Учет обледенения

use crate::{Bound, ILoadMass, LoadMass, Moment};
use std::rc::Rc;

/// Учет намокания палубного лесного груза.  
/// При расчете намокания необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. Масса намокания и его моменты учитывается
/// при расчете прочности.
#[derive(Clone)]
pub struct WettingMass {
    /// Коэффициент изменения массы при намокании
    coeff: f64,
    /// Палубный груз - лес
    loads_timber: Rc<Vec<Rc<LoadMass>>>,
}
///
impl WettingMass {
    /// Основной конструктор
    /// * coeff - Коэффициент изменения массы при намокании
    /// * loads_timber - Палубный груз - лес
    pub fn new(coeff: f64, loads_timber: Rc<Vec<Rc<LoadMass>>>) -> Self {
        Self {
            coeff,
            loads_timber,
        }
    }
}
///
impl IWettingMass for WettingMass {
    /// Масса намокания попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: Option<Bound>) -> f64 {
        self.loads_timber
            .iter()
            .map(|v| v.value(bound))
            .sum::<f64>()
            * self.coeff
    }
    /// Суммарный статический момент массы намокания
    fn moment(&self) -> Moment {
        self.loads_timber
            .iter()
            .map(|v| v.moment())
            .sum::<Moment>()
            .scale(self.coeff)
    }
}
#[doc(hidden)]
pub trait IWettingMass {
    /// Масса намокания попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: Option<Bound>) -> f64;
    /// Суммарный статический момент массы намокания
    fn moment(&self) -> Moment;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeWettingMass {
    mass: f64,
    moment: Moment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeWettingMass {
    pub fn new(mass: f64, moment: Moment) -> Self {
        Self { mass, moment }
    }
}
#[doc(hidden)]
impl IWettingMass for FakeWettingMass {
    fn mass(&self, _: Option<Bound>) -> f64 {
        self.mass
    }
    fn moment(&self) -> Moment {
        self.moment.clone()
    }
}
