//! Диаграмма плеч статической остойчивости.
use crate::metacentric_height::IMetacentricHeight;
use crate::Integral;
use crate::{math::Curve, Curve2D, ICurve, ICurve2D};
use std::rc::Rc;

/// Диаграмма плеч статической остойчивости – зависимость  
/// плеча восстанавливающего момента от угла крена судна.
#[derive(Clone)]
pub struct StabilityArm {
    /// Кривае плечей остойчивости формы для разных осадок
    data: Curve2D,
    /// средняя осадка
    mean_draught: f64,
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Результат расчета  - диаграмма плеч статической остойчивости
    diagram: Option<Vec<(f64, f64)>>,
    /// Угол максимума диаграмма плеч статической остойчивости
    max_angle: Option<f64>,
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
            max_angle: None,
            angle_static_roll: None,
            arm_dynamic_stability: None,
        }
    }
    /// Расчет диаграммы статической остойчивости l, м,  
    /// для каждого угла крена (11) + расчет плеча
    /// динамической остойчивости (13)
    fn calculate(&mut self) {
        // расчет диаграммы
        let mut diagram = (-90..=90)
            .map(|angle_deg| {
                let angle_deg = angle_deg as f64;
                //         let value = self.data.value(self.data.get(MEAN_DRAUGHT).to_f64(), angle_deg)
                let value = self.data.value(self.mean_draught, angle_deg)
                    - self.metacentric_height.z_g_fix()
                        * (angle_deg * std::f64::consts::PI / 180.).sin();
                (angle_deg, value)
            })
            .collect::<Vec<(f64, f64)>>();
        // нахождение максимума диаграммы
        let curve = Curve::new_catmull_rom(&diagram);
        let mut angle = 45.;
        let mut angle_max = angle;
        let mut value = curve.value(angle);
        let mut value_max = value;
        let mut delta_angle = angle / 2.;
        for i in 0..20 {
            let delta_angle_l = angle - delta_angle;
            let value_l = curve.value(delta_angle_l);
            let delta_angle_r = angle + delta_angle;
            let value_r = curve.value(delta_angle_r);
            if value_l >= value_r {
                value = value_l;
                angle -= delta_angle;
            } else {
                value = value_r;
                angle += delta_angle;
            }
            if value >= value_max {
                value_max = value;
                angle_max = angle;
            } else {
                value = value_max;
                angle = angle_max;
            }
            delta_angle *= 0.5;
            //        log::info!("StabilityArm calculate: value:{value} angle:{angle} value_max:{value_max} angle_max:{angle_max} delta_angle:{delta_angle} i:{i} ");
        }
        self.max_angle = Some(angle_max);
        diagram.push((angle_max, value_max));
        diagram.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.diagram = Some(diagram.clone());
        //
        let angle_static_roll = *self
            .angle(self.metacentric_height.l_0().abs())
            .first()
            .expect("StabilityArm calculate err: no values");
        //
        let arm_dynamic_stability = curve.integral(0., angle_static_roll);
        //        log::info!("\t StabilityArm diagram:{:?}  angle_static_roll:{angle_static_roll} arm_dynamic_stability:{arm_dynamic_stability}", diagram);
        log::info!("\t StabilityArm  angle_static_roll:{angle_static_roll} arm_dynamic_stability:{arm_dynamic_stability}");
        self.diagram = Some(diagram);
        self.angle_static_roll = Some(angle_static_roll);
        self.arm_dynamic_stability = Some(arm_dynamic_stability);
    }
}
///
impl IStabilityArm for StabilityArm {
    /// Углы крена судна соответствующие плечу кренящего момента
    fn angle(&mut self, target: f64) -> Vec<f64> {
        if self.diagram.is_none() {
            self.calculate();
        }
        let curve = Curve::new_linear(
            &self
                .diagram
                .clone()
                .expect("StabilityArm angle error: no diagram!"),
        );
        let max_angle = self
            .max_angle
            .expect("StabilityArm angle error: no max_angle!");
        if curve.value(max_angle) < target {
            return Vec::new();
        }
        let mut angles = vec![max_angle * 0.5, max_angle * 1.5];
        let mut delta_angle = max_angle * 0.25;
        for _ in 0..20 {
            let last_delta_value = target - curve.value(angles[0]);
            //log::info!("StabilityArm calculate: l_0:{l_0} angle:{angle} last_delta_value:{last_delta_value} i:{i} delta_angle:{delta_angle} ");
            if last_delta_value.abs() < 0.00001 {
                break;
            }
            angles[0] += delta_angle * last_delta_value.signum();
            let last_delta_value = target - curve.value(angles[1]);
            //log::info!("StabilityArm calculate: l_0:{l_0} angle:{angle} last_delta_value:{last_delta_value} i:{i} delta_angle:{delta_angle} ");
            if last_delta_value.abs() < 0.00001 {
                break;
            }
            angles[1] -= delta_angle * last_delta_value.signum();
            delta_angle *= 0.5;
        }

        /*       angles = angles
        .into_iter()
        .map(|mut angle| {
            let mut delta_angle = max_angle * 0.25;
            for _ in 0..20 {
                let last_delta_value = target - curve.value(angle);
                //log::info!("StabilityArm calculate: l_0:{l_0} angle:{angle} last_delta_value:{last_delta_value} i:{i} delta_angle:{delta_angle} ");
                if last_delta_value.abs() < 0.00001 {
                    break;
                }
                angle += delta_angle * last_delta_value.signum();
                delta_angle *= 0.5;
            }
            angle
        })
        .collect();*/
        angles
    }
    /// Диаграмма статической остойчивости
    fn diagram(&mut self) -> Vec<(f64, f64)> {
        if self.diagram.is_none() {
            self.calculate();
        }
        self.diagram
            .clone()
            .expect("StabilityArm diagram error: no diagram!")
    }
    /// Угол начального статического крена судна (12)
    fn angle_static_roll(&mut self) -> f64 {
        if self.diagram.is_none() {
            self.calculate();
        }
        self.angle_static_roll
            .clone()
            .expect("StabilityArm angle_roll error: no angle_roll!")
    }
}
#[doc(hidden)]
pub trait IStabilityArm {
    /// Углы крена судна соответствующие плечу кренящего момента
    fn angle(&mut self, target: f64) -> Vec<f64>;
    /// Диаграмма статической остойчивости
    fn diagram(&mut self) -> Vec<(f64, f64)>;
    /// Угол начального статического крена судна (12)
    fn angle_static_roll(&mut self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeStabilityArm {
    angle: Vec<f64>,
    diagram: Vec<(f64, f64)>,
    angle_static_roll: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeStabilityArm {
    pub fn new(angle: Vec<f64>, diagram: Vec<(f64, f64)>, angle_static_roll: f64) -> Self {
        Self {
            angle,
            diagram,
            angle_static_roll,
        }
    }
}
#[doc(hidden)]
impl IStabilityArm for FakeStabilityArm {
    /// Углы крена судна соответствующие плечу кренящего момента
    fn angle(&mut self, _: f64) -> Vec<f64> {
        self.angle.clone()
    }
    /// Диаграмма статической остойчивости
    fn diagram(&mut self) -> Vec<(f64, f64)> {
        self.diagram.clone()
    }
    /// Угол начального статического крена судна (12)
    fn angle_static_roll(&mut self) -> f64 {
        self.angle_static_roll
    }
}
