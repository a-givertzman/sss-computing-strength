//! Диапазон значений

use crate::Error;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bound {
    None,
    Full,
    /// диапазон: (начало диапазона, конец диапазона)
    Value(f64, f64),
}
//
impl Bound {
    /// Основной конструктор  
    /// * start - начало диапазона
    /// * end - конец диапазона
    pub fn new(start: f64, end: f64) -> Result<Self, Error> {
        if end <= start {
            return Err(Error::FromString(
                "Bound new error: end <= start".to_string(),
            ));
        }
        Ok(Self::Value(start, end))
    }
    /// Дополнительный конструктор  
    /// * (f64, f64) - (начало диапазона, конец диапазона)
    #[allow(unused)]
    pub fn from(v: (f64, f64)) -> Result<Self, Error> {
        Self::new(v.0, v.1)
    }
    /// Отношение общей части пересечения к длине диапазона
    pub fn part_ratio(&self, other: &Bound) -> Result<f64, Error> {
        Ok(match self.intersect(other)? {
            Bound::None => 0.,
            Bound::Full => 1.,
            r @ Bound::Value(_, _) => {
                r.length().expect("Bound part_ratio error")
                    / self.length().expect("Bound part_ratio error")
            }
        })
    }
    /// Пересечение c другим диапазоном, возвращает общий диапазон
    pub fn intersect(&self, other: &Bound) -> Result<Bound, Error> {
        Ok(match self {
            Bound::None => Bound::None,
            Bound::Full => *other,
            Bound::Value(self_start, self_end) => match other {
                Bound::None => Bound::None,
                Bound::Full => *self,
                Bound::Value(other_start, other_end) => {
                    if other_start >= self_end {
                        return Ok(Bound::None);
                    }
                    if other_end <= self_start {
                        return Ok(Bound::None);
                    }
                    if other_start <= self_start && other_end >= self_end {
                        return Ok(*self);
                    }
                    Bound::new(other_start.max(*self_start), other_end.min(*self_end))?
                }
            },
        })
    }
    /// Длинна диапазона
    pub fn length(&self) -> Option<f64> {
        match self {
            Bound::None => Some(0.),
            Bound::Full => None,
            Bound::Value(start, end) => Some(end - start),
        }
    }
    /// Начало диапазона
    pub fn start(&self) -> Option<f64> {
        match self {
            Bound::Value(start, _) => Some(*start),
            _ => None,
        }
    }
    /// Конец диапазона
    pub fn end(&self) -> Option<f64> {
        match self {
            Bound::Value(_, end) => Some(*end),
            _ => None,
        }
    }
    /// Центр диапазона
    pub fn center(&self) -> Option<f64> {
        match self {
            Bound::Value(start, end) => Some((start + end) / 2.),
            _ => None,
        }
    }
    /// Если true, то диапазон с нулевой длинной
    pub fn is_none(&self) -> bool {
        *self == Bound::None
    }
    /// Если true, то диапазон с ненулевой длинной
    #[allow(unused)]
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
    /// Если true, то ограниченный диапазон с ненулевой длинной 
    pub fn is_value(&self) -> bool {
        *self != Bound::None && *self != Bound::Full
    }
}
//
impl std::fmt::Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bound::None => write!(f, "Bound::None"),
            Bound::Full => write!(f, "Bound::Full"),
            Bound::Value(start, end) => write!(f, "Bound::Value({}, {})", start, end),
        }
    }
}
