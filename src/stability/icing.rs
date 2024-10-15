//! Учет обледенения

use crate::{icing_stab::IIcingStab, Error, Moment};
use std::rc::Rc;

/// Учет обледенения судна.
/// Может быть без обледенения, частичным и полным.  
/// При расчете обледенения необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. При учете обледенения к массе судна добавляются  
/// масса льда на бортах, палубах, палубном грузе. Масса льда и его моменты,  
/// рассчитываются для осадки 𝑑𝑚𝑖𝑛 и распространяются на все случаи загрузки.
#[derive(Clone)]
pub struct IcingMoment {
    /// Тип обледенения судна
    icing_stab: Rc<dyn IIcingStab>,
    /// Площади поверхности для расчета остойчивости
    area_stability: Rc<dyn crate::stability::IArea>,
}
//
impl IcingMoment {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения судна
    /// * area_stability - Площади поверхности для расчета остойсивости
    pub fn new(
        icing_stab: Rc<dyn IIcingStab>,
        area_stability: Rc<dyn crate::stability::IArea>,
    ) -> Self {
        Self {
            icing_stab,
            area_stability,
        }
    }
}
//
impl IIcingMoment for IcingMoment {
    /// Суммарный статический момент массы льда.
    fn moment(&self) -> Result<Moment, Error> {
        let moment_v = self.area_stability.moment_v()?;
        let coef_v_ds_area = self.icing_stab.coef_v_ds_area();
        let mass_v = self.icing_stab.mass_v();
        let m_ice_v = moment_v.scale((1. + coef_v_ds_area) * mass_v);
        let m_ice_h_desc = self
            .area_stability
            .moment_h()?
            .scale(self.icing_stab.mass_desc_h());
        let moment_timber_h = self.area_stability.moment_timber_h()?;
        let delta_moment_timber_h = self.area_stability.delta_moment_timber_h()?;
        let delta_p_timber_h = self.icing_stab.mass_timber_h() - self.icing_stab.mass_desc_h();
        let delta_m_ice_timber_h = moment_timber_h.scale(delta_p_timber_h)
            + delta_moment_timber_h.scale(self.icing_stab.mass_timber_h());
        let res = m_ice_v.clone() + m_ice_h_desc.clone() + delta_m_ice_timber_h.clone();
        log::trace!("\t IcingMass moment moment_v:{moment_v} coef_v_ds_area:{coef_v_ds_area} mass_v:{mass_v} 
            m_ice_v:{m_ice_v} m_ice_h_desc:{m_ice_h_desc} moment_timber_h:{moment_timber_h} delta_moment_timber_h:{delta_moment_timber_h}
            delta_p_timber_h:{delta_p_timber_h} delta_m_ice_timber_h:{delta_m_ice_timber_h} res:{res}");
        Ok(res)
    }
}
#[doc(hidden)]
pub trait IIcingMoment {
    /// Суммарный статический момент массы льда.
    fn moment(&self) -> Result<Moment, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeIcingMoment {
    moment: Moment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeIcingMoment {
    pub fn new(moment: Moment) -> Self {
        Self { moment }
    }
}
#[doc(hidden)]
impl IIcingMoment for FakeIcingMoment {
    fn moment(&self) -> Result<Moment, Error> {
        Ok(self.moment.clone())
    }
}
