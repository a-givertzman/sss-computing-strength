//! Парусность судна для осадки dmin

use std::rc::Rc;

use crate::{icing::IIcingStab, ILoad, Moment, Position};

/// Парусность судна для осадки dmin
#[derive(Clone)]
pub struct AreaDmin {
    /// Все грузы судна
    loads_cargo: Rc<Vec<Rc<Box<dyn ILoad>>>>,
    /// Площадь парусности судна для минимальной осадки
    area_const: f64,
    /// Площадь парусности, м^2
    area_full: Option<f64>,
}
///
impl AreaDmin {
    /// Аргументы конструктора:  
    /// * loads_cargo - грузы судна
    /// * area_const - Площадь парусности судна
    pub fn new(
        loads_cargo: Rc<Vec<Rc<Box<dyn ILoad>>>>,
        area_const: f64,
    ) -> Self {
        assert!(area_const >= 0., "area_const {area_const} >= 0");
        Self {
            loads_cargo,
            area_const,
            area_full: None,            
          }
    }
    ///
    fn calculate(&mut self) {
        // Площадь парусности сплошных поверхностей 
        // для минимальной осадки без палубного груза
        let a_v_cs_dmin1 = self.area_const;

        // Площадь парусности палубного груза
        let a_v_pg = self.loads_cargo.iter().map(|l| l.windage_area(None)).sum();

        // Суммарная площадь для минимальной осадки
        let a_v_cs_dmin = a_v_cs_dmin1 + a_v_pg;
    
        // Парусность несплошных поверхностей
        let a_v_ds = 0.05*a_v_cs_dmin;

        // Площадь парусности и моменты судна для минимальной осадки 
        let a_v_dmin = a_v_cs_dmin + a_v_ds;

        self.area_full = Some(a_v_dmin);         

        log::info!("AreaDmin a_v_cs_dmin1:{a_v_cs_dmin1} a_v_pg:{a_v_pg} a_v_cs_dmin:{a_v_cs_dmin} a_v_ds:{a_v_ds} a_v_dmin:{a_v_dmin}");
    }
}
///
impl IArea for AreaDmin {
    /// Площадь парусности, м^2
    fn area(&mut self) -> f64 {
        if self.area_full.is_none() {
            self.calculate();
        }
        self.area_full.expect("AreaDmin a_v error: no value!")
    }    
}
#[doc(hidden)]
pub trait IArea {
    /// Площадь парусности, м^2
    fn area(&mut self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeAreaDmin {
    /// Площадь парусности, м^2
    area: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeAreaDmin {
    pub fn new(
        area: f64,
    ) -> Self {
        Self {
            area,
        }
    }
}
#[doc(hidden)]
impl IArea for FakeAreaDmin {
    /// Площадь парусности, м^2
    fn area(&mut self) -> f64 {
        self.area
    }    
}


