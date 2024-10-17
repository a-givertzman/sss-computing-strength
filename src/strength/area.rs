//! Распределение площади горизонтальных поверхностей и
//! площади парусности судна для осадки dmin
use crate::{
    area::{HAreaStrength, VerticalArea}, icing_timber::IcingTimberBound, Bound, Bounds, Error, IDesk
};
use std::rc::Rc;
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
//
impl Area {
    /// Аргументы конструктора:  
    /// * area_const_v - Площадь парусности корпуса судна
    /// * area_const_h - Площадь горизонтальных поверхностей корпуса судна
    /// * desks - Все палубные грузы судна
    /// * icing_timber_bound - Ограничение для горизонтальной площади обледенения палубного груза - леса
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
//
impl IArea for Area {
    /// Площадь парусности для заданного диапазона, м^2
    fn area_v(&self, bound: &Bound) -> Result<f64, Error> {
        let mut area_sum = 0.;
        for v in self.area_const_v.iter() {
            area_sum += v.value(bound)?;
        }
        // Ищем площадь горизонтальной поверхности палубных грузов.
        // Перебираем горизонтальую поверхность с шагом, проходим по грузам и
        // берем максимальную площадь среди всех грузов на этом шаге.
        let min_x = self
            .desks
            .iter()
            .filter_map(|v| v.min_x())
            .min_by(|&a, &b| a.partial_cmp(&b).unwrap());
        let max_x = self
            .desks
            .iter()
            .filter_map(|v| v.max_x())
            .max_by(|&a, &b| a.partial_cmp(&b).unwrap());
        if min_x.is_some() && max_x.is_some() {
            let bound = bound.intersect(&Bound::new(min_x.unwrap(), max_x.unwrap())?)?;
            if bound.is_some() {
                let (min_x, max_x) = (
                    bound.start().ok_or(Error::FromString(
                        "Area area_v error: bound.start()".to_owned(),
                    ))?,
                    bound.end().ok_or(Error::FromString(
                        "Area area_v error: bound.end()".to_owned(),
                    ))?,
                );
                area_sum += Bounds::from_min_max(min_x, max_x, 200)?.iter().map(|bound_x| {
                    let min_z = self
                        .desks
                        .iter()
                        .filter_map(|v| v.min_z())
                        .min_by(|&a, &b| a.partial_cmp(&b).unwrap());
                    let max_z = self
                        .desks
                        .iter()
                        .filter_map(|v| v.max_z())
                        .max_by(|&a, &b| a.partial_cmp(&b).unwrap());
                    if let (Some(min_z), Some(max_z)) = (min_z, max_z) {
                        Bounds::from_min_max(min_z, max_z, 50).expect("Area area_v error: Bounds::from_min_max").iter().map(|bound_z| 
                            self
                                .desks
                                .iter()
                                .filter_map(|v| v.windage_area(&bound_x, &bound_z).ok())
                                .max_by(|&a, &b| a.partial_cmp(&b).unwrap())
                                .unwrap_or(0.)   
                        ).sum()                 
                    } else {
                        0.
                    }
                }).sum::<f64>()
            }
        }
        Ok(area_sum)
    }
    /// Площадь горизонтальных поверхностей открытых палуб для  
    /// заданного диапазона, м^2
    fn area_h(&self, bound: &Bound) -> Result<f64, Error> {
        let mut sum = 0.;
        for v in self.area_const_h.iter() {
            sum += v.value(bound)?;
        }
        Ok(sum)
    }
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, bound_x: &Bound) -> Result<f64, Error> {
        let mut area_sum = 0.;
        for v in self.desks.iter().filter(|v| v.is_timber()) {
            area_sum += v.horizontal_area(
                &bound_x.intersect(&self.icing_timber_bound.bound_x()?)?,
                &self.icing_timber_bound.bound_y()?,
            )?;
        }
        Ok(area_sum)
    }
}
#[doc(hidden)]
pub trait IArea {
    /// Площадь парусности для заданного диапазона, м^2  
    fn area_v(&self, bound: &Bound) -> Result<f64, Error>;
    /// Площадь горизонтальных поверхностей открытых палуб для  
    /// заданного диапазона, м^2
    fn area_h(&self, bound: &Bound) -> Result<f64, Error>;
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, bound: &Bound) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeArea {
    area_v: f64,
    area_h: f64,
    area_timber_h: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeArea {
    pub fn new(area_v: f64, area_h: f64, area_timber_h: f64) -> Self {
        Self {
            area_v,
            area_h,
            area_timber_h,
        }
    }
}
#[doc(hidden)]
impl IArea for FakeArea {
    /// Площадь парусности для заданного диапазона, м^2
    fn area_v(&self, _: &Bound) -> Result<f64, Error> {
        Ok(self.area_v)
    }
    /// Площадь горизонтальных поверхностей открытых палуб для  
    /// заданного диапазона, м^2
    fn area_h(&self, _: &Bound) -> Result<f64, Error> {
        Ok(self.area_h)
    }
    /// Площадь горизонтальных поверхностей палубного лесного  
    /// груза для заданного диапазона, м^2  
    fn area_timber_h(&self, _: &Bound) -> Result<f64, Error> {
        Ok(self.area_timber_h)
    }
}
