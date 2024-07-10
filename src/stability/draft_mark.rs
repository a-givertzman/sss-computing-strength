
//! Расчет уровня заглубления для координат отметок заглубления на корпусе судна 

use std::{collections::HashMap, f64::consts::PI, rc::Rc};

use crate::{draught::IDraught, Curve, ICurve, ILeverDiagram, IParameters, ParameterID, Position};

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
    pub fn calculate(&mut self) -> Vec<(String, (f64, f64, f64))> {
        let roll = self.parameters.get(ParameterID::Roll).expect("DraftMark calculate error: no ParameterID::Roll!");
        self.points.iter().filter_map(|(s, v)| {
            assert!(v.len() > 2, "DraftMark calculate v.len() > 2");
            let mut z_fix: Vec<(f64, f64, f64, f64)> = 
                v.iter().map(|v| (v.0, v.1, v.2, v.2 - v.1*(roll*PI/180.).sin() - self.draught.value(v.0)) )
                .collect();
            z_fix.sort_by(|a, b| a.3.abs().partial_cmp(&b.3.abs()).expect("DraftMark calculate error: partial_cmp!"));
            // Если все марки ниже или выше уровня воды
            if z_fix[0].3.signum() == z_fix[1].3.signum() {
                if z_fix[0].3.abs() < 0.001 { // если уровень прямо на марке
                    return Some((s.to_owned(), (z_fix[0].0, z_fix[0].1, z_fix[0].2)))
                }
                return None; // иначе нет результата
            }
            // Интерполированные значения координат марок заглубления
            let fix_x = Curve::new_linear(&z_fix.iter().map(|&v| (v.3, v.0)).collect()).value(0.);
            let fix_y = Curve::new_linear(&z_fix.iter().map(|&v| (v.3, v.1)).collect()).value(0.);
            let fix_z = Curve::new_linear(&z_fix.iter().map(|&v| (v.3, v.2)).collect()).value(0.);
            Some((s.to_owned(), (fix_x, fix_y, fix_z)))
        }).collect()
    }
}
