//! Диаграмма плеч статической остойчивости.
use std::rc::Rc;
use crate::Integral;
use crate::{math::Curve, Curve2D, ICurve, ICurve2D};
use crate::metacentric_height::{IMetacentricHeight, MetacentricHeight};
use crate::math::vec::IntegralCotes;

/// Диаграмма плеч статической остойчивости – зависимость  
/// плеча восстанавливающего момента от угла крена судна.
pub struct StabilityArm {
    /// Кривае плечей остойчивости формы для разных осадок
    data: Curve2D,
    /// средняя осадка
    mean_draught: f64,
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Результат расчета  - диаграмма плеч статической остойчивости
    diagram: Option<Vec<(f64, f64)>>,
    /// Угол начального статического крена судна
    angle_static_roll: Option<f64>,
    /// Плечо диаграмы динамической остойчивости
    arm_dynamic_stability: Option<f64>,
}
///
impl StabilityArm {
    /// Основной конструктор.
    pub fn new(
        data: Curve2D,
        mean_draught: f64,
        metacentric_height: Rc<dyn IMetacentricHeight>,
    ) -> Self {
        Self {
            data,
            mean_draught,
            metacentric_height,
            diagram: None,
            angle_static_roll: None,
            arm_dynamic_stability: None,
        }
    }
    /// Угол начального статического крена судна  
    /// соответствующий плечу кренящего момента $L_0$ (12)
    /// * mean_draught: среднаяя осадка
    /// * roll_moment: плече кренящего момента
    pub fn angle(&mut self, roll_moment: f64) -> f64 {
        if self.diagram.is_none() {
            self.calculate();
        }
        Curve::new(
            self.diagram
                .as_ref()
                .expect("StabilityArm angle error: no diagram!"),
        )
        .value(roll_moment)
    }
    /// Диаграмма статической остойчивости
    pub fn diagram(&mut self) -> Vec<(f64, f64)> {
        if self.diagram.is_none() {
            self.calculate();
        }
        self.diagram
            .clone()
            .expect("StabilityArm diagram error: no diagram!")
    }
    /// Угол начального статического крена судна (12)
    pub fn angle_static_roll(&mut self) -> f64 {
        if self.diagram.is_none() {
            self.calculate();
        }
        self.angle_static_roll
            .clone()
            .expect("StabilityArm angle_roll error: no angle_roll!")
    }
    /// Расчет диаграммы статической остойчивости l, м,  
    /// для каждого угла крена (11) + расчет плеча 
    /// динамической остойчивости (13)
    fn calculate(&mut self) {
        let diagram = (0..=90)
            .map(|angle_deg| {
                let angle_deg = angle_deg as f64;
                let value = self.data.value(self.mean_draught, angle_deg)
                    - self.metacentric_height.z_g_fix()
                        * (angle_deg * std::f64::consts::PI / 180.).sin();
   /*             let moment = self.data.value(self.mean_draught, angle_deg);
                let sin_angle = (angle_deg * std::f64::consts::PI / 180.).sin();
                let value = moment - self.metacentric_height.z_g_fix()*sin_angle;                
                dbg!(moment, self.metacentric_height.z_g_fix(), angle_deg, sin_angle, value);
*/
                (angle_deg, value)
            })
            .collect::<Vec<(f64, f64)>>();

        let l_0 = self.metacentric_height.l_0().abs();
        let curve = Curve::new(&diagram);
        let mut angle = 30.;
        let mut delta_angle = angle*0.5;
        for i in 0..20 {
            let last_delta_value = l_0 - curve.value(angle);
            log::info!("StabilityArm calculate: l_0:{l_0} angle:{angle} last_delta_value:{last_delta_value} i:{i} delta_angle:{delta_angle} ");
            if last_delta_value.abs() < 0.00001 {
                break;
            }
            angle += delta_angle*last_delta_value.signum();
            delta_angle *= 0.5;
        }
        let angle_static_roll = angle;
        //let arm_dynamic_stability = diagram.iter().filter(|(angle, _)| angle <= &angle_static_roll ).map(|(_, value)| *value ).collect::<Vec<f64>>().integral_cotes(1.);
        let mut tmp: Vec<(f64, f64)> = diagram.clone().into_iter().filter(|(angle, _)| *angle < angle_static_roll ).collect();
        tmp.push((angle_static_roll, l_0));
        let arm_dynamic_stability = tmp.integral();
        log::info!("\t StabilityArm diagram:{:?}  angle_static_roll:{angle_static_roll} arm_dynamic_stability:{arm_dynamic_stability}", diagram);
        self.diagram = Some(diagram);
        self.angle_static_roll = Some(angle_static_roll);
        self.arm_dynamic_stability = Some(arm_dynamic_stability);
    }
}
