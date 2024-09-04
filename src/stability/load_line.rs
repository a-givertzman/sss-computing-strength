/// Проверка осадок судна
use std::{collections::HashMap, f64::consts::PI, rc::Rc};

use crate::{data::structs::LoadLineParsedData, draught::IDraught, Error, IParameters, ParameterID};

/// Расчет уровня заглубления для осадок судна
pub struct LoadLine {
    /// Осадка судна
    draught: Box<dyn IDraught>,
    /// Координаты осадок судна
    /// относительно центра
    data: Vec<LoadLineParsedData>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
///
impl LoadLine {
    /// Конструктор по умолчанию.
    /// * draught - Осадка судна
    /// * data - Координаты осадок судна относительно центра
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
    /// Расчет заглубления точки осадки
    pub fn calculate(&mut self) -> Result<Vec<(String, f64)>, Error> {
        let roll = self
            .parameters
            .get(ParameterID::Roll)
            .ok_or(Error::FromString("DraftMark calculate error: no ParameterID::Roll!".to_string()))?;
        let mut result = Vec::new();
        for v in self.data.iter() {
            let z_fix = v.pos.z()  - v.pos.y() * (roll * PI / 180.).sin() - self.draught.value(v.pos.x())?;
            result.push((v.name.clone(), z_fix));
        }
        Ok(result)
    }
}
