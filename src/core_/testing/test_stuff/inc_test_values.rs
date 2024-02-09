#![allow(non_snake_case)]

use super::test_value::Value;

///
/// 
#[derive(Debug, Clone)]
pub struct IncTestValues {
    // id: String,
    // initial: i64,
    iterations: usize,
    value: i64,
}
///
/// 
impl IncTestValues {
    ///
    /// 
    pub fn  new(parent: impl Into<String>, initial: i64, iterations: usize) -> Self {
        Self {
            // id: format!("{}/RandomTestPoints", parent.into()),
            // initial,
            iterations,
            value: initial,
        }
    }
    }
///
/// 
impl Iterator for IncTestValues {
    type Item = Value;
    //
    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations > 0 {
            self.iterations -= 1;
            let value = self.value;
            self.value = value + 1;
            Some(Value::Int(value))
        } else {
            None
        }
    }
}