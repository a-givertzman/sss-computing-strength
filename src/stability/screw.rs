//! Расчет уровня заглубления для координат отметок заглубления на корпусе судна

use std::{f64::consts::PI, rc::Rc};

use crate::{data::structs::ScrewParsedData, draught::IDraught, Error, IParameters, ParameterID};

/// Расчет уровня заглубления для винтов судна
pub struct Screw {
    /// Осадка судна
    draught: Box<dyn IDraught>,
    /// Координаты винтов судна относительно центра
    data: Vec<ScrewParsedData>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
///
impl Screw {
    /// Конструктор по умолчанию.
    /// * draught - Осадка судна
    /// * data - Координаты винтов судна относительно центра
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        draught: Box<dyn IDraught>,
        data: Vec<ScrewParsedData>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            draught,
            data,
            parameters, 
        }
    }
    /// Расчет процента заглубления винта
    pub fn calculate(&mut self) -> Result<Vec<(String, f64)>, Error> {
        let roll = self
            .parameters
            .get(ParameterID::Roll)
            .ok_or(Error::FromString("DraftMark calculate error: no ParameterID::Roll!".to_string()))?;
        let mut result = Vec::new();
        for v in self.data.iter() {
            let z_fix = v.pos.z()  - v.pos.y() * (roll * PI / 180.).sin() - self.draught.value(v.pos.x())?;
            let percent = (1. - z_fix/v.d).min(2.).max(0.)*50.;
   //         dbg!(&v.pos, v.pos.y() * (roll * PI / 180.).sin(), self.draught.value(v.pos.x())?, z_fix, percent);
            result.push((v.name.clone(), percent));
        }
        Ok(result)
    }
}
