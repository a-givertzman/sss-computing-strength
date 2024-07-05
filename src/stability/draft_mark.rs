
//! Расчет уровня заглубления для координат отметок заглубления на корпусе судна 

use std::{collections::HashMap, f64::consts::PI, rc::Rc};

use crate::{draught::IDraught, Curve, ICurve, ILeverDiagram, IParameters, ParameterID, Position};

/// Расчет уровня заглубления для координат отметок заглубления на корпусе судна 
pub struct DraftMark {
    /// Осадка судна
    draught: Box<dyn IDraught>,
    /// Диаграмма плеч статической и динамической остойчивости
    lever_diagram: Rc<dyn ILeverDiagram>,
    /// Координаты отметок заглубления на корпусе судна 
    /// относительно центра
    points: HashMap<String, Vec<(f64, f64, f64)>>,
}
///
impl DraftMark {
    /// Конструктор по умолчанию.
    /// * draught - Осадка судна
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    /// * points - Координаты отметок заглубления на корпусе судна относительно центра
    pub fn new(
        draught: Box<dyn IDraught>,
        lever_diagram: Rc<dyn ILeverDiagram>,
        points: HashMap<String, Vec<(f64, f64, f64)>>,
    ) -> Self {
        Self {
            draught,
            lever_diagram,
            points,
        }
    }
    /// Расчет координат точек с уровнем заглубления 0
    pub fn calculate(&mut self) -> Vec<(String, (f64, f64, f64))> {
        let roll = *self.lever_diagram.angle(0.).first().expect("DraftMark calculate error: no theta0!");
        self.points.iter().filter_map(|(s, v)| {
            assert!(v.len() > 2, "DraftMark calculate v.len() > 2");
            let mut z_fix: Vec<(f64, f64, f64, f64)> = 
                v.iter().map(|v| (v.0, v.1, v.2, v.2 + v.1*(roll*PI/180.).sin() - self.draught.value(v.0)) )
                .collect();
            z_fix.sort_by(|a, b| b.3.partial_cmp(&a.3).expect("DraftMark calculate error: partial_cmp!"));
            // Если все марки ниже или выше уровня воды, то нет результата
            if z_fix[0].3.signum() == z_fix[1].3.signum() {
                return None;
            }
            // Интерполированные значения координат марок заглубления
            let fix_x = Curve::new_linear(&z_fix.iter().map(|&v| (v.3, v.0)).collect()).value(0.);
            let fix_y = Curve::new_linear(&z_fix.iter().map(|&v| (v.3, v.1)).collect()).value(0.);
            let fix_z = Curve::new_linear(&z_fix.iter().map(|&v| (v.3, v.2)).collect()).value(0.);
            Some((s.to_owned(), (fix_x, fix_y, fix_z)))
        }).collect()
    }
}
