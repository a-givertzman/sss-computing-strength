#![allow(non_snake_case)]

use rand::{rngs::ThreadRng, Rng};

use super::test_value::Value;

///
/// Produces random Value
/// - random item in initial vector, if it is not empty
/// - random in f64::MIN..f64::MAX, if initial vector is empty
#[derive(Debug, Clone)]
pub struct RandomTestValues {
    id: String,
    initial: Vec<Value>,
    iterations: usize,
    rnd: ThreadRng,
}
///
/// 
impl RandomTestValues {
    ///
    /// 
    pub fn  new(parent: impl Into<String>, initial: Vec<Value>, iterations: usize) -> Self {
        Self {
            id: format!("{}/RandomTestPoints", parent.into()),
            initial,
            iterations,
            rnd: rand::thread_rng(),
        }
    }
    }
///
/// 
impl Iterator for RandomTestValues {
    type Item = Value;
    //
    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations > 0 {
            self.iterations -= 1;
            if self.initial.is_empty() {
                let value = self.rnd.gen_range((f64::MIN / 2.0)..(f64::MAX / 2.0));
                return Some(Value::Float(value))
            } else {
                let index = self.rnd.gen_range(0..self.initial.len());
                match self.initial.get(index) {
                    Some(value) => {
                        return Some(value.clone());
                    },
                    None => {
                        panic!("{}.next | Out of range: index {}, not in initial length 0..{}", self.id, index, self.initial.len())
                    },
                };
            }
        }
        None
    }
}