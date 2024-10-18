//! Момент площади горизонтальных поверхностей и
//! площади парусности судна

use std::rc::Rc;

use crate::{
    area::HAreaStability, icing_timber::IcingTimberBound, Bound, Bounds, Error, IDesk, Moment, Position
};

/// Момент площади горизонтальных поверхностей и
/// площади парусности судна
#[derive(Clone)]
pub struct Area {
    /// Площадь парусности корпуса судна для минимальной осадки
    av_cs_dmin1: f64,
    /// Cтатический момент площади парусности сплошных
    /// поверхностей для минимальной осадки, относительно миделя и относительно ОП
    mvx_cs_dmin1: f64,
    mvz_cs_dmin1: f64,
    /// Площадь горизонтальных поверхностей корпуса судна
    area_const_h: Vec<HAreaStability>,
    /// Все палубные грузы судна
    desks: Rc<Vec<Rc<dyn IDesk>>>,
    /// Ограничение для горизонтальной площади обледенения палубного груза - леса
    icing_timber_bound: IcingTimberBound,
}
//
impl Area {
    /// * av_cs_dmin1 - Площадь парусности корпуса судна для минимальной осадки
    /// * mvx_cs_dmin1, mvz_cs_dmin1 - Cтатический момент площади парусности сплошных поверхностей для минимальной осадки
    /// * area_const_h - Площадь горизонтальных поверхностей корпуса судна
    /// * desks - Все палубные грузы судна
    /// * icing_timber_bound - Ограничение для гортзонтальной площади обледенения палубного груза - леса
    pub fn new(
        av_cs_dmin1: f64,
        mvx_cs_dmin1: f64,
        mvz_cs_dmin1: f64,
        area_const_h: Vec<HAreaStability>,
        desks: Rc<Vec<Rc<dyn IDesk>>>,
        icing_timber_bound: IcingTimberBound,
    ) -> Self {
        Self {
            av_cs_dmin1,
            mvx_cs_dmin1,
            mvz_cs_dmin1,
            area_const_h,
            desks,
            icing_timber_bound,
        }
    }
}
//
impl IArea for Area {
    /// Площадь парусности
    fn area_v(&self) -> Result<f64, Error> {
        let mut area_sum = self.av_cs_dmin1;
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
            area_sum += Bounds::from_min_max(min_x.unwrap(), max_x.unwrap(), 200)?.iter().map(|bound_x| {
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
        Ok(area_sum)
    }
    /// Момент площади парусности
    fn moment_v(&self) -> Result<Moment, Error> {
        let mut moment_sum = Moment::new(self.mvx_cs_dmin1, 0., self.mvz_cs_dmin1);
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
            moment_sum += Bounds::from_min_max(min_x.unwrap(), max_x.unwrap(), 200)?.iter().map(|bound_x| {
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
                    Bounds::from_min_max(min_z, max_z, 50).expect("Area area_v error: Bounds::from_min_max").iter().map(|bound_z| {
                            let area = self
                                .desks
                                .iter()
                                .filter_map(|v| v.windage_area(&bound_x, &bound_z).ok())
                                .max_by(|&a, &b| a.partial_cmp(&b).unwrap())
                                .unwrap_or(0.);
                            Moment::new(area*bound_x.center().unwrap_or(0.), 0., area*bound_z.center().unwrap_or(0.))
                        }
                    ).sum::<Moment>()                 
                } else {
                    Moment::zero()
                }
            }).sum::<Moment>()
        }
       /* for v in self.desks.iter() {
            moment_sum += v.windage_moment()?;
        }*/
        Ok(moment_sum)
    }
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Result<Moment, Error> {
        let mut moment_sum = self.area_const_h.iter().map(|v| v.moment()).sum::<Moment>();
        for v in self.desks.iter() {
            moment_sum += Moment::from_pos(
                Position::new(v.shift().x(), v.shift().y(), v.height()?),
                v.horizontal_area(&Bound::Full, &Bound::Full)?,
            );
        }
        Ok(moment_sum)
    }
    /// Момент площади горизонтальных поверхностей палубного груза - леса
    fn moment_timber_h(&self) -> Result<Moment, Error> {
        let mut moment_sum = Moment::new(0., 0., 0.);
        for v in self.desks.iter().filter(|v| v.is_timber()) {
            moment_sum += Moment::from_pos(
                Position::new(
                    v.shift().x(),
                    v.shift().y(),
                    v.shift().z() + v.height()? / 2.,
                ),
                v.horizontal_area(&Bound::Full, &Bound::Full)?,
            );
        }
        Ok(moment_sum)
    }
    /// Изменение момента площади горизонтальных поверхностей палубного груза - леса
    /// относительно палубы
    fn delta_moment_timber_h(&self) -> Result<Moment, Error> {
        let mut moment_sum = Moment::new(0., 0., 0.);
        for v in self.desks.iter().filter(|v| v.is_timber()) {
            moment_sum += Moment::from_pos(
                Position::new(v.shift().x(), v.shift().y(), v.height()?),
                v.horizontal_area(
                    &self.icing_timber_bound.bound_x()?,
                    &self.icing_timber_bound.bound_y()?,
                )?,
            );
        }
        Ok(moment_sum)
    }
}
#[doc(hidden)]
pub trait IArea {
    /// Площадь парусности
    fn area_v(&self) -> Result<f64, Error>;
    /// Момент площади парусности
    fn moment_v(&self) -> Result<Moment, Error>;
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Result<Moment, Error>;
    /// Момент площади горизонтальных поверхностей палубного груза - леса
    fn moment_timber_h(&self) -> Result<Moment, Error>;
    /// Изменение момента площади горизонтальных поверхностей палубного груза - леса
    /// относительно палубы
    fn delta_moment_timber_h(&self) -> Result<Moment, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeArea {
    area_v: f64,
    moment_v: Moment,
    moment_h: Moment,
    moment_timber_h: Moment,
    delta_moment_timber_h: Moment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeArea {
    pub fn new(
        area_v: f64,
        moment_v: Moment,
        moment_h: Moment,
        moment_timber_h: Moment,
        delta_moment_timber_h: Moment,
    ) -> Self {
        Self {
            area_v,
            moment_v,
            moment_h,
            moment_timber_h,
            delta_moment_timber_h,
        }
    }
}
#[doc(hidden)]
impl IArea for FakeArea {
    /// Площадь парусности
    fn area_v(&self) -> Result<f64, Error> {
        Ok(self.area_v)
    }
    /// Момент площади парусности
    fn moment_v(&self) -> Result<Moment, Error> {
        Ok(self.moment_v.clone())
    }
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Result<Moment, Error> {
        Ok(self.moment_h.clone())
    }
    /// Момент площади горизонтальных поверхностей палубного груза - леса
    fn moment_timber_h(&self) -> Result<Moment, Error> {
        Ok(self.moment_timber_h.clone())
    }
    /// Изменение момента площади горизонтальных поверхностей палубного груза - леса
    /// относительно палубы
    fn delta_moment_timber_h(&self) -> Result<Moment, Error> {
        Ok(self.delta_moment_timber_h.clone())
    }
}
