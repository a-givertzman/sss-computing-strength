//! Критерии проверки посадки судна

pub(crate) mod bow_board;
pub(crate) mod load_line;
pub(crate) mod minimum_draft;
pub(crate) mod reserve_buoyncy;
pub(crate) mod screw;

pub use bow_board::*;
pub use load_line::*;
pub use minimum_draft::*;
pub use reserve_buoyncy::*;
pub use screw::*;

use super::{CriterionData, CriterionID};
use crate::{
    data::structs::ShipType, draught::IDraught, trim::ITrim, Error, IParameters, ParameterID,
};
use std::rc::Rc;

/// Критерии проверки посадки судна
pub struct CriterionDraught {
    /// Тип судна
    ship_type: ShipType,
    /// Дедвейт
    deadweight: f64,
    /// Тип надводного борта
    freeboard_type: String,
    /// Суммарая площадь проекции носа судна на диаметральную плоскость
    bow_area_min: f64,
    /// Минимально допустимый дифферент с точки зрения расчета вероятностного индекса деления на отсеки  
    aft_trim: f64,
    /// Максимально допустимый дифферент с точки зрения расчета вероятностного индекса деления на отсеки  
    forward_trim: f64,
    /// Расчетная минимальная высота в носу
    bow_h_min: f64,
    /// Осадка судна
    draught: Rc<dyn IDraught>,
    /// Вычисление средней осадки и дифферента
    trim: Rc<dyn ITrim>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
    /// Расчет уровня заглубления для осадок судна
    load_line: LoadLine,
    /// Высота на носовом перпендикуляре
    bow_board: DepthAtForwardPerpendicular,
    /// Расчет уровня заглубления для винтов судна
    screw: Screw,
    /// Запас плавучести в носу
    reserve_buoyncy: ReserveBuoyncyInBow,
    /// Минимальная осадка
    minimum_draft: MinimumDraft,
}
///
impl CriterionDraught {
    /// Главный конструктор:
    /// * ship_type - Тип судна
    /// * deadweight -Дедвейт
    /// * freeboard_type - Тип надводного борта
    /// * bow_area_min - Суммарая площадь проекции носа судна на диаметральную плоскость
    /// * aft_trim - Минимально допустимый дифферент с точки зрения расчета вероятностного индекса деления на отсеки  
    /// * forward_trim - Максимально допустимый дифферент с точки зрения расчета вероятностного индекса деления на отсеки  
    /// * bow_h_min - Расчетная минимальная высота в носу
    /// * draught - Осадка судна
    /// * trim - Cредняя осадка и дифферент
    /// * parameters - Набор результатов расчетов для записи в БД
    /// * load_line - Расчет уровня заглубления для осадок судна
    /// * bow_board - Высота на носовом перпендикуляре
    /// * screw - Расчет уровня заглубления для винтов судна
    /// * reserve_buoyncy - Запас плавучести в носу
    /// * minimum_draft - Минимальная осадка
    pub fn new(
        ship_type: ShipType,
        deadweight: f64,
        freeboard_type: String,
        bow_area_min: f64,
        aft_trim: f64,
        forward_trim: f64,
        bow_h_min: f64,
        draught: Rc<dyn IDraught>,
        trim: Rc<dyn ITrim>,
        parameters: Rc<dyn IParameters>,
        load_line: LoadLine,
        bow_board: DepthAtForwardPerpendicular,
        screw: Screw,
        reserve_buoyncy: ReserveBuoyncyInBow,
        minimum_draft: MinimumDraft,
    ) -> Self {
        Self {
            ship_type,
            deadweight,
            freeboard_type,
            bow_area_min,
            aft_trim,
            forward_trim,
            bow_h_min,
            draught,
            trim,
            parameters,
            load_line,
            bow_board,
            screw,
            reserve_buoyncy,
            minimum_draft,
        }
    }
    ///
    pub fn create(&self) -> Vec<CriterionData> {
        let mut out_data = Vec::new();
        out_data.append(&mut self.load_line());
        out_data.append(&mut self.trim());
        out_data.append(&mut self.bow_board());
        out_data.append(&mut self.screw());

  /*       if self.freeboard_type == "B"
            && self.ship_type == ShipType::Tanker
            && self.ship_type == ShipType::ChemicalTanker
            && self.ship_type == ShipType::GasCarrier
        {
            out_data.push(self.reserve_buoyncy());
        }

        if (self.ship_type == ShipType::Tanker && self.deadweight >= 20000.)
            || (self.ship_type == ShipType::OilTanker && self.deadweight >= 30000.)
        {
            out_data.append(&mut self.draught_min());
        }
 */       
        out_data
    }
    /// Осадка по грузовой марке
    pub fn load_line(&self) -> Vec<CriterionData> {
        let mut res = Vec::new();
        match self.load_line.calculate() {
            Ok(load_line) => {
                for (name, z_fix, z_target) in load_line {
                    res.push(CriterionData::new_result(
                        CriterionID::from(name),
                        z_fix,
                        z_target,
                    ));
                }
            }
            Err(err) => (),
    /*        Err(err) => {
                res.push(CriterionData::new_error(
                    CriterionID::LoadLineDraftSB,
                    "Ошибка расчета уровня заглубления для осадок судна: ".to_owned()
                        + &err.to_string(),
                ));
                res.push(CriterionData::new_error(
                    CriterionID::LoadLineDraftPS,
                    "Ошибка расчета уровня заглубления для осадок судна: ".to_owned()
                        + &err.to_string(),
                ));
            }*/
        };
        res
    }
    /// Максимальный и минимальный дифферент
    pub fn trim(&self) -> Vec<CriterionData> {
        let (aft_trim, forward_trim) = match self.trim.value() {
            Ok((_, current_trim)) => (
                CriterionData::new_result(CriterionID::MaximumAftTrim, current_trim, self.aft_trim),
                CriterionData::new_result(
                    CriterionID::MaximumForwardTrim,
                    current_trim,
                    self.forward_trim,
                ),
            ),
            Err(err) => (
                CriterionData::new_error(
                    CriterionID::MaximumAftTrim,
                    "Ошибка расчета минимального дифферента: ".to_owned() + &err.to_string(),
                ),
                CriterionData::new_error(
                    CriterionID::MaximumForwardTrim,
                    "Ошибка расчета максимального дифферента: ".to_owned() + &err.to_string(),
                ),
            ),
        };
        vec![aft_trim, forward_trim]
    }
    /// Высота на носовом перпендикуляре
    pub fn bow_board(&self) -> Vec<CriterionData> {
        let mut res = Vec::new();
        match self.bow_board.calculate() {
            Ok(bow_board) => {
                for (_, y, delta_h) in bow_board {
                    res.push(if y <= 0. {
                        CriterionData::new_result(CriterionID::DepthAtForwardPerpendicularPS, delta_h, self.bow_h_min)
                    } else {
                        CriterionData::new_result(CriterionID::DepthAtForwardPerpendicularSB, delta_h, self.bow_h_min)
                    });
                }
            }
            Err(err) => {
                res.push(CriterionData::new_error(
                    CriterionID::DepthAtForwardPerpendicularPS,
                    "Ошибка расчета высоты на носовом перпендикуляре: ".to_owned()
                        + &err.to_string(),
                ));
                res.push(CriterionData::new_error(
                    CriterionID::DepthAtForwardPerpendicularSB,
                    "Ошибка расчета высоты на носовом перпендикуляре: ".to_owned()
                        + &err.to_string(),
                ));
            }
        };
        res
    }
    /// Заглубление винта
    pub fn screw(&self) -> Vec<CriterionData> {
        let mut res = Vec::new();
        match self.screw.calculate() {
            Ok(screw) => {
                for (_, y, percent) in screw {
                    res.push(if y < -1. {
                        CriterionData::new_result(CriterionID::ScrewImmersionPS, percent, 100.)
                    } else if y > 1. {
                        CriterionData::new_result(CriterionID::ScrewImmersionSB, percent, 100.)
                    } else {
                        CriterionData::new_result(CriterionID::ScrewImmersionCL, percent, 100.)
                    });
                }
            }
            Err(err) => {
                res.push(CriterionData::new_error(
                    CriterionID::ScrewImmersionCL,
                    "Ошибка расчета уровня заглубления для винтов судна: ".to_owned()
                        + &err.to_string(),
                ));
                res.push(CriterionData::new_error(
                    CriterionID::ScrewImmersionPS,
                    "Ошибка расчета уровня заглубления для винтов судна: ".to_owned()
                        + &err.to_string(),
                ));
                res.push(CriterionData::new_error(
                    CriterionID::ScrewImmersionSB,
                    "Ошибка расчета уровня заглубления для винтов судна: ".to_owned()
                        + &err.to_string(),
                ));
            }
        };
        res
    }
    /// Запас плавучести в носу
    pub fn reserve_buoyncy(&self) -> CriterionData {
        match self.reserve_buoyncy.calculate() {
            Ok(value) => CriterionData::new_result(
                CriterionID::ReserveBuoyncyInBow,
                value,
                self.bow_area_min,
            ),
            Err(err) => CriterionData::new_error(
                CriterionID::ReserveBuoyncyInBow,
                "Ошибка расчета запаса плавучести в носовой оконечности: ".to_owned()
                    + &err.to_string(),
            ),
        }
    }
 /*   /// Минимальная осадка
    pub fn draught_min(&self) -> Vec<CriterionData> {
        let minimum_draft_middle = match self.parameters.get(ParameterID::DraughtMean) {
            Some(draught_mean) => CriterionData::new_result(
                CriterionID::MinimumMiddleDraftCL,
                draught_mean,
                self.minimum_draft.middle(),
            ),
            None => CriterionData::new_error(
                CriterionID::MinimumMiddleDraftCL,
                "Ошибка расчета минимальной осадки: отсутствует ParameterID::DraughtMean!".to_owned(),
            ),
        };
        let minimum_draft_bow = match self.parameters.get(ParameterID::DraughtBow) {
            Some(draught_bow) => CriterionData::new_result(
                CriterionID::MinimumForwardDraftCL,
                draught_bow,
                self.minimum_draft.bow(),
            ),
            None => CriterionData::new_error(
                CriterionID::MinimumForwardDraftCL,
                "Ошибка расчета минимальной осадки: отсутствует ParameterID::DraughtBow!".to_owned(),
            ),
        };
        vec![minimum_draft_middle, minimum_draft_bow]
    }*/
}
