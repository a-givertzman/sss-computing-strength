//! Нагрузка на корпус судна
use std::rc::Rc;

use crate::{load::ILoad, math::*};

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
}

impl Mass {
    /// Аргументы конструктора:  
    /// * loads_const - постоянная масса судна распределенная по шпациям
    /// * shift_const - смещение постоянный массы судна
    /// * loads_stock - масса запасов судна распределенная по шпациям
    /// * shift_stock - смещение массы запасов судна
    /// * loads_cargo - грузы судна
    /// * bounds - вектор разбиения на отрезки для эпюров
    pub fn new (
        loads_const: Vec<Rc<Box<dyn ILoad>>>,
        shift_const: Position,
        loads_cargo: Vec<Rc<Box<dyn ILoad>>>, bounds: Rc<Bounds>) -> Self {
        Self { loads_const, shift_const, loads_cargo, bounds, }
    }
    /// Суммарный статический момент. Для постоянной массы и для запасов считается по 
    /// заданным значениям смещения центра масс
    fn moment_mass (&self) -> MassMoment {
        let res = self.loads_const.iter().map(|c: &Rc<Box<dyn ILoad>>| MassMoment::from_pos(self.shift_const, c.mass(None)) ).sum::<MassMoment>() +
    //    let res = self.loads_const.iter().map(|c: &Rc<Box<dyn ILoad>>| c.moment_mass() ).sum::<MassMoment>() +
        self.loads_cargo.iter().map(|c| c.moment_mass() ).sum::<MassMoment>();
        log::info!("\t Mass moment_mass:{res} ");
        res
    }
    /// Суммарный момент свободной поверхности
    fn moment_surface (&self) -> SurfaceMoment {
        self.loads_cargo.iter().map(|c| c.moment_surface() ).sum::<SurfaceMoment>()
    }
}

impl IMass for Mass {
    /// Суммарная масса
    fn sum(&self) -> f64 {
        let res = self.loads_const.iter().map(|v| v.mass(None)).sum::<f64>() +
        self.loads_cargo.iter().map(|v| v.mass(None)).sum::<f64>();
        log::info!("\t Mass sum:{res} ");
        res
    }    
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64> {
        self.bounds.iter().map(|b| 
            self.loads_const.iter().map(|v| v.mass(Some(*b))).sum::<f64>() + 
            self.loads_cargo.iter().map(|v| v.mass(Some(*b))).sum::<f64>()).collect()
    }
    /// Отстояние центра масс
    fn shift(&self) -> Position {
        let res = self.moment_mass().to_pos(self.sum());
        log::info!("\t Mass shift:{res} ");
        res
    }
    /// Поправка к продольной метацентрической высоте на влияние  
    /// свободной поверхности жидкости в цистернах (2)
    fn delta_m_h(&self) -> DeltaMH {
        assert!(self.sum() > 0., "Mass delta_m_h sum > 0");
        let res = DeltaMH::from_moment(self.moment_surface(), self.sum());
        log::info!("\t Mass delta_m_h:({}, {})", res.long(), res.cross());
        res
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
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeMass {
    sum: f64,
    values: Vec<f64>,
    shift: Position,
    delta_m_h: DeltaMH,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeMass {
    pub fn new( 
        sum: f64,
        values: Vec<f64>,
        shift: Position,
        delta_m_h: DeltaMH,
    ) -> Self {
        Self { sum, values, shift, delta_m_h, }
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
}