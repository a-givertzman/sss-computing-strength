//! Распределение объема вытесненной воды по шпациям
use super::displacement::Displacement;
use crate::{draught::IDraught, math::Bounds, Error};
use std::rc::Rc;

///
/// Распределение объема вытесненной воды по шпациям
pub struct Volume {
    /// вектор разбиения на отрезки для эпюров
    bounds: Rc<Bounds>,
    /// водоизмещение судна
    displacement: Rc<Displacement>,
    /// Осадка судна
    draught: Rc<dyn IDraught>,
}
//
impl Volume {
    /// Основной конструктор. Аргументы:  
    /// * bounds - Вектор разбиения на отрезки для эпюров
    /// * displacement - Водоизмещение судна, м^3
    /// * draught - Осадка судна
    pub fn new(
        displacement: Rc<Displacement>,
        draught: Rc<dyn IDraught>,
        bounds: Rc<Bounds>,
    ) -> Self {
        Self {
            bounds,
            draught,
            displacement,
        }
    }
}
//
impl IVolume for Volume {
    /// Распределение объема вытесненной воды по шпациям
    fn values(&self) -> Result<Vec<f64>, Error> {
        let mut result: Vec<f64> = Vec::new();
        for v in self.bounds.iter() {
            if !v.is_value() {
                let error = Error::FromString("Volume value error: bound is no value".to_owned());
                log::error!("{error}");
                return Err(error);
            }
            result.push(self.displacement.value(
                v,
                self.draught.value(v.start().unwrap())?,
                self.draught.value(v.end().unwrap())?,
            )?)
        }
        log::debug!(
            "\t Volume result:{:?}, res_sum:{}",
            result,
            result.iter().sum::<f64>()
        );
        Ok(result)
    }
}

#[doc(hidden)]
pub trait IVolume {
    fn values(&self) -> Result<Vec<f64>, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeVolume {
    data: Vec<f64>,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeVolume {
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }
}
#[doc(hidden)]
impl IVolume for FakeVolume {
    fn values(&self) -> Result<Vec<f64>, Error> {
        Ok(self.data.clone())
    }
}
