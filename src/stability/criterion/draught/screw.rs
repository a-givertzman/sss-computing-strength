//! Расчет уровня заглубления для винтов судна

use std::{f64::consts::PI, rc::Rc};

use crate::{data::structs::ScrewParsedData, draught::IDraught, Error, IParameters, ParameterID};

/// Расчет уровня заглубления для винтов судна
pub struct Screw {
    /// Осадка судна
    draught: Rc<dyn IDraught>,
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
        draught: Rc<dyn IDraught>,
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
    /// (name, y, percent)
    pub fn calculate(&self) -> Result<Vec<(String, f64, f64)>, Error> {
        let roll = self
            .parameters
            .get(ParameterID::Roll)
            .ok_or(Error::FromString("DraftMark calculate error: no ParameterID::Roll!".to_string()))?;
        let mut result = Vec::new();
        for v in self.data.iter() {
            let z_fix = v.pos.z()  - v.pos.y() * (roll * PI / 180.).sin() - self.draught.value(v.pos.x())?;
            let percent = (1. - z_fix/v.d).min(2.).max(0.)*50.;
   //         dbg!(&v.pos, v.pos.y() * (roll * PI / 180.).sin(), self.draught.value(v.pos.x())?, z_fix, percent);
            result.push((v.name.clone(), v.pos.y(), percent));
        }
        Ok(result)
    }
}
