use std::collections::HashMap;

use crate::data::structs::{DataArray, Pair, PointData, Quadruple, Triple};

///
impl From<Vec<(f64, f64)>> for DataArray<Pair> {
    fn from(src: Vec<(f64, f64)>) -> Self {
        Self{data: src.into_iter().map(|(key, value)| Pair{key, value} ).collect(), error: HashMap::new() }
    }
}
///
impl From<Vec<(f64, f64, f64)>> for DataArray<Triple> {
    fn from(src: Vec<(f64, f64, f64)>) -> Self {
        Self{data: src.into_iter().map(|(key, value_x, value_y)| Triple{key, value_x, value_y} ).collect(), error: HashMap::new() }
    }
}
///
impl From<Vec<(f64, f64, f64, f64)>> for DataArray<Quadruple> {
    fn from(src: Vec<(f64, f64, f64, f64)>) -> Self {
        Self{data: src.into_iter().map(|(key, value_x, value_y, value_z)| Quadruple{key, value_x, value_y, value_z} ).collect(), error: HashMap::new() }
    }
}
///
impl From<Vec<(&str, f64, f64, f64)>> for DataArray<PointData> {
    fn from(src: Vec<(&str, f64, f64, f64)>) -> Self {
        Self{data: src.into_iter().map(|(name, x, y, z)| PointData{name: name.to_owned(), x, y, z} ).collect(), error: HashMap::new() }
    }
}