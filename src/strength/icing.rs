//! Учет обледенения

use std::rc::Rc;

use crate::{icing_stab::IIcingStab, Bound, Error};

/// Учет обледенения судна.
/// Может быть без обледенения, частичным и полным.  
/// При расчете обледенения необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. При учете обледенения к массе судна добавляются  
/// масса льда на бортах, палубах, палубном грузе. Масса льда и его моменты,  
/// рассчитываются для осадки 𝑑𝑚𝑖𝑛 и распространяются на все случаи загрузки.
#[derive(Clone)]
pub struct IcingMass {
    /// Тип обледенения судна
    icing_stab: Rc<dyn IIcingStab>,
    /// Распределение площади поверхностей
    area_strength: Rc<dyn crate::strength::IArea>,
}
//
impl IcingMass {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения судна
    /// * area_strength - Распределение площади поверхностей
    pub fn new(
        icing_stab: Rc<dyn IIcingStab>,
        area_strength: Rc<dyn crate::strength::IArea>,
    ) -> Self {
        Self {
            icing_stab,
            area_strength,
        }
    }
}

impl IIcingMass for IcingMass {
    /// Масса льда попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: &Bound) -> Result<f64, Error> {
        Ok(
            self.area_strength.area_h(bound)? * self.icing_stab.mass_desc_h()
                + self.area_strength.area_timber_h(bound)?
                    * (self.icing_stab.mass_timber_h() - self.icing_stab.mass_desc_h())
                + self.area_strength.area_v(bound)?
                    * (1. + self.icing_stab.coef_v_ds_area())
                    * self.icing_stab.mass_v(),
        )
    }
}
#[doc(hidden)]
pub trait IIcingMass {
    /// Масса льда попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: &Bound) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeIcingMass {
    mass: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeIcingMass {
    pub fn new(mass: f64) -> Self {
        Self { mass }
    }
}
#[doc(hidden)]
impl IIcingMass for FakeIcingMass {
    fn mass(&self, _: &Bound) -> Result<f64, Error> {
        Ok(self.mass)
    }
}
