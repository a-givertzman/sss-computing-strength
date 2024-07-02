//! Момент площади горизонтальных поверхностей и
//! площади парусности судна

use std::rc::Rc;

use crate::{
    area::HAreaStability, Bound, IDesk, Moment, Position
};

/// Момент площади горизонтальных поверхностей и
/// площади парусности судна
#[derive(Clone)]
pub struct Area {
    /// Площадь парусности корпуса судна для текущей осадки
    av_cs_dmin1: f64,
    /// Cтатический момент площади парусности сплошных
    /// поверхностей для текущей осадки, относительно миделя и относительно ОП
    mvx_cs_dmin1: f64,
    mvz_cs_dmin1: f64,
    /// Площадь горизонтальных поверхностей корпуса судна
    area_const_h: Vec<HAreaStability>,
    /// Все палубные грузы судна
    desk_cargo: Rc<Vec<Rc<dyn IDesk>>>,
    /// Ограничение по оси Х для площади обледенения палубного груза - леса
    timber_icing_x: Option<Bound>,
    /// Коэффициент массы для обледенения палубного груза - леса
    timber_icing_y: f64,
}
///
impl Area {
    /// * av_cs - Площадь парусности корпуса судна для текущей осадки
    /// * mvx_cs, mvz_cs - Cтатический момент площади парусности сплошных
    /// поверхностей для текущей осадки, относительно миделя и относительно ОП
    /// * area_const_h - Площадь горизонтальных поверхностей корпуса судна
    /// * desk_cargo - Все палубные грузы судна
    /// * timber_icing_x - Ограничение по оси Х для площади обледенения палубного груза - леса
    /// * timber_icing_y - Ограничение по оси Y для площади обледенения палубного груза - леса
    pub fn new(
        av_cs_dmin1: f64,
        mvx_cs_dmin1: f64,
        mvz_cs_dmin1: f64,
        area_const_h: Vec<HAreaStability>,
        desk_cargo: Rc<Vec<Rc<dyn IDesk>>>,
        timber_icing_x: Option<Bound>,
        timber_icing_y: f64,
    ) -> Self {
        Self {
            av_cs_dmin1,
            mvx_cs_dmin1,
            mvz_cs_dmin1,
            area_const_h,
            desk_cargo,
            timber_icing_x,
            timber_icing_y,
        }
    }
}
///
impl IArea for Area {
    /// Площадь парусности
    fn area_v(&self) -> f64 {
        self.av_cs_dmin1
            + self
                .desk_cargo
                .iter()
                .map(|v| v.windage_area(None))
                .sum::<f64>()
    }
    /// Момент площади парусности
    fn moment_v(&self) -> Moment {
        Moment::new(self.mvx_cs_dmin1, 0., self.mvz_cs_dmin1) + self
                .desk_cargo
                .iter()
                .map(|v| v.windage_moment())
                .sum::<Moment>()
    }
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Moment {
        self.area_const_h.iter().map(|v| v.moment()).sum::<Moment>()
            + self
                .desk_cargo
                .iter()
                .map(|v| Moment::from_pos(v.shift(), v.horizontal_area(None, None)))
                .sum::<Moment>()
    }
    /// Момент площади горизонтальных поверхностей палубного груза - леса
    fn moment_timber_h(&self) -> Moment {
        self.desk_cargo
            .iter()
            .filter(|v| v.is_timber())
            .map(|v| {
                Moment::from_pos(
                    Position::new(
                        v.shift().x(),
                        v.shift().y() ,
                        v.shift().z() + v.height() / 2.,
                    ),
                    v.horizontal_area(self.timber_icing_x, None),
                )
            })
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
    /// Момент площади горизонтальных поверхностей палубного груза - леса
    fn moment_timber_h(&self) -> Moment;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeArea {
    area_v: f64,
    moment_v: Moment,
    moment_h: Moment,
    moment_timber_h: Moment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeArea {
    pub fn new(area_v: f64, moment_v: Moment, moment_h: Moment, moment_timber_h: Moment) -> Self {
        Self {
            area_v,
            moment_v,
            moment_h,
            moment_timber_h,
        }
    }
}
#[doc(hidden)]
impl IArea for FakeArea {
    /// Площадь парусности
    fn area_v(&self) -> f64 {
        self.area_v
    }
    /// Момент площади парусности
    fn moment_v(&self) -> Moment {
        self.moment_v.clone()
    }
    /// Момент площади горизонтальных поверхностей
    fn moment_h(&self) -> Moment {
        self.moment_h.clone()
    }
    /// Момент площади горизонтальных поверхностей палубного груза - леса
    fn moment_timber_h(&self) -> Moment {
        self.moment_timber_h.clone()
    }
}
