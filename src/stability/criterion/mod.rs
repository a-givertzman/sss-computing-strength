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
    LlDraftSSB = 101,
    LlDraftSPS = 102,
    LlDraftWSB = 103,
    LlDraftWPS = 104,
    LlDraftWNASB = 105,
    LlDraftWNAPS = 106,
    LlDraftTSB = 107,
    LlDraftTPS = 108,
    LlDraftFSB = 109,
    LlDraftFPS= 110,
    LlDraftTFSB = 111,
    LlDraftTFPS = 112,
    LlDraftLSSB = 113,
    LlDraftLSPS = 114,
    LlDraftLWSB = 115,
    LlDraftLWPS = 116,
    LlDraftLWNASB = 117,
    LlDraftLWNAPS = 118,
    LlDraftLTSB = 119,
    LlDraftLTPS = 120,
    LlDraftLFSB = 121,
    LlDraftLFPS = 122,
    LlDraftLTFSB = 123,
    LlDraftLTFPS = 124,
    LlDraftSI1Reserve = 125,
    LlDraftSI16Reserve = 140,
    MaximumForwardTrim = 141,
    MaximumAftTrim = 142,
    DepthAtForwardPerpendicularSB = 143,
    DepthAtForwardPerpendicularPS = 144,
    ScrewImmersionCL = 145,
    ScrewImmersionSB = 146,
    ScrewImmersionPS = 147,
    ScrewImmersionReserve = 148,
  //  ScrewImmersionRreserve = 149,
    ReserveBuoyncyInBow = 150,
}
//
impl From<String> for CriterionID {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Осадка по Л ГВЛ ПрБ" | "LL draft S SB" => CriterionID::LlDraftSSB,
            "Осадка по Л ГВЛ ЛБ" | "LL draft S PS" => CriterionID::LlDraftSPS,
            "Осадка по З ГВЛ ПрБ" | "LL draft W SB" => CriterionID::LlDraftWSB,
            "Осадка по З ГВЛ ЛБ" | "LL draft W PS" => CriterionID::LlDraftWPS,
            "Осадка по ЗСА ГВЛ ПрБ" | "LL draft WNA SB" => CriterionID::LlDraftWNASB,
            "Осадка по ЗСА ГВЛ ЛБ" | "LL draft WNA PS" => CriterionID::LlDraftWNAPS,
            "Осадка по Т ГВЛ ПрБ" | "LL draft T SB" => CriterionID::LlDraftTSB,
            "Осадка по Т ГВЛ ЛБ" | "LL draft T PS" => CriterionID::LlDraftTPS,
            "Осадка по П ГВЛ ПрБ" | "LL draft F SB" => CriterionID::LlDraftFSB,
            "Осадка по П ГВЛ ЛБ" | "LL draft F PS" => CriterionID::LlDraftFPS,
            "Осадка по ТП ГВЛ ПрБ" | "LL draft TF SB" => CriterionID::LlDraftTFSB,
            "Осадка по ТП ГВЛ ЛБ" | "LL draft TF PS" => CriterionID::LlDraftTFPS,
            "Осадка по ЛЛ ГВЛ ПрБ" | "LL draft LS SB" => CriterionID::LlDraftLSSB,
            "Осадка по ЛЛ ГВЛ ЛБ" | "LL draft LS PS" => CriterionID::LlDraftLSPS,
            "Осадка по ЛЗ ГВЛ ПрБ" | "LL draft LW SB" => CriterionID::LlDraftLWSB,
            "Осадка по ЛЗ ГВЛ ЛБ" | "LL draft LW PS" => CriterionID::LlDraftLWPS,
            "Осадка по ЛЗСА ГВЛ ПрБ" | "LL draft LWNA SB" => CriterionID::LlDraftLWNASB,
            "Осадка по ЛЗСА ГВЛ ЛБ" | "LL draft LWNA PS" => CriterionID::LlDraftLWNAPS,
            "Осадка по ЛТ ГВЛ ПрБ" | "LL draft LT SB" => CriterionID::LlDraftLTSB,
            "Осадка по ЛТ ГВЛ ЛБ" | "LL draft LT PS" => CriterionID::LlDraftLTPS,
            "Осадка по ЛП ГВЛ ПрБ" | "LL draft LF SB" => CriterionID::LlDraftLFSB,
            "Осадка по ЛП ГВЛ ЛБ" | "LL draft LF PS" => CriterionID::LlDraftLFPS,
            "Осадка по ЛТП ГВЛ ПрБ" | "LL draft LTF SB" => CriterionID::LlDraftLTFSB,
            "Осадка по ЛТП ГВЛ ЛБ" | "LL draft LTF PS" => CriterionID::LlDraftLTFPS,
            "Осадка по ГВЛ Р1 (резерв)" | "LL draft SI1 (reserve)" => CriterionID::LlDraftSI1Reserve,
            "Осадка по ГВЛ Р16 (резерв)" | "LL draft SI16 (reserve)" => CriterionID::LlDraftSI16Reserve,
            "Максимальный дифферент на нос" | "Maximum forward trim" => CriterionID::MaximumForwardTrim,
            "Максимальный дифферент на корму" | "Maximum aft trim" => CriterionID::MaximumAftTrim,
            "Высота на носовом перпендикуляре ПрБ" | "Depth at forward perpendicular SB" => CriterionID::DepthAtForwardPerpendicularSB,
            "Высота на носовом перпендикуляре ЛБ" | "Depth at forward perpendicular PS" => CriterionID::DepthAtForwardPerpendicularPS,
            "Заглубление винта ДП" | "Screw immersion CL" => CriterionID::ScrewImmersionCL,
            "Заглубление винта ПрБ" | "Screw immersion SB" => CriterionID::ScrewImmersionSB,
            "Заглубление винта ЛБ" | "Screw immersion PS" => CriterionID::ScrewImmersionPS,
            "Запас плавучести в носу" | "Reserve buoyncy in bow" => CriterionID::ReserveBuoyncyInBow,  
            _ => CriterionID::ScrewImmersionReserve,
        } 
    }
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
//
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
