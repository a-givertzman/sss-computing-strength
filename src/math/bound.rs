//! Диапазон значений
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bound {
    /// начало диапазона
    start: f64,
    /// конец диапазона
    end: f64,   
}
/// 
impl Bound {
    ///
    /// Конструктор  
    /// - start - начало диапазона
    /// - end - конец диапазона
    pub fn new(start: f64, end: f64) -> Self {
        assert!(end > start);
        Self {
            start,
            end,
        }
    }
    ///
    /// Отношение общей части пересечения к длине диапазона
    pub fn part_ratio(&self, bound: &Bound) -> f64 {
        self.intersect(bound).map(|v| v.length()/self.length() ).unwrap_or(0.)
    }
    ///
    /// Пересечение c другим диапазоном, возвращает общий диапазон
    pub fn intersect(&self, other: &Bound) -> Option<Bound> {
        if other.start() >= self.end {
            return None;
        }
        if other.end() <= self.start {
            return None;
        }
        if other.start() <= self.start && other.end() >= self.end {
            return Some(*self);
        }
        Some(Bound::new(other.start().max(self.start), other.end().min(self.end)))
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
        (self.start + self.end)/2.
    }
}

/// Набор диапазонов значений
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    // непрерывный вектор диапазонов
    data: Vec<f64>, 
}

impl Bounds {
    /// Основной конструктор 
    pub fn new(data: Vec<f64>) -> Self {
        assert!(!data.is_empty(), "data.is_empty()");
        Self { data }
    }
    /// Вспомогательный конструктор 
    pub fn from_n(ship_length: f64, n: usize) -> Self {
        assert!(!data.is_empty(), "data.is_empty()");
        Self { data }
    }
}
    // длинна судна
    let ship_length = data.ship_length;
    let n = data.n_parts as usize;
    let delta_x = ship_length / n as f64;
    let start_x = -ship_length / 2.;
    // вектор разбиения судна на отрезки
    let bounds = (0..n as usize)
        .map(|v| {
            Bound::new(
                start_x + delta_x * v as f64,
                start_x + delta_x * (v as f64 + 1.),
            )
        })
        .collect::<Vec<_>>();