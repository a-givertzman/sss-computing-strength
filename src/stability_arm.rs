//! Диаграмма плеч статической остойчивости.

use crate::{math::Curve, Curve2D, ICurve, ICurve2D};

/// Диаграмма плеч статической остойчивости – зависимость  
/// плеча восстанавливающего момента от угла крена судна.
pub struct StabilityArm {
    data: Curve2D,    
    diagram: Option<Vec<(f64, f64)>>
}
///
impl StabilityArm {
    ///
    pub fn new(data: Curve2D) -> Self {
        Self{ data, diagram: None, }
    }
    /// Угол начального статического крена судна  
    /// соответствующий плечу кренящего момента $L_0$ (12)
    /// * mean_draught: среднаяя осадка
    /// * roll_moment: плече кренящего момента
    pub fn angle(&mut self, mean_draught: f64, roll_moment: f64) -> f64 {
        if self.diagram.is_none() {
            self.calculate(mean_draught);
        }

        Curve::new(self.diagram.as_ref().expect("StabilityArm angle error: no diagram!")).value(roll_moment)
    }
    /// Расчет диаграммы статической остойчивости l, м,  
    /// для каждого угла крена (11)
    pub fn calculate(&mut self, mean_draught: f64) -> Vec<(f64, f64)> {
        let delta = 90./self.data.len() as f64;
        self.diagram = Some((0..self.data.len()).into_iter().map(|v| {
            let angle = v as f64*delta;
            (angle, self.data.value(mean_draught, angle))
        }).collect());

        self.diagram.clone().unwrap()
    }
}