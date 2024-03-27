//! Коэффициент k для судов, имеющих скуловые кили или 
/// брусковый киль. Табл. 2.1.5.2
use serde::{Deserialize, Serialize};

/// Промежуточные структуры для serde_json для парсинга
/// Коэффициент k для судов, имеющих скуловые кили или 
/// брусковый киль. Табл. 2.1.5.2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoefficientKData {
    /// Отношение A_l/L_wlB, A_k - суммарная площадь килей
    pub a_div_l: f64,
    /// Коэффициент k для судов, имеющих скуловые кили или брусковый киль
    pub k: f64,
}
///
impl std::fmt::Display for CoefficientKData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CoefficientKData(A_l/L_wlB:{}, k:{} )",
            self.a_div_l, self.k,
        )
    }
}