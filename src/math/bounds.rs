use super::Bound;


/// Набор диапазонов значений
#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    // непрерывный вектор диапазонов
    values: Vec<Bound>, 
}

impl Bounds {
    /// Основной конструктор 
    pub fn new(values: Vec<Bound>) -> Self {
        assert!(!values.is_empty(), "data.is_empty()");
        Self { values }
    }
    /// Вспомогательный конструктор 
    pub fn from_n(ship_length: f64, n: usize) -> Self {
        assert!(ship_length > 0., "ship_length {} > 0.", ship_length);
        assert!(n > 1, "n {} > 0.", n);
        let delta_x = ship_length / n as f64;
        let start_x = -ship_length / 2.;
        // вектор разбиения судна на отрезки
        let res = Self::new((0..n )
            .map(|v| {
                Bound::new(
                    start_x + delta_x * v as f64,
                    start_x + delta_x * (v as f64 + 1.),
                )
            })
            .collect::<Vec<_>>() );
        log::info!("\t Bounds from_n: ship_length:{ship_length} n:{n} values:{:?} ", res.values);
        res
    }
    /// Итератор по коллекции
    pub fn iter(&self) -> std::slice::Iter<'_, Bound> {
        self.values.iter()
    }
    /// Количество элементов разбиения
    pub fn qnt(&self) -> usize {
        self.values.len()
    }
    /// Длинна диапазона
    pub fn length(&self) -> f64 {
        self.values.last().expect("No values!").end() - self.values.first().expect("No values!").start()
    }

}
