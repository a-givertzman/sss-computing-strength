//! Район плавания судна
use serde::{Deserialize, Serialize};
use crate::Error;
/// Район плавания судна
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize,)]
pub enum NavigationArea {
    /// Неограниченный
    #[serde(alias="Unlimited")]
    Unlimited,
    /// Ограниченный R1
    #[serde(alias="R1")]
    R1,
    /// Ограниченный R2
    #[serde(alias="R2")]
    R2,
    /// Ограниченный R2-RSN
    #[serde(alias="R2-RSN")]
    R2Rsn,
    /// Ограниченный R2-RSN(4,5)
    #[serde(alias="R2-RSN(4,5)")]
    R2Rsn45,
    /// Ограниченный R3-RSN
    #[serde(alias="R3-RSN")]
    R3Rsn,
}
//
impl NavigationArea {
    //
    pub fn from_str(src: &str) -> Result<Self, Error> {
        Ok(match src.trim().to_uppercase().as_str() {
            "UNLIMITED" => NavigationArea::Unlimited,
            "R3-RSN" => NavigationArea::R3Rsn,
            "R2-RSN(4,5)" => NavigationArea::R2Rsn45,
            "R2-RSN" => NavigationArea::R2Rsn,
            "R2" => NavigationArea::R2,
            "R1" => NavigationArea::R1,
            src => return Err(Error::FromString(format!("NavigationArea from_str error: no type {src}"))),
        })
    }
}
//
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