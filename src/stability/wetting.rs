//! Учет момента от намокания палубного лесного груза

use crate::{ILoadMass, LoadMass, Moment};
use std::rc::Rc;

/// Учет намокания палубного лесного груза.  
/// При расчете намокания необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. Масса намокания и его моменты учитывается
/// при расчете прочности.
#[derive(Clone)]
pub struct WettingMoment {
    /// Коэффициент изменения массы при намокании
    coeff: f64,
    /// Палубный груз - лес
    loads_timber: Rc<Vec<Rc<LoadMass>>>,
}
///
impl WettingMoment {
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
impl IWettingMoment for WettingMoment {
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
pub trait IWettingMoment {
    /// Суммарный статический момент массы намокания
    fn moment(&self) -> Moment;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeWettingMoment {
    moment: Moment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeWettingMoment {
    pub fn new(moment: Moment) -> Self {
        Self { moment }
    }
}
#[doc(hidden)]
impl IWettingMoment for FakeWettingMoment {
    fn moment(&self) -> Moment {
        self.moment.clone()
    }
}
