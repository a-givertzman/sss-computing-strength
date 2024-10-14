//! Непрерывный набор диапазонов значений
use crate::Error;

use super::Bound;

/// Непрерывный набор диапазонов значений
#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    // Непрерывный вектор диапазонов
    values: Vec<Bound>,
}
///
impl Bounds {
    /// Основной конструктор
    pub fn new(values: Vec<Bound>) -> Result<Self, Error> {
        for v in &values {
            match v {
                Bound::None => return Err(Error::FromString("Bounds new error: Bound::None in values".to_owned())),
                Bound::Full => return Err(Error::FromString("Bounds new error: Bound::Full in values".to_owned())),
                Bound::Value(_, _) => continue,
            }
        }
        if values.len() < 2 {
            return Err(Error::FromString(
                "Bounds::new error: values.len() < 2 ".to_string(),
            ));
        }
        Ok(Self { values })
    }
    /// Вспомогательный конструктор
    /// * loa - L.O.A
    /// * middle_x - X midship from Fr0
    /// * n - Number of Parts
    #[allow(unused)]
    pub fn from_n(loa: f64, middle_x: f64, n: usize) -> Result<Self, Error> {
        if loa <= 0. {
            return Err(Error::FromString(format!(
                "Bounds from_n error: loa {loa} <= 0."
            )));
        }
        if n <= 1 {
            return Err(Error::FromString(format!(
                "Bounds from_n error: n {n} <= 1"
            )));
        }
        let n_parts = n as f64;
        let mut values = Vec::new();
        for i in 0..n {
            let i = i as f64;
            values.push(Bound::new(loa*i/n_parts - middle_x, loa*(i+1.)/n_parts - middle_x)?);
        }
        Self::new(values)
    }
    // Вспомогательный конструктор
    pub fn from_frames(frames: &Vec<(f64, f64)>) -> Result<Self, Error> {
        if frames.len() <= 1 {
            return Err(Error::FromString(
                "Bounds from_frames error: frames.len() <= 1".to_string(),
            ));
        }
        let mut values = Vec::new();
        for i in 0..frames.len() {
            values.push(Bound::new(frames[i].0, frames[i].1)?);
        }
        log::trace!("\t Bounds from_frames: frames:{:?} values:{:?} ", frames, values);
        Self::new(values)
    }
    /// Итератор по коллекции
    pub fn iter(&self) -> std::slice::Iter<'_, Bound> {
        self.values.iter()
    }
    /// Длинна диапазона
    #[allow(unused)]
    pub fn length(&self) -> f64 {
        self.values
            .last()
            .expect("Bounds length error: no values!")
            .end()
            .expect("Bounds delta error: no end value for last element!")
            - self
                .values
                .first()
                .expect("No values!")
                .start()
                .expect("Bounds delta error: no start value for first element!")
    }
    /// Длинна элемента разбиения
    pub fn delta(&self) -> f64 {
        self.values
            .first()
            .expect("Bounds delta error: no values!")
            .length()
            .expect("Bounds delta error: no length for first element!")
    }
}
