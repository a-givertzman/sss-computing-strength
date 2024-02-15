///диапазон значений
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bound {
    start: f64,
    end: f64,
}

impl Bound {
    ///
    pub fn new(x_start: f64, x_end: f64) -> Self {
        assert!(x_end > x_start);
        Self {
            start: x_start,
            end: x_end,
        }
    }
    ///какая часть попадает в границы переданного диапазона, 0..=1
    pub fn part(&self, bound: &Bound) -> f64 {
        self.intersect(bound).map(|v| v.length()/self.length() ).unwrap_or(0.)
    }
    ///пересечение диапазонов, 0..=self.length
    pub fn intersect(&self, bound: &Bound) -> Option<Bound> {
        assert!(bound.end() > bound.start());
        if bound.start() >= self.end {
            return None;
        }
        if bound.end() <= self.start {
            return None;
        }
        if bound.start() <= self.start && bound.end() >= self.end {
            return Some(*self);
        }

        Some(Bound::new(bound.start().max(self.start), bound.end().min(self.end)))
    }
    ///
    pub fn length(&self) -> f64 {
        self.end - self.start
    }
    ///
    pub fn start(&self) -> f64 {
        self.start
    }
    ///
    pub fn end(&self) -> f64 {
        self.end
    }
    ///
    pub fn center(&self) -> f64 {
        (self.start + self.end)/2.
    }
    ///
    pub fn add(&mut self, other: f64) {
        self.start += other;
        self.end += other;
    }
}