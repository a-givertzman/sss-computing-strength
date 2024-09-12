//! Критерии проверки остойчивости
pub(crate) mod stability;
pub(crate) mod draught;
pub(crate) mod computer;

pub use stability::*;
pub use draught::*;
pub use computer::*;

#[derive(Hash, Eq, PartialEq)]
pub enum CriterionID {
    Wheather = 1,
    WindStaticHeel = 2,
    AreaLC0_30 = 3,
    AreaLc0Thetalmax = 4,    
    AreaLC0_40 = 5,
    AreaLC30_40 = 6,
    MaximumLC = 7,
    MaximumLcTimber = 8,
    MaximumLcIcing = 9,
    HeelMaximumLC = 10,
    HeelFirstMaximumLC = 11,
    MinMetacentricHight = 12,
    Acceleration = 13,
    HeelTurning = 14,
    HeelGrainDisplacement = 15,
    AreaLcGrainDisplacement = 16,
    MinMetacentricHeightSubdivIndex = 17,
    LoadLineDraftSB = 101,
    LoadLineDraftPS = 102,
    MaximumForwardTrim = 103,
    MaximumAftTrim = 104,
    DepthAtForwardPerpendicularSB = 105,
    DepthAtForwardPerpendicularPS = 106,
    ScrewImmersionCL = 107,
    ScrewImmersionPS = 108,
    ScrewImmersionSB = 109,
    ReserveBuoyncyInBow = 112,
    MinimumMiddleDraftCL = 113,
    MinimumForwardDraftCL = 114,
}
/// Результат проверки критерия
pub struct CriterionData {
    /// id критерия
    pub criterion_id: usize,
    /// Результат расчета
    pub result: f64,
    /// Пороговое значение критерия
    pub target: f64,
    /// Текст ошибки
    pub error_message: Option<String>,
}
///
impl CriterionData {
    /// Конструктор при наличии результата
    pub fn new_result(criterion_id: CriterionID, result: f64, target: f64) -> Self {
        Self {
            criterion_id: criterion_id as usize,
            result,
            target,
            error_message: None,
        }
    }
    /// Конструктор при ошибке расчета
    pub fn new_error(criterion_id: CriterionID, error_message: String) -> Self {
        Self {
            criterion_id: criterion_id as usize,
            result: 0.,
            target: 0.,
            error_message: Some(error_message),
        }
    }
}
