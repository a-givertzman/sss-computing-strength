/// Высота на носовом перпендикуляре
use std::{f64::consts::PI, rc::Rc};

use crate::{data::structs::BowBoardParsedData, draught::IDraught, Error, IParameters, ParameterID};

/// Высота в носу, определяемая как расстояние по вертикали  
/// на носовом перпендикуляре между ватерлинией и верхней кромкой  
/// открытой палубы у борта, должна быть не менее расчетной  
/// минимальной высоты в носу F_b
pub struct DepthAtForwardPerpendicular {
    /// Осадка судна
    draught: Rc<dyn IDraught>,
    /// Координаты носового перпендикуляра судна
    /// относительно центра
    bow_board: Vec<BowBoardParsedData>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
//
impl DepthAtForwardPerpendicular {
    /// Конструктор по умолчанию.
    /// * draught - Осадка судна
    /// * data - Координаты носового перпендикуляра судна относительно центра
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        draught: Rc<dyn IDraught>,
        bow_board: Vec<BowBoardParsedData>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            draught,
            bow_board,
            parameters, 
        }
    }
    /// Расчет расстояние по вертикали в точке носового перпендикуляра
    /// (name, y, delta_h)
    pub fn calculate(&self) -> Result<Vec<(String, f64, f64)>, Error> {
        let roll = self
            .parameters
            .get(ParameterID::Roll)
            .ok_or(Error::FromString("DepthAtForwardPerpendicular calculate error: no ParameterID::Roll!".to_string()))? * PI / 180.;
        let trim = self
            .parameters
            .get(ParameterID::TrimDeg)
            .ok_or(Error::FromString("DepthAtForwardPerpendicular calculate error: no ParameterID::Trim!".to_string()))? * PI / 180.;
        let mut result = Vec::new();
        for v in self.bow_board.iter() {
            let delta_h = (v.pos.z() - v.pos.y() * roll.sin() - self.draught.value(v.pos.x())?)*trim.cos();
            result.push((v.name.clone(), v.pos.y(), delta_h));
        }
        Ok(result)
    }
}
