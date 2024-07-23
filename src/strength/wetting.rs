//! Учет массы от намокания палубного лесного груза

use crate::{Bound, Error, ILoadMass, LoadMass};
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
    fn mass(&self, bound: Option<Bound>) -> Result<f64, Error> {
        let mut sum = 0.;
        for v in self.loads_timber
            .iter() {
                sum += v.value(bound)?; 
            }
        Ok(sum * self.coeff)
    }
}
#[doc(hidden)]
pub trait IWettingMass {
    /// Масса намокания попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: Option<Bound>) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeWettingMass {
    mass: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeWettingMass {
    pub fn new(mass: f64) -> Self {
        Self { mass }
    }
}
#[doc(hidden)]
impl IWettingMass for FakeWettingMass {
    fn mass(&self, _: Option<Bound>) -> Result<f64, Error> {
        Ok(self.mass)
    }
}
