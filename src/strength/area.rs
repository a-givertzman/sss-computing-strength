//! Распределение площади горизонтальных поверхностей и
//! площади парусности судна для осадки dmin

use std::rc::Rc;

use crate::{
    area::{HAreaStrength, VerticalArea},
    Bound, IDesk,
};

/// Распределение площади горизонтальных поверхностей и
/// площадь парусности судна для осадки dmin
#[derive(Clone)]
pub struct Area {
    /// Площадь парусности корпуса судна
    area_const_v: Vec<VerticalArea>,
    /// Площадь горизонтальных поверхностей корпуса судна
    area_const_h: Vec<HAreaStrength>,
    /// Все грузы судна
    loads_cargo: Rc<Vec<Rc<dyn IDesk>>>,
}
///
impl Area {
    /// Аргументы конструктора:  
    /// * area_const_v - Площадь парусности корпуса судна
    /// * area_const_h - Площадь горизонтальных поверхностей корпуса судна
    /// * loads_cargo - Все грузы судна
    pub fn new(
        area_const_v: Vec<VerticalArea>,
        area_const_h: Vec<HAreaStrength>,
        loads_cargo: Rc<Vec<Rc<dyn IDesk>>>,
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
    /// Площадь парусности для заданного диапазона, м^2
    fn area_v(&self, bound: Option<Bound>) -> f64 {
        self.area_const_v
            .iter()
            .map(|v| v.value(bound))
            .sum::<f64>()
            + self
                .loads_cargo
                .iter()
                .map(|v| v.windage_area(bound))
                .sum::<f64>()
    }
    /// Площадь горизонтальных поверхностей открытых палуб для  
    /// заданного диапазона, м^2
    fn area_desc_h(&self, bound: Option<Bound>) -> f64 {
        self.area_const_h
            .iter()
            .map(|v| v.value(bound))
            .sum::<f64>()
            + self
                .loads_cargo
                .iter()
      //          .filter(|v| !v.is_timber())
                .map(|v| v.horizontal_area(bound))
                .sum::<f64>()
    }
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, bound: Option<Bound>) -> f64 {
        self.loads_cargo
            .iter()
            .filter(|v| v.is_timber())
            .map(|v| v.horizontal_area(bound))
            .sum::<f64>()
    }
}
#[doc(hidden)]
pub trait IArea {
    /// Площадь парусности для заданного диапазона, м^2  
    fn area_v(&self, bound: Option<Bound>) -> f64;
    /// Площадь горизонтальных поверхностей открытых палуб для  
    /// заданного диапазона, м^2
    fn area_desc_h(&self, bound: Option<Bound>) -> f64;
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, bound: Option<Bound>) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeArea {
    area_v: f64,
    area_desc_h: f64,
    area_timber_h: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeArea {
    pub fn new(area_v: f64, area_desc_h: f64, area_timber_h: f64) -> Self {
        Self {
            area_v,
            area_desc_h,
            area_timber_h,
        }
    }
}
#[doc(hidden)]
impl IArea for FakeArea {
    /// Площадь парусности для заданного диапазона, м^2
    fn area_v(&self, _: Option<Bound>) -> f64 {
        self.area_v
    }
    /// Площадь горизонтальных поверхностей открытых палуб для  
    /// заданного диапазона, м^2
    fn area_desc_h(&self, _: Option<Bound>) -> f64 {
        self.area_desc_h
    }
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, _: Option<Bound>) -> f64 {
        self.area_timber_h
    }
}
