//! Учет обледенения

use std::rc::Rc;
use crate::{Bound, ILoad, Moment};
use super::IIcingStab;

/// Учет обледенения судна, расчет массы льда. 
/// Может быть без обледенения, частичным и полным.  
/// При расчете обледенения необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. При учете обледенения к массе судна добавляются  
/// масса льда на бортах, палубах, палубном грузе. Масса льда и его моменты,  
/// рассчитываются для осадки 𝑑𝑚𝑖𝑛 и распространяются на все случаи загрузки. 
pub struct IcingMass {
    /// Тип обледенения
    icing_stab: Rc<dyn IIcingStab>,
    /// Распределение площади поверхностей
    area_strength: Rc<dyn crate::strength::IArea>,
    /// Момент площади поверхностей
    area_moment: Rc<dyn crate::stability::IArea>,
}
///
impl IcingMass {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения
    /// * area_strength - Распределение площади поверхностей
    /// * area_moment - Момент площади поверхностей  
    pub fn new(
        icing_stab: Rc<dyn IIcingStab>,
        area_strength: Rc<dyn crate::strength::IArea>,   
        area_moment: Rc<dyn crate::stability::IArea>,
    ) -> Self {
        Self{
            icing_stab, 
            area_strength,   
            area_moment,      
        }
    }
}
///
impl IIcingMass for IcingMass {
    /// Масса льда попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: Option<Bound>) -> f64 {
        self.area_strength.area_h(bound) * self.icing_stab.mass_h() + 
        self.area_strength.area_v(bound) * self.icing_stab.mass_v()
    }
    /// Суммарный статический момент массы льда.
    fn moment(&self) -> Moment {
        self.area_moment.moment_h().scale( self.icing_stab.mass_h() ) + 
        self.area_moment.moment_v().scale( self.icing_stab.mass_v() )
    }
}
#[doc(hidden)]
pub trait IIcingMass {
    /// Масса льда попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: Option<Bound>) -> f64;
    /// Суммарный статический момент массы льда.
    fn moment(&self) -> Moment;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeIcing {
    mass: f64,
    moment: Moment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeIcing {
    pub fn new(
        mass: f64,
        moment: Moment,
    ) -> Self {
        Self {
            mass,
            moment,
        }
    }
}
#[doc(hidden)]
impl IIcingMass for FakeIcing {
    fn mass(&self, _: Option<Bound>) -> f64 {
        self.mass
    }
    fn moment(&self) -> Moment{
        self.moment
    }
}



