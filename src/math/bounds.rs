//! Непрерывный набор диапазонов значений
use super::Bound;

/// Непрерывный набор диапазонов значений
#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    // Непрерывный вектор диапазонов
    values: Vec<Bound>, 
    /// Длинна элемента разбиения  
    delta: f64,
}
///
impl Bounds {
    /// Основной конструктор 
    pub fn new(values: Vec<Bound>, delta: f64) -> Self {
        assert!(!values.is_empty(), "data.is_empty()");
        Self { values, delta }
    }
    /// Вспомогательный конструктор 
    pub fn from_n(ship_length: f64, n: usize) -> Self {
        assert!(ship_length > 0., "ship_length {} > 0.", ship_length);
        assert!(n > 1, "n {} > 0.", n);
        let delta = ship_length / n as f64;
        let start = -ship_length / 2.;
        // вектор разбиения судна на отрезки
        let res = Self::new((0..n )
            .map(|v| {
                Bound::new(
                    start + delta * v as f64,
                    start + delta * (v as f64 + 1.),
                )
            })
            .collect::<Vec<_>>(), delta);
 //       log::info!("\t Bounds from_n: ship_length:{ship_length} n:{n} values:{:?} ", res.values);
        res
    }
    /// Итератор по коллекции
    pub fn iter(&self) -> std::slice::Iter<'_, Bound> {
        self.values.iter()
    }
    /// Длинна диапазона
    pub fn length(&self) -> f64 {
        self.values.last().expect("No values!").end() - self.values.first().expect("No values!").start()
    }
    /// Длинна элемента разбиения
    pub fn delta(&self) -> f64 {
        self.delta
    }
}
