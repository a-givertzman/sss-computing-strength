//! Учет обледенения

use std::rc::Rc;
use crate::{Area, Bound, ILoad, Position};
use super::IIcingStab;

/// Учет обледенения судна, расчет массы льда. 
/// Может быть без обледенения, частичным и полным.  
/// При расчете обледенения необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. При учете обледенения к массе судна добавляются  
/// масса льда на бортах, палубах, палубном грузе. Масса льда и его моменты,  
/// рассчитываются для осадки 𝑑𝑚𝑖𝑛 и распространяются на все случаи загрузки. 
pub struct Icing {
    /// Тип обледенения
    icing_stab: Rc<dyn IIcingStab>,
    /// Площадь горизонтальных поверхностей
    area_h: Vec<Area>,
    /// Площадь поверхности парусности
    area_v: Vec<Area>,    
    /// Все грузы судна
    loads_cargo: Rc<Vec<Rc<Box<dyn ILoad>>>>,
}
///
impl Icing {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения
    /// * icing_area_h - Площадь горизонтальных поверхностей
    /// * icing_area_v - Площадь поверхности парусности    
    /// * loads_cargo - Грузы судна
    pub fn new(
        icing_stab: Rc<dyn IIcingStab>,
        area_h: Vec<Area>,
        area_v: Vec<Area>,        
        loads_cargo: Rc<Vec<Rc<Box<dyn ILoad>>>>,
    ) -> Self {
        Self{
            icing_stab, 
            area_h,
            area_v,            
            loads_cargo,
        }
    }
}
///
impl IIcing for Icing {
    /// Суммарная масса льда попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: Option<Bound>) -> f64 {
        self.area_h.iter().map(|v| v.value(bound) ).sum::<f64>() * self.icing_stab.mass_h() + 
        self.area_v.iter().map(|v| v.value(bound) ).sum::<f64>() * self.icing_stab.mass_v() +
        self.loads_cargo.iter().map(|v| v.windage_area(bound) ).sum::<f64>() * self.icing_stab.mass_h()
    }
    /// Суммарный статический момент
    fn moment_mass(&self) -> Moment {
        self
                    .loads_cargo
                    .iter()
                    .map(|c| c.moment_mass())
                    .sum::<Moment>()
                    
        self.area_h.iter().map(|v| v.value(bound) ).sum::<f64>() * self.icing_stab.mass_h() + 
        self.area_v.iter().map(|v| v.value(bound) ).sum::<f64>() * self.icing_stab.mass_v() +
        self.loads_cargo.iter().map(|v| v.windage_area(bound) ).sum::<f64>() * self.icing_stab.mass_h()
    }
    /// Отстояние центра масс
    fn shift(&self) -> Position {
        self.moment_mass().to_pos(self.mass(None))
    }
}
#[doc(hidden)]
pub trait IIcing {
    /// Суммарная масса льда попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: Option<Bound>) -> f64;
    /// Суммарный статический момент
    fn moment_mass(&self) -> Moment;
    /// Отстояние центра масс
    fn shift(&self) -> Position;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeIcing {
    mass: f64,
    moment_mass: Moment,
    shift: Position,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeIcing {
    pub fn new(
        mass: f64,
        moment_mass: Moment,
        shift: Position,
    ) -> Self {
        Self {
            mass,
            moment_mass,
            shift,
        }
    }
}
#[doc(hidden)]
impl IIcing for FakeIcing {
    fn mass(&self, _: Option<Bound>) -> f64 {
        self.mass
    }
    fn moment_mass(&self) -> Moment {
        self.moment_mass
    }
    fn shift(&self) -> Position {
        self.shift
    }
}



