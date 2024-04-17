//! Момент площади горизонтальных поверхностей и
//! площади парусности судна

use std::rc::Rc;

use crate::{area::{HAreaStability, VerticalArea}, ILoad, Moment};

/// Момент площади горизонтальных поверхностей и
/// площади парусности судна
#[derive(Clone)]
pub struct Area {
    /// Площадь парусности корпуса судна
    area_const_v: Vec<VerticalArea>,
    /// Площадь горизонтальных поверхностей корпуса судна
    area_const_h: Vec<HAreaStability>,
    /// Все грузы судна
    loads_cargo: Rc<Vec<Rc<Box<dyn ILoad>>>>,
}
///
impl Area {
    /// Аргументы конструктора:  
    /// * area_const_v - Площадь парусности корпуса судна
    /// * area_const_h - Площадь горизонтальных поверхностей корпуса судна
    /// * loads_cargo - Все грузы судна
    pub fn new(
        area_const_v: Vec<VerticalArea>,
        area_const_h: Vec<HAreaStability>,
        loads_cargo: Rc<Vec<Rc<Box<dyn ILoad>>>>,
    ) -> Self {
        Self {
            area_const_v,
            area_const_h,
            loads_cargo,
        }
    }
}
///
impl IArea for Area {
    /// Площадь парусности
    fn area_v(&self) -> f64 {
        (self.area_const_v.iter().map(|v| v.value(None)).sum::<f64>()
            + self
                .loads_cargo
                .iter()
                .map(|v| v.windage_area(None))
                .sum::<f64>()) * 1.05        
    }
    /// Момент площади парусности
    fn moment_v(&self) -> Moment {
        (self.area_const_v.iter().map(|v| v.moment()).sum::<Moment>()
            + self
                .loads_cargo
                .iter()
                .map(|v| v.windage_moment())
                .sum::<Moment>())
        .scale(1.1)
    }
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Moment {
        self.area_const_h.iter().map(|v| v.moment()).sum::<Moment>()
            + self
                .loads_cargo
                .iter()
                .map(|v| Moment::new(0., 0., v.horizontal_area(None) * v.height()))
                .sum::<Moment>()
    }
}
#[doc(hidden)]
pub trait IArea {
    /// Площадь парусности
    fn area_v(&self) -> f64;
    /// Момент площади парусности
    fn moment_v(&self) -> Moment;
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Moment;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeArea {
    area_v: f64,
    moment_v: Moment,
    moment_h: Moment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeArea {
    pub fn new(area_v: f64, moment_v: Moment, moment_h: Moment) -> Self {
        Self { area_v, moment_v, moment_h }
    }
}
#[doc(hidden)]
impl IArea for FakeArea {
    /// Площадь парусности
    fn area_v(&self) -> f64 {
        self.area_v.clone()
    }
    /// Момент площади парусности
    fn moment_v(&self) -> Moment {
        self.moment_v.clone()
    }
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Moment {
        self.moment_h.clone()
    }
}
