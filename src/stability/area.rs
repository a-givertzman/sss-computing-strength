//! Момент площади горизонтальных поверхностей и
//! площади парусности судна

use std::rc::Rc;

use crate::{ILoad, Moment};

/// Момент площади горизонтальных поверхностей и
/// площади парусности судна
#[derive(Clone)]
pub struct Area {
    /// Площадь парусности корпуса судна
    area_const_v: Vec<crate::math::Area>,
    /// Площадь горизонтальных поверхностей корпуса судна
    area_const_h: Vec<crate::math::Area>,
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
        area_const_v: Vec<crate::math::Area>,
        area_const_h: Vec<crate::math::Area>,
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
    /// Момент площади парусности
    fn moment_v(&self) -> Moment {
        (self.area_const_v.iter().map(|v| v.moment()).sum::<Moment>()
            + self
                .loads_cargo
                .iter()
                .map(|v| v.windage_moment())
                .sum::<Moment>())
        .scale(1.05)
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
    /// Момент площади парусности
    fn moment_v(&self) -> Moment;
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Moment;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeArea {
    moment_v: Moment,
    moment_h: Moment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeArea {
    pub fn new(moment_v: Moment, moment_h: Moment) -> Self {
        Self { moment_v, moment_h }
    }
}
#[doc(hidden)]
impl IArea for FakeArea {
    /// Момент площади парусности
    fn moment_v(&self) -> Moment {
        self.moment_v
    }
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Moment {
        self.moment_h
    }
}
