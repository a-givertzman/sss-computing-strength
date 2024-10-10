//! Парусность судна

use std::rc::Rc;

use crate::{icing_stab::IIcingStab, Error, Moment};

/// Парусность судна, площадь и положение 
/// центра относительно миделя и ОП
#[derive(Clone)]
pub struct Windage {
    /// Тип обледенения
    icing_stab: Rc<dyn IIcingStab>,
    /// Площади поверхности для расчета остойчивости
    area_stability: Rc<dyn crate::stability::IArea>,
    /// Разница в площадях парусности
    delta_area: f64,
    /// Разница в статических моментах относительно миделя (x) и ОП (z) 
    delta_moment: Moment,
    /// Отстояние по вертикали центра площади проекции подводной части корпуса
    volume_shift: f64, 
}
//
impl Windage {
    /// Главный конструктор: 
    /// * icing_stab - Тип обледенения
    /// * area_stability - Площади поверхности для расчета остойсивости
    /// * delta_area - Разница в площадях парусности
    /// * delta_moment - Разница в статических моментах относительно миделя (x) и ОП (z) 
    /// * volume_shift - Отстояние по вертикали центра площади проекции подводной части корпуса
    pub fn new(
        icing_stab: Rc<dyn IIcingStab>,
        area_stability: Rc<dyn crate::stability::IArea>,
        delta_area: f64,
        delta_moment: Moment,
        volume_shift: f64,  
    ) -> Self {
        Self {
            icing_stab,
            area_stability,
            delta_area,
            delta_moment, 
            volume_shift,
        }
    }
    //
    fn moment(&self) -> Result<Moment, Error> {
        Ok(self.area_stability.moment_v()?.scale(1. + self.icing_stab.coef_v_moment()))
    }
}
//
impl IWindage for Windage {
    /// Площадь парусности, м^2
    fn a_v(&self) -> Result<f64, Error> {
        let area_v = self.area_stability.area_v()?;
        let coef = 1. + self.icing_stab.coef_v_area();
        let result = area_v*coef - self.delta_area;
    //    log::info!("\t IWindage a_v area_v:{area_v} coef:{coef} delta_area:{} result:{result} ", self.delta_area);
        Ok(result)
    }    
    /// Плечо парусности, м
    fn z_v(&self) -> Result<f64, Error> {
        let m_vz = self.moment()?.z() - self.delta_moment.z();
        let a_v = self.a_v()?;
        let z_v_bp = m_vz/a_v;
        let result = z_v_bp - self.volume_shift;
    //    log::info!("\t IWindage z_v moment:{} m_vz:{m_vz} a_v:{a_v} z_v_bp:{z_v_bp} volume_shift:{} result:{result} ", self.moment()?, self.volume_shift);
        Ok(result)
    }
}
#[doc(hidden)]
pub trait IWindage {
    /// Площадь парусности, м^2
    fn a_v(&self) -> Result<f64, Error>;
    /// Плечо парусности, м
    fn z_v(&self) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeWindage {
    /// Площадь парусности, м^2
    a_v: f64,
    /// Плечо парусности, м
    z_v: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeWindage {
    pub fn new(
        a_v: f64,
        z_v: f64,
    ) -> Self {
        Self {
            a_v,
            z_v,
        }
    }
}
#[doc(hidden)]
impl IWindage for FakeWindage {
    /// Площадь парусности, м^2
    fn a_v(&self) -> Result<f64, Error> {
        Ok(self.a_v)
    }    
    /// Плечо парусности, м
    fn z_v(&self) -> Result<f64, Error> {
        Ok(self.z_v)
    }
}


