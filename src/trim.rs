//! Интерфейс для расчета дифферента

pub trait ITrim {
    /// Вычисление дифферента
    fn value(&mut self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeTrim {
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeTrim {
    pub fn new(value: f64,) -> Self {
        Self { value }
    }
}
#[doc(hidden)]
impl ITrim for FakeTrim {
    fn value(&mut self) -> f64 {
        self.value
    }
}

