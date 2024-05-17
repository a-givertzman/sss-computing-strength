//! Район плавания судна
use serde::{Deserialize, Serialize};

/// Район плавания судна
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize,)]
pub enum NavigationArea {
    /// Неограниченный
    Unlimited,
    /// Ограниченный R1
    R1,
    /// Ограниченный R2
    R2,
    /// Ограниченный R2-RSN
    R2Rsn,
    /// Ограниченный R2-RSN(4,5)
    R2Rsn45,
    /// Ограниченный R3-RSN
    R3Rsn,
}
///
impl NavigationArea {
    /// Конструктор
    /// * area_type - Район плавания судна
    pub fn new(area_type: &str) -> Self {
        match area_type.trim().to_uppercase().as_str() {
            "R2-RSN(4,5)" => NavigationArea::R2Rsn45,
            "R2-RSN" => NavigationArea::R2Rsn,
            "R3-RSN" => NavigationArea::R3Rsn,
            "R1" => NavigationArea::R1,
            "R2" => NavigationArea::R2,
            _ => NavigationArea::Unlimited,
        }
    }
}
///
impl std::fmt::Display for NavigationArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            NavigationArea::R2Rsn45 => "R2-RSN(4,5)",
            NavigationArea::R2Rsn => "R2-RSN",
            NavigationArea::R3Rsn => "R3-RSN",
            NavigationArea::R1 => "R1",
            NavigationArea::R2 => "R2",
            _ => "Unlimited",
        };
        write!(f, "NavigationArea({text})",)
    }
}
