//! Набор результатов расчетов для записи в БД

use std::{cell::RefCell, collections::HashMap};

/// Набор результатов расчетов для записи в БД
pub struct Results {
    data: RefCell<HashMap<String, Vec<f64>>>,
} 
//
impl Results {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }
}
//
impl IResults for Results {
    /// Добавление пары имя/значение
    fn add(&self, name: String, values: Vec<f64>) {
        self.data.borrow_mut().insert(name, values);
    }
    /// Геттер для данных
    fn take_data(&self) -> Vec<(String, Vec<f64>)> {
        self.data.take().into_iter().collect()
    }
}
#[doc(hidden)]
pub trait IResults {
    /// Добавление пары имя/значение
    fn add(&self, name: String, values: Vec<f64>);
    /// Геттер для данных
    fn take_data(&self) -> Vec<(String, Vec<f64>)>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeResults;
#[doc(hidden)]
#[allow(dead_code)]
impl IResults for FakeResults {
    fn add(&self, _: String, _: Vec<f64>) { }
    fn take_data(&self) -> Vec<(String, Vec<f64>)> {
        Vec::new()
    }
}
