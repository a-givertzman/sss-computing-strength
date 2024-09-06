/// Высота на носовом перпендикуляре
use std::{f64::consts::PI, rc::Rc};

use crate::{data::structs::LoadLineParsedData, draught::IDraught, Error, IParameters, ParameterID};

/// Высота в носу, определяемая как расстояние по вертикали  
/// на носовом перпендикуляре между ватерлинией и верхней кромкой  
/// открытой палубы у борта, должна быть не менее расчетной  
/// минимальной высоты в носу F_b
pub struct DepthAtForwardPerpendicular {
    /// Осадка судна
    draught: Box<dyn IDraught>,
    /// Координаты носового перпендикуляра судна
    /// относительно центра
    data: Vec<LoadLineParsedData>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
///
impl DepthAtForwardPerpendicular {
    /// Конструктор по умолчанию.
    /// * draught - Осадка судна
    /// * data - Координаты носового перпендикуляра судна относительно центра
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        draught: Box<dyn IDraught>,
        data: Vec<LoadLineParsedData>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            draught,
            data,
            parameters, 
        }
    }
    /// Расчет расстояние по вертикали в точке носового перпендикуляра
    pub fn calculate(&mut self) -> Result<Vec<(String, f64)>, Error> {
        let roll = self
            .parameters
            .get(ParameterID::Roll)
            .ok_or(Error::FromString("DepthAtForwardPerpendicular calculate error: no ParameterID::Roll!".to_string()))? * PI / 180.;
        let trim = self
            .parameters
            .get(ParameterID::Trim)
            .ok_or(Error::FromString("DepthAtForwardPerpendicular calculate error: no ParameterID::Trim!".to_string()))? * PI / 180.;
        let mut result = Vec::new();
        for v in self.data.iter() {
            let delta_h = (v.pos.z() - v.pos.y() * roll.sin() - self.draught.value(v.pos.x())?)*trim.cos();
            result.push((v.name.clone(), delta_h));
        }
        Ok(result)
    }
}
