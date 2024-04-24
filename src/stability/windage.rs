//! Парусность судна

use std::rc::Rc;

use crate::{icing::IIcingStab, Moment};

/// Парусность судна, площадь и положение 
/// центра относительно миделя и ОП
#[derive(Clone)]
pub struct Windage {
    /// Тип обледенения
    icing_stab: Rc<dyn IIcingStab>,
    /// Площади поверхности для расчета остойсивости
    area_stability: Rc<dyn crate::stability::IArea>,
    /// Разница в площадях парусности
    delta_area: f64,
    /// Разница в статических моментах относительно миделя (x) и ОП (z) 
    delta_moment: Moment,
    /// Отстояние по вертикали центра площади проекции подводной части корпуса
    volume_shift: f64, 
    /// Площадь парусности, м^2
    a_v: Option<f64>,      
    /// Плечо парусности, м
    z_v: Option<f64>, 
}
///
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
            a_v: None,            
            z_v: None,
        }
    }
    ///
    fn calculate(&mut self) {
        // Площадь парусности и моменты судна с учетом поправки
        let a_v = self.area_stability.area_v()*self.icing_stab.coef_v_area() - self.delta_area;
        let moment = self.area_stability.moment_v().scale(self.icing_stab.coef_v_moment());
        let m_vx = moment.x() - self.delta_moment.x();
        let m_vz = moment.z() - self.delta_moment.z();

        let x_v = m_vx/a_v;
        let z_v_bp = m_vz/a_v;
        let z_v = z_v_bp - self.volume_shift;
        self.a_v = Some(a_v);
        self.z_v = Some(z_v);

        log::info!("Windage a_v:{a_v} m_vx:{m_vx} m_vz:{m_vz} x_v:{x_v} z_v:{z_v}");
    }
}
///
impl IWindage for Windage {
    /// Площадь парусности, м^2
    fn a_v(&mut self) -> f64 {
        if self.a_v.is_none() {
            self.calculate();
        }

        self.a_v.expect("Windage a_v error: no value!")
    }    
    /// Плечо парусности, м
    fn z_v(&mut self) -> f64 {
        if self.z_v.is_none() {
            self.calculate();
        }

        self.z_v.expect("Windage z_v error: no value!")
    }
}
#[doc(hidden)]
pub trait IWindage {
    /// Площадь парусности, м^2
    fn a_v(&mut self) -> f64;
    /// Плечо парусности, м
    fn z_v(&mut self) -> f64;
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
    fn a_v(&mut self) -> f64 {
        self.a_v
    }    
    /// Плечо парусности, м
    fn z_v(&mut self) -> f64 {
        self.z_v
    }
}


