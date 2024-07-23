//! Распределение площади горизонтальных поверхностей и
//! площади парусности судна для осадки dmin

use std::rc::Rc;

use crate::{
    area::{HAreaStrength, VerticalArea},
    icing_timber::IcingTimberBound,
    Bound, Error, IDesk,
};

/// Распределение площади горизонтальных поверхностей и
/// площадь парусности судна для осадки dmin
#[derive(Clone)]
pub struct Area {
    /// Площадь парусности корпуса судна
    area_const_v: Vec<VerticalArea>,
    /// Площадь горизонтальных поверхностей корпуса судна
    area_const_h: Vec<HAreaStrength>,
    /// Все палубные грузы судна
    desks: Rc<Vec<Rc<dyn IDesk>>>,
    /// Ограничение для гортзонтальной площади обледенения палубного груза - леса
    icing_timber_bound: IcingTimberBound,
}
///
impl Area {
    /// Аргументы конструктора:  
    /// * area_const_v - Площадь парусности корпуса судна
    /// * area_const_h - Площадь горизонтальных поверхностей корпуса судна
    /// * desks - Все палубные грузы судна
    /// * icing_timber_bound - Ограничение для гортзонтальной площади обледенения палубного груза - леса
    pub fn new(
        area_const_v: Vec<VerticalArea>,
        area_const_h: Vec<HAreaStrength>,
        desks: Rc<Vec<Rc<dyn IDesk>>>,
        icing_timber_bound: IcingTimberBound,
    ) -> Self {
        Self {
            area_const_v,
            area_const_h,
            desks,
            icing_timber_bound,
        }
    }
}
///
impl IArea for Area {
    /// Площадь парусности для заданного диапазона, м^2
    fn area_v(&self, bound: Option<Bound>) -> Result<f64, Error> {
        let mut area_sum = 0.;
        for v in self.area_const_v.iter() {
            area_sum += v.value(bound)?;
        }
        for v in self.desks.iter() {
            area_sum += v.windage_area(bound)?;
        }
        Ok(area_sum)
    }
    /// Площадь горизонтальных поверхностей открытых палуб для  
    /// заданного диапазона, м^2
    fn area_desc_h(&self, bound: Option<Bound>) -> Result<f64, Error> {
        let mut sum = 0.;
        for v in self.area_const_h.iter() {
            sum += v.value(bound)?;
        }
        Ok(sum)
    }
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, bound_x: Option<Bound>) -> Result<f64, Error> {
        let bound_x = match (bound_x, self.icing_timber_bound.bound_x()?) {
            (None, None) => None,
            (None, Some(timber_icing_x)) => Some(timber_icing_x),
            (Some(bound_x), None) => Some(bound_x),
            (Some(bound_x), Some(timber_icing_x)) => bound_x.intersect(&timber_icing_x)?,
        };
        let mut area_sum = 0.;
        for v in self.desks.iter().filter(|v| v.is_timber()) {
            area_sum += v.horizontal_area(bound_x, self.icing_timber_bound.bound_y()?)?;
        }
        Ok(area_sum)
    }
}
#[doc(hidden)]
pub trait IArea {
    /// Площадь парусности для заданного диапазона, м^2  
    fn area_v(&self, bound: Option<Bound>) -> Result<f64, Error>;
    /// Площадь горизонтальных поверхностей открытых палуб для  
    /// заданного диапазона, м^2
    fn area_desc_h(&self, bound: Option<Bound>) -> Result<f64, Error>;
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, bound: Option<Bound>) -> Result<f64, Error>;
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
    fn area_v(&self, _: Option<Bound>) -> Result<f64, Error> {
        Ok(self.area_v)
    }
    /// Площадь горизонтальных поверхностей открытых палуб для  
    /// заданного диапазона, м^2
    fn area_desc_h(&self, _: Option<Bound>) -> Result<f64, Error> {
        Ok(self.area_desc_h)
    }
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, _: Option<Bound>) -> Result<f64, Error> {
        Ok(self.area_timber_h)
    }
}
