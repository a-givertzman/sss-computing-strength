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
        if values.len() < 2 {
            return Err(Error::FromString(format!("Bounds::new error: values.len() < 2 ")));
        }
        Ok(Self { values })
    }
    /// Вспомогательный конструктор
    pub fn from_n(ship_length: f64, n: usize) -> Result<Self, Error> {
        if ship_length <= 0. {
            return Err(Error::FromString(format!("Bounds from_n error: ship_length {ship_length} <= 0.")));
        }
        if n <= 1 {
            return Err(Error::FromString(format!("Bounds from_n error: n {n} <= 1")));
        }
        let delta = ship_length / n as f64;
        let start = -ship_length / 2.;
        // вектор разбиения судна на отрезки
        let mut values = Vec::new();
        for i in 0..n {
            let i = i as f64;
            values.push( Bound::new(start + delta * i, start + delta * (i + 1.))?);
        }
        Self::new(
            values,
        )
    }
    // Вспомогательный конструктор
    pub fn from_frames(frames: &Vec<(f64, f64)>) -> Result<Self, Error> {
        if frames.len() <= 1 {
            return Err(Error::FromString(format!("Bounds from_frames error: frames.len() <= 1")));
        }
        let mut values = Vec::new();
        for i in 0..frames.len() {
            values.push(Bound::new(frames[i].0, frames[i].1)?);
        }
        //      log::info!("\t Bounds from_frames: frames:{:?} values:{:?} ", frames, res);
        Self::new(values)
    }
    /// Итератор по коллекции
    pub fn iter(&self) -> std::slice::Iter<'_, Bound> {
        self.values.iter()
    }
    /// Длинна диапазона
    pub fn length(&self) -> f64 {
        self.values
            .last()
            .expect("Bounds length error: no values!")
            .end()
            - self.values.first().expect("No values!").start()
    }
    /// Длинна элемента разбиения
    pub fn delta(&self) -> f64 {
        self.values
            .first()
            .expect("Bounds delta error: no values!")
            .length()
    }
}
