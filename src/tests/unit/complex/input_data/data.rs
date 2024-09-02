use std::collections::HashMap;

use crate::data::structs::{DataArray, Pair};

///
impl DataArray<Pair> {
    /// Преобразование данных в массив ключ + значение
    pub fn from(src: Vec<(f64, f64)>) -> Self {
        Self{data: src.into_iter().map(|(key, value)| Pair{key, value} ).collect(), error: HashMap::new() }
    }
}