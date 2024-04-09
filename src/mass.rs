//! Нагрузка на корпус судна
use std::{cell::RefCell, rc::Rc};

use crate::math::*;

use super::load::ILoad;

/// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
#[derive(Clone)]
pub struct Mass {
    /// Постоянная масса судна распределенная по шпациям
    loads_const: Vec<Rc<Box<dyn ILoad>>>,
    /// Смещение постоянный массы судна
    shift_const: Position,
    /// Все грузы судна
    loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
    /// Вектор разбиения на отрезки для эпюров
    bounds: Rc<Bounds>,
    /// Суммарный статический момент
    moment_mass: Rc<RefCell<Option<Moment>>>,
    /// Суммарный момент свободной поверхности
    moment_surface: Rc<RefCell<Option<SurfaceMoment>>>,
    /// Суммарная масса
    sum: Rc<RefCell<Option<f64>>>,
    /// Распределение массы по вектору разбиения
    values: Rc<RefCell<Option<Vec<f64>>>>,
    /// Отстояние центра масс
    shift: Rc<RefCell<Option<Position>>>,
    /// Поправка к продольной метацентрической высоте на влияние  
    /// свободной поверхности жидкости в цистернах
    delta_m_h: Rc<RefCell<Option<DeltaMH>>>,
}
///
impl Mass {
    /// Аргументы конструктора:  
    /// * loads_const - постоянная масса судна распределенная по шпациям
    /// * shift_const - смещение постоянный массы судна
    /// * loads_stock - масса запасов судна распределенная по шпациям
    /// * shift_stock - смещение массы запасов судна
    /// * loads_cargo - грузы судна
    /// * bounds - вектор разбиения на отрезки для эпюров
    pub fn new(
        loads_const: Vec<Rc<Box<dyn ILoad>>>,
        shift_const: Position,
        loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
        bounds: Rc<Bounds>,
    ) -> Self {
        Self {
            loads_const,
            shift_const,
            loads_cargo,
            bounds,
            moment_mass: Rc::new(RefCell::new(None)),
            moment_surface: Rc::new(RefCell::new(None)),
            sum: Rc::new(RefCell::new(None)),
            values: Rc::new(RefCell::new(None)),
            shift: Rc::new(RefCell::new(None)),
            delta_m_h: Rc::new(RefCell::new(None)),
        }
    }
}
///
impl IMass for Mass {
    /// Суммарная масса
    fn sum(&self) -> f64 {
        if self.sum.borrow().is_none() {
            let res = self.loads_const.iter().map(|v| v.mass(None)).sum::<f64>()
                + self.loads_cargo.iter().map(|v| v.mass(None)).sum::<f64>();
            log::info!("\t Mass sum:{res} ");
            *self.sum.borrow_mut() = Some(res);
        }
        self.sum.borrow().clone().expect("Mass sum error: no value")
    }
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64> {
        if self.values.borrow().is_none() {
            let res: Vec<f64> = self
                .bounds
                .iter()
                .map(|b| {
                    self.loads_const
                        .iter()
                        .map(|v| v.mass(Some(*b)))
                        .sum::<f64>()
                        + self
                            .loads_cargo
                            .iter()
                            .map(|v| v.mass(Some(*b)))
                            .sum::<f64>()
                })
                .collect();
            log::info!("\t Mass values:{:?} ", res);
            *self.values.borrow_mut() = Some(res);
        }
        self.values
            .borrow()
            .clone()
            .expect("Mass values error: no values")
    }
    /// Отстояние центра масс
    fn shift(&self) -> Position {
        if self.shift.borrow().is_none() {
            let res = self.moment_mass().to_pos(self.sum());
            log::info!("\t Mass shift:{res} ");
            *self.shift.borrow_mut() = Some(res);
        }
        self.shift
            .borrow()
            .clone()
            .expect("Mass shift error: no value")
    }
    /// Поправка к продольной метацентрической высоте на влияние  
    /// свободной поверхности жидкости в цистернах (2)
    fn delta_m_h(&self) -> DeltaMH {
        if self.delta_m_h.borrow().is_none() {
            assert!(self.sum() > 0., "Mass delta_m_h sum > 0");
            let res = DeltaMH::from_moment(self.moment_surface(), self.sum());
            log::info!("\t Mass delta_m_h:({}, {})", res.long(), res.cross());
            *self.delta_m_h.borrow_mut() = Some(res);
        }
        self.delta_m_h
            .borrow()
            .clone()
            .expect("Mass delta_m_h error: no value")
    }
    /// Суммарный статический момент. Для постоянной массы и для запасов считается по
    /// заданным значениям смещения центра масс
    fn moment_mass(&self) -> Moment {
        if self.moment_mass.borrow().is_none() {
            let res = self.loads_const.iter().map(|c: &Rc<Box<dyn ILoad>>| Moment::from_pos(self.shift_const.clone(), c.mass(None)) ).sum::<Moment>() +
        //    let res = self.loads_const.iter().map(|c: &Rc<Box<dyn ILoad>>| c.moment_mass() ).sum::<MassMoment>() +
            self.loads_cargo.iter().map(|c| c.mass_moment() ).sum::<Moment>();
            log::info!("\t Mass moment_mass:{res} ");
            *self.moment_mass.borrow_mut() = Some(res);
        }
        self.moment_mass
            .borrow()
            .clone()
            .expect("Mass moment_mass error: no value")
    }
    /// Суммарный момент свободной поверхности
    fn moment_surface(&self) -> SurfaceMoment {
        if self.moment_surface.borrow().is_none() {
            let res = self
                .loads_cargo
                .iter()
                .map(|c| c.moment_surface())
                .sum::<SurfaceMoment>();
            log::info!("\t Mass moment_surface:{res} ");
            *self.moment_surface.borrow_mut() = Some(res);
        }
        self.moment_surface
            .borrow()
            .clone()
            .expect("Mass moment_surface error: no value")
    }
}

#[doc(hidden)]
pub trait IMass {
    /// Суммарная масса
    fn sum(&self) -> f64;
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64>;
    /// Отстояние центра масс
    fn shift(&self) -> Position;
    /// Поправка к продольной метацентрической высоте на  
    /// влияние свободной поверхности жидкости в цистернах
    fn delta_m_h(&self) -> DeltaMH;
    /// Суммарный статический момент. Для постоянной массы и для запасов считается по
    /// заданным значениям смещения центра масс
    fn moment_mass(&self) -> Moment;
    /// Суммарный момент свободной поверхности
    fn moment_surface(&self) -> SurfaceMoment;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeMass {
    sum: f64,
    values: Vec<f64>,
    shift: Position,
    delta_m_h: DeltaMH,
    moment_mass: Moment,
    moment_surface: SurfaceMoment,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeMass {
    pub fn new(
        sum: f64,
        values: Vec<f64>,
        shift: Position,
        delta_m_h: DeltaMH,
        moment_mass: Moment,
        moment_surface: SurfaceMoment,
    ) -> Self {
        Self {
            sum,
            values,
            shift,
            delta_m_h,
            moment_mass,
            moment_surface,
        }
    }
}
#[doc(hidden)]
impl IMass for FakeMass {
    fn sum(&self) -> f64 {
        self.sum
    }
    fn values(&self) -> Vec<f64> {
        self.values.clone()
    }
    fn shift(&self) -> Position {
        self.shift.clone()
    }
    fn delta_m_h(&self) -> DeltaMH {
        self.delta_m_h.clone()
    }
    fn moment_mass(&self) -> Moment {
        self.moment_mass.clone()
    }
    fn moment_surface(&self) -> SurfaceMoment {
        self.moment_surface.clone()
    }    
}
