//! Структуры для вывода данных
use serde::{Deserialize, Serialize};

/// Выходная структура данных
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutData {
    /// эпюр срезающих сил (координата по х, значение)
    pub shear_force: Vec<(f64, f64)>,
    /// эпюр изгибающего момента (координата по х, значение)
    pub bending_moment: Vec<(f64, f64)>,
}


impl OutData {
    ///
    #[allow(dead_code)]
    pub fn serialize(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }
}


