//! Расчет уровня заглубления для координат отметок заглубления на корпусе судна

use std::{collections::HashMap, f64::consts::PI, rc::Rc};

use crate::{draught::IDraught, Curve, Error, ICurve, IParameters, ParameterID};

/// Расчет уровня заглубления для координат отметок заглубления на корпусе судна
pub struct DraftMark {
    /// Осадка судна
    draught: Box<dyn IDraught>,
    /// Координаты отметок заглубления на корпусе судна
    /// относительно центра
    points: HashMap<String, Vec<(f64, f64, f64)>>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
///
impl DraftMark {
    /// Конструктор по умолчанию.
    /// * draught - Осадка судна
    /// * points - Координаты отметок заглубления на корпусе судна относительно центра
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        draught: Box<dyn IDraught>,
        points: HashMap<String, Vec<(f64, f64, f64)>>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            draught,
            points,
            parameters,
        }
    }
    /// Расчет координат точек с уровнем заглубления 0
    pub fn calculate(&mut self) -> Result<Vec<(String, (f64, f64, f64))>, Error> {
        let roll = self
            .parameters
            .get(ParameterID::Roll)
            .ok_or(Error::FromString("DraftMark calculate error: no ParameterID::Roll!".to_string()))?;
        let mut result = Vec::new();
        for (s, v) in self.points.iter() {
            if v.len() <= 2 {
                return Err(Error::FromString("DraftMark calculate error: v.len() < 2".to_string()));
            } 
            let mut z_fix: Vec<(f64, f64, f64, f64)> = Vec::new();
            for v in v.iter() {
                z_fix.push((
                    v.0,
                    v.1,
                    v.2,
                    v.2 - v.1 * (roll * PI / 180.).sin() - self.draught.value(v.0)?,
                ));
            }
            z_fix.sort_by(|a, b| {
                a.3.abs()
                    .partial_cmp(&b.3.abs())
                    .expect("DraftMark calculate error: partial_cmp!")
            });
            // Если все марки ниже или выше уровня воды
            if z_fix[0].3.signum() == z_fix[1].3.signum() {
                if z_fix[0].3.abs() < 0.001 {
                    // если уровень прямо на марке
                    result.push((s.to_owned(), (z_fix[0].0, z_fix[0].1, z_fix[0].2)));
                }
                continue;
            }
            // Интерполированные значения координат марок заглубления
            let fix_x =
                Curve::new_linear(&z_fix.iter().map(|&v| (v.3, v.0)).collect())?.value(0.)?;
            let fix_y =
                Curve::new_linear(&z_fix.iter().map(|&v| (v.3, v.1)).collect())?.value(0.)?;
            let fix_z =
                Curve::new_linear(&z_fix.iter().map(|&v| (v.3, v.2)).collect())?.value(0.)?;
            result.push((s.to_owned(), (fix_x, fix_y, fix_z)));
        }
        Ok(result)
    }
}
