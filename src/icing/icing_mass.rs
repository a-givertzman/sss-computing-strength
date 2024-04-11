//! Учет обледенения

use std::rc::Rc;
use crate::{Bound, ILoad, icing::IcingArea};
use super::{IcingStab};

/// Учет обледенения судна, расчет массы льда. 
/// Может быть без обледенения, частичным и полным.  
/// При расчете обледенения необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. При учете обледенения к массе судна добавляются  
/// масса льда на бортах, палубах, палубном грузе. Масса льда и его моменты,  
/// рассчитываются для осадки 𝑑𝑚𝑖𝑛 и распространяются на все случаи загрузки. 
pub struct IcingMass {
    /// Тип обледенения
    icing_stab: IcingStab,
    /// Площадь обледенения горизонтальных поверхностей
    icing_area_h: Vec<IcingArea>,
    /// Площадь обледенения поверхности парусности
    icing_area_v: Vec<IcingArea>,    
    /// Все грузы судна
    loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
}
///
impl IcingMass {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения
    /// * icing_area_h - Площадь обледенения горизонтальных поверхностей
    /// * icing_area_v - Площадь обледенения поверхности парусности    
    /// * loads_cargo - Грузы судна
    pub fn new(
        icing_stab: IcingStab,
        icing_area_h: Vec<IcingArea>,
        icing_area_v: Vec<IcingArea>,        
        loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
    ) -> Self {
        Self{
            icing_stab, 
            icing_area_h,
            icing_area_v,            
            loads_cargo,
        }
    }
    /// Суммарная масса льда попадающая в Bound или вся если Bound отсутствует
    pub fn mass(&self, bound: Option<Bound>) -> f64 {
        self.icing_area_h.iter().map(|v| v.area(bound) ).sum::<f64>() * self.icing_stab.mass_h() + 
        self.icing_area_v.iter().map(|v| v.area(bound) ).sum::<f64>() * self.icing_stab.mass_v() +
        self.loads_cargo.iter().map(|v| v.windage_area(bound) ).sum::<f64>() * self.icing_stab.mass_h()
    }
}
///
impl IIcingMass for IcingMass {
}
#[doc(hidden)]
pub trait IIcingMass {
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeIcingMass {

}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeIcingMass {
    pub fn new(

    ) -> Self {
        Self {

        }
    }
}



