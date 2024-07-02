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
    /// Ограничение по оси Х для площади обледенения палубного груза - леса
    timber_icing_x: Option<Bound>,
    /// Коэффициент массы для обледенения палубного груза - леса
    timber_icing_y: f64,
}
///
impl Area {
    /// Аргументы конструктора:  
    /// * area_const_v - Площадь парусности корпуса судна
    /// * area_const_h - Площадь горизонтальных поверхностей корпуса судна
    /// * loads_cargo - Все грузы судна
    /// * timber_icing_x - Ограничение по оси Х для площади обледенения палубного груза - леса
    /// * timber_icing_y - Ограничение по оси Y для площади обледенения палубного груза - леса
    pub fn new(
        area_const_v: Vec<VerticalArea>,
        area_const_h: Vec<HAreaStrength>,
        loads_cargo: Rc<Vec<Rc<dyn IDesk>>>,
        timber_icing_x: Option<Bound>,
        timber_icing_y: f64,
    ) -> Self {
        Self {
            area_const_v,
            area_const_h,
            loads_cargo,
            timber_icing_x,
            timber_icing_y,
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
    }
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, bound_x: Option<Bound>) -> f64 {
        let bound_x = match (bound_x, self.timber_icing_x) {
            (None, None) => None,
            (None, Some(timber_icing_x)) => Some(timber_icing_x),
            (Some(bound_x), None) => Some(bound_x),
            (Some(bound_x), Some(timber_icing_x)) => bound_x.intersect(&timber_icing_x),
        };
        self.loads_cargo
            .iter()
            .filter(|v| v.is_timber())
            .map(|v| v.horizontal_area(bound_x, None)*self.timber_icing_y )
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
