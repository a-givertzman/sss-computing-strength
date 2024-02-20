///
/// Диапазон значений
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bound {
    start: f64,
    end: f64,
}
///
/// 
impl Bound {
    ///
    /// Creates new instance of Bound
    /// - start - начало диапазона
    /// - end - конец диапазона
    pub fn new(start: f64, end: f64) -> Self {
        assert!(end > start);
        Self {
            start: start,
            end: end,
        }
    }
    ///
    /// Отношение общей части пересечения к длине диапазона self
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