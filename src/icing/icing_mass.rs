//! Учет обледенения

use std::rc::Rc;
use crate::{Bound, Moment};
use super::IIcingStab;

/// Учет обледенения судна, расчет массы льда. 
/// Может быть без обледенения, частичным и полным.  
/// При расчете обледенения необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. При учете обледенения к массе судна добавляются  
/// масса льда на бортах, палубах, палубном грузе. Масса льда и его моменты,  
/// рассчитываются для осадки 𝑑𝑚𝑖𝑛 и распространяются на все случаи загрузки. 
#[derive(Clone)]
pub struct IcingMass {
    /// Тип обледенения
    icing_stab: Rc<dyn IIcingStab>,
    /// Распределение площади поверхностей
    area_strength: Rc<dyn crate::strength::IArea>,
    /// Площади поверхности для расчета остойсивости
    area_stability: Rc<dyn crate::stability::IArea>,
}
///
impl IcingMass {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения
    /// * area_strength - Распределение площади поверхностей
    /// * area_stability - Площади поверхности для расчета остойсивости
    pub fn new(
        icing_stab: Rc<dyn IIcingStab>,
        area_strength: Rc<dyn crate::strength::IArea>,   
        area_stability: Rc<dyn crate::stability::IArea>,
    ) -> Self {
        Self{
            icing_stab, 
            area_strength,   
            area_stability,      
        }
    }
}
///
impl IIcingMass for IcingMass {
    /// Масса льда попадающая в Bound или вся если Bound отсутствует
    fn mass(&self, bound: Option<Bound>) -> f64 {
        self.area_strength.area_desc_h(bound) * self.icing_stab.mass_desc_h() + 
        self.area_strength.area_timber_h(bound) * self.icing_stab.mass_timber_h() + 
        self.area_strength.area_v(bound) * (1. + self.icing_stab.coef_v_ds_area()) * self.icing_stab.mass_v()
    }
    /// Суммарный статический момент массы льда.
    fn moment(&self) -> Moment {
        let moment_v = self.area_stability.moment_v();
        let coef_v_ds_area = self.icing_stab.coef_v_ds_area();
        let mass_v = self.icing_stab.mass_v();
        let m_ice_v = moment_v.scale( (1. + coef_v_ds_area) * mass_v );
        let m_ice_h_desc = self.area_stability.moment_h().scale( self.icing_stab.mass_desc_h() );
        let moment_timber_h = self.area_stability.moment_timber_h();
        let delta_p_timber_h = self.icing_stab.mass_timber_h() - self.icing_stab.mass_desc_h();
        let delta_m_ice_timber_h = moment_timber_h.scale(delta_p_timber_h);
        let res = m_ice_v.clone() + m_ice_h_desc.clone() + delta_m_ice_timber_h.clone();        
        log::info!("\t IcingMass moment moment_v:{moment_v} coef_v_ds_area:{coef_v_ds_area} 
        mass_v:{mass_v} m_ice_v:{m_ice_v} m_ice_h_desc:{m_ice_h_desc} moment_timber_h:{moment_timber_h}
         delta_p_timber_h:{delta_p_timber_h} delta_m_ice_timber_h:{delta_m_ice_timber_h} res:{res}");
        res
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
        self.moment.clone()
    }
}