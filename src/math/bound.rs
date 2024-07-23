//! Диапазон значений

use crate::Error;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bound {
    /// начало диапазона
    start: f64,
    /// конец диапазона
    end: f64,
}
///
impl Bound {
    /// Основной конструктор  
    /// * start - начало диапазона
    /// * end - конец диапазона
    pub fn new(start: f64, end: f64) -> Result<Self, Error> {
        if end <= start {
            return Err(Error::FromString(format!("Bound new error: end <= start")));
        }
        Ok(Self { start, end })
    }
    /// Дополнительный конструктор  
    /// * (f64, f64) - (начало диапазона, конец диапазона)
    pub fn from(v: (f64, f64)) -> Result<Self, Error> {
        Self::new(v.0, v.1)
    }
    ///
    /// Отношение общей части пересечения к длине диапазона
    pub fn part_ratio(&self, bound: &Bound) -> Result<f64, Error> {
        Ok(self.intersect(bound)?
            .map(|v| v.length() / self.length())
            .unwrap_or(0.))
    }
    ///
    /// Пересечение c другим диапазоном, возвращает общий диапазон
    pub fn intersect(&self, other: &Bound) -> Result<Option<Bound>, Error> {
        if other.start() >= self.end {
            return Ok(None);
        }
        if other.end() <= self.start {
            return Ok(None);
        }
        if other.start() <= self.start && other.end() >= self.end {
            return Ok(Some(*self));
        }
        Ok(Some(Bound::new(
            other.start().max(self.start),
            other.end().min(self.end),
        )?))
    }
    ///
    /// Длинна диапазона
    pub fn length(&self) -> f64 {
        self.end - self.start
    }
    ///
    /// Начало диапазона
    pub fn start(&self) -> f64 {
        self.start
    }
    ///
    /// Конец диапазона
    pub fn end(&self) -> f64 {
        self.end
    }
    ///
    /// Центр диапазона
    pub fn center(&self) -> f64 {
        (self.start + self.end) / 2.
    }
}
///
impl std::fmt::Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bound({}, {})", self.start, self.end)
    }
}
