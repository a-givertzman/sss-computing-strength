//! Диаграмма плеч статической и динамической остойчивости.
use crate::{math::{Curve, Curve2D, ICurve, ICurve2D}, IMass, Position};

use super::metacentric_height::IMetacentricHeight;
use std::rc::Rc;

/// Диаграмма плеч статической и динамической остойчивости – зависимость  
/// плеча восстанавливающего момента от угла крена судна.
#[derive(Clone)]
pub struct StabilityArm {
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    mass: Rc<dyn IMass>, 
    /// Отстояние центра величины погруженной части судна
    center_draught_shift: Position, 
    /// Кривая плечей остойчивости формы для разных осадок
    data: Curve2D,
    /// Средняя осадка
    mean_draught: f64,
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Результат расчета  - диаграмма плеч статической остойчивости
    dso: Option<Vec<(f64, f64)>>,
    /// Результат расчета  - диаграмма плеч динамической остойчивости
    ddo: Option<Vec<(f64, f64)>>,
    /// Угол максимума диаграмма плеч статической остойчивости
    theta_max: Option<f64>,
    /// Угол начального статического крена судна
    angle_static_roll: Option<f64>,
}
///
impl StabilityArm {
    /// Основной конструктор.
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * center_draught_shift - Отстояние центра величины погруженной части судна
    /// * data - Кривая плечей остойчивости формы для разных осадок
    /// * mean_draught - Средняя осадка
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    pub fn new(
        mass: Rc<dyn IMass>, 
        center_draught_shift: Position,
        data: Curve2D,
        mean_draught: f64,
        metacentric_height: Rc<dyn IMetacentricHeight>,
    ) -> Self {
        Self {
            mass,
            center_draught_shift,
            data,
            mean_draught,
            metacentric_height,
            dso: None,
            ddo: None,
            theta_max: None,
            angle_static_roll: None,
        }
    }
    /// Расчет диаграммы статической остойчивости l, м,  
    /// для каждого угла крена (11) + расчет плеча
    /// динамической остойчивости (13)
    fn calculate(&mut self) {
        // расчет диаграммы
        let delta_y = self.mass.shift().y() - self.center_draught_shift.y();
        let mut dso = (-90..=90)
            .map(|angle_deg| {
                let angle_deg = angle_deg as f64;
                let angle_rad = angle_deg * std::f64::consts::PI / 180.;
                let value = self.data.value(self.mean_draught, angle_deg)
                    - self.metacentric_height.z_g_fix()
                        * angle_rad.sin()
                    - delta_y
                        * angle_rad.cos();
                (angle_deg, value)
            })
            .collect::<Vec<(f64, f64)>>();
        // нахождение максимума диаграммы
        let curve = Curve::new_catmull_rom(&dso);
        let mut angle = 45.;
        let mut max_angle = angle;
        let mut value = curve.value(angle);
        let mut max_value = value;
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
            if value >= max_value {
                max_value = value;
                max_angle = angle;
            } else {
                value = max_value;
                angle = max_angle;
            }
            delta_angle *= 0.5;
            //    log::info!("StabilityArm calculate: value:{value} angle:{angle} max_value:{max_value} max_angle:{max_angle} delta_angle:{delta_angle} i:{i} ");
        }
        self.theta_max = Some(max_angle);
        dso.push((max_angle, max_value));
        dso.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.dso = Some(dso);
        //
        let angle_zero = *self
            .angle(0.)
            .first()
            .expect("StabilityArm calculate err: no angle_zero");

        let mut ddo = vec![(angle_zero, 0.)];
        let start = angle_zero.ceil() as i32;
        ddo.append(&mut (start..=90).map(|angle_deg| {
            let angle_deg = angle_deg as f64;
            let value = curve.integral(angle_zero, angle_deg) * std::f64::consts::PI / 180.;
            (angle_deg, value)
        })
        .collect::<Vec<(f64, f64)>>());
        self.ddo = Some(ddo);
    }
}
///
impl IStabilityArm for StabilityArm {
    /// Углы крена судна соответствующие плечу кренящего момента
    fn angle(&mut self, target: f64) -> Vec<f64> {
        if self.dso.is_none() {
            self.calculate();
        }
        let curve = Curve::new_linear(
            &self
                .dso
                .clone()
                .expect("StabilityArm angle error: no diagram!"),
        );
        let max_angle = self
            .theta_max
            .expect("StabilityArm angle error: no max_angle!");
        if curve.value(max_angle) < target {
            return Vec::new();
        }

        let mut delta_angle = 22.5;
        let mut angles = vec![max_angle - delta_angle, max_angle + delta_angle];        
        for i in 0..20 {
            let last_delta_value = target - curve.value(angles[0]);
  //          log::info!("StabilityArm calculate: target:{target} angle1:{} last_delta_value:{last_delta_value} i:{i} delta_angle:{delta_angle} ", angles[0]);
            if last_delta_value.abs() < 0.00001 {
                break;
            }
            angles[0] += delta_angle * last_delta_value.signum();
            let last_delta_value = target - curve.value(angles[1]);
 //           log::info!("StabilityArm calculate: target:{target} angle2:{} last_delta_value:{last_delta_value} i:{i} delta_angle:{delta_angle} ", angles[1]);
            if last_delta_value.abs() < 0.00001 {
                break;
            }
            angles[1] -= delta_angle * last_delta_value.signum();
            delta_angle *= 0.5;
        }

        angles
    }
    /// Диаграмма статической остойчивости
    fn dso(&mut self) -> Vec<(f64, f64)> {
        if self.dso.is_none() {
            self.calculate();
        }
        self.dso
            .clone()
            .expect("StabilityArm diagram error: no diagram!")
    }
    /// Диаграмма динамической остойчивости
    fn ddo(&mut self) -> Vec<(f64, f64)> {
        if self.ddo.is_none() {
            self.calculate();
        }
        self.ddo
            .clone()
            .expect("StabilityArm diagram error: no diagram!")
    }
    /// Угол, соответствующий максимуму диаграммы статической остойчивости
    fn theta_max(&mut self) -> f64 {
        if self.theta_max.is_none() {
            self.calculate();
        }
        self.theta_max
            .clone()
            .expect("StabilityArm theta_max error: no theta_max!")
    }
}
#[doc(hidden)]
pub trait IStabilityArm {
    /// Углы крена судна соответствующие плечу кренящего момента
    fn angle(&mut self, target: f64) -> Vec<f64>;
    /// Диаграмма статической остойчивости
    fn dso(&mut self) -> Vec<(f64, f64)>;
    /// Диаграмма динамической остойчивости
    fn ddo(&mut self) -> Vec<(f64, f64)>;
    /// Угол, соответствующий максимуму диаграммы статической остойчивости
    fn theta_max(&mut self) -> f64;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeStabilityArm {
    angle: Vec<f64>,
    dso: Vec<(f64, f64)>,
    ddo: Vec<(f64, f64)>,
    theta_max: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeStabilityArm {
    pub fn new(angle: Vec<f64>, dso: Vec<(f64, f64)>, ddo: Vec<(f64, f64)>, theta_max: f64,) -> Self {
        Self {
            angle,
            dso,
            ddo,
            theta_max,
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
    fn dso(&mut self) -> Vec<(f64, f64)> {
        self.dso.clone()
    }
    /// Диаграмма динамической остойчивости
    fn ddo(&mut self) -> Vec<(f64, f64)> {
        self.ddo.clone()
    }
    /// Угол, соответствующий максимуму диаграммы статической остойчивости
    fn theta_max(&mut self) -> f64 {
        self.theta_max
    }
}
