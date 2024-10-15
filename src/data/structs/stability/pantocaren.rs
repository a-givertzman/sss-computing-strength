//! Промежуточные структуры для serde_json для парсинга пантокаренов  
use super::DataArray;
use super::DataArray;
use serde::{Deserialize, Serialize};
/// Промежуточные структуры для serde_json для парсинга данных  
/// плечей устойчивости формы (пантокарены)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PantocarenData {
    /// Осадка при плотности воды 1.
    pub draught: f64,
    /// Крен, градус
    pub roll: f64,
    /// Плечо устойчивости, м
    pub moment: f64,
}
//
impl std::fmt::Display for PantocarenData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PantocarenData(draught:{}, roll:{}, moment:{} )",
            self.draught, self.roll, self.moment,
        )
    }
}
//
pub type PantocarenDataArray = DataArray<PantocarenData>;
//
impl PantocarenDataArray {
    /// Преобразовает и возвращает данные
    pub fn data(mut self) -> Vec<(f64, Vec<(f64, f64)>)> {
        let mut vec: Vec<(f64, Vec<(f64, f64)>)> = Vec::new();
        self.data.sort_by(|a, b| {
            a.draught
                .partial_cmp(&b.draught)
                .expect("PantocarenDataArray data sort error!")
        });
        self.data.into_iter().for_each(|v| {
            if vec.last().is_none() || vec.last().unwrap().0 != v.draught {
                vec.push((v.draught, vec![(v.roll, v.moment)]));
            } else {
                vec.last_mut().unwrap().1.push((v.roll, v.moment));
            }
        });
        vec
    }
}
