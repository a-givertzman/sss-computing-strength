//! Диаграмма плеч статической и динамической остойчивости.
use crate::{
    math::{Curve, Curve2D, ICurve, ICurve2D}, IMass, IParameters, ParameterID, Position
};

use super::metacentric_height::IMetacentricHeight;
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

type RcOpt<T> = Rc<RefCell<Option<T>>>;

/// Диаграмма плеч статической и динамической остойчивости – зависимость  
/// плеча восстанавливающего момента от угла крена судна.
#[derive(Clone)]
pub struct LeverDiagram {
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
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>, 
    /// Результат расчета - диаграмма плеч статической остойчивости
    dso: RcOpt<Vec<(f64, f64)>>,
    /// Результат расчета - диаграмма плеч статической остойчивости
    dso_curve: RcOpt<Curve>,
    /// Результат расчета - диаграмма плеч динамической остойчивости
    ddo: RcOpt<Vec<(f64, f64)>>,
    /// Угол максимума диаграммы плеч статической остойчивости
    theta_max: RcOpt<f64>,
    /// Угол пересечения с нулем диаграммы плеч статической остойчивости
    theta_last: RcOpt<f64>,
    /// Углы максимумов диаграммы плеч статической остойчивости
    max_angles: RcOpt<Vec<(f64, f64)>>,
}
///
impl LeverDiagram {
    /// Основной конструктор.
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * center_draught_shift - Отстояние центра величины погруженной части судна
    /// * data - Кривая плечей остойчивости формы для разных осадок
    /// * mean_draught - Средняя осадка
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        mass: Rc<dyn IMass>,
        center_draught_shift: Position,
        data: Curve2D,
        mean_draught: f64,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        parameters: Rc<dyn IParameters>, 
    ) -> Self {
        Self {
            mass,
            center_draught_shift,
            data,
            mean_draught,
            metacentric_height,
            parameters,
            dso: Rc::new(RefCell::new(None)),
            dso_curve: Rc::new(RefCell::new(None)),
            ddo: Rc::new(RefCell::new(None)),
            theta_max: Rc::new(RefCell::new(None)),
            theta_last: Rc::new(RefCell::new(None)),
            max_angles: Rc::new(RefCell::new(None)),
        }
    }
    /// Расчет диаграммы статической остойчивости l, м,  
    /// для каждого угла крена (11) + расчет плеча
    /// динамической остойчивости (13)
    fn calculate(&self) {
        // расчет диаграммы
        let theta = |angle_deg: i32| {
            let angle_deg = angle_deg as f64;
            let angle_rad = angle_deg * std::f64::consts::PI / 180.;
            let v1 = self.data.value(self.mean_draught, angle_deg);
            let v2 = self.metacentric_height.z_g_fix() * angle_rad.sin();
            let v3 = (self.mass.shift().y() - self.center_draught_shift.y()) * angle_rad.cos();
            let value = v1 - v2 - v3;
 //           log::info!("StabilityArm calculate расчет диаграммы: {angle_deg}, {angle_rad}, {v1}, {v2}, {v3}, {value}");
            (angle_deg, value)
        };
        let mut dso = (-90..=90)
            .map(|angle_deg| {
                theta(angle_deg)
            })
            .collect::<Vec<(f64, f64)>>();
        // если крен на левый борт то переворачиваем диаграмму
        if theta(0).1 > 0. {
            dso = dso.into_iter().map(|(a, v)| (-a, -v) ).collect();
        } 
        // нахождение максимума диаграммы
        let curve = Curve::new_catmull_rom(&dso);
        let mut angle = 45.;
        let mut max_angle = angle;
        let mut value = curve.value(angle);
        let mut max_value = value;
        let mut delta_angle = angle / 2.;
        for _i in 0..20 {
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
                angle = max_angle;
            }
            delta_angle *= 0.5;
    //        log::info!("StabilityArm calculate: value:{value} angle:{angle} max_value:{max_value} max_angle:{max_angle} delta_angle:{delta_angle} i:{_i} ");
        }
        *self.theta_max.borrow_mut() = Some(max_angle);
        dso.push((max_angle, max_value));
        dso.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        *self.dso.borrow_mut() = Some(dso);
        *self.dso_curve.borrow_mut() = Some(curve.clone());
        // нахождение углов максимумов и угла пересечения с 0
        let mut max_angles: Vec<(f64, f64)> = Vec::new();
        let mut last_value = curve.value(0.);
        let mut last_angle = 0.;
        (0..=9000).for_each(|angle_deg| {
            let angle_deg = angle_deg as f64 * 0.01;
            let value = curve.value(angle_deg);
            if value < last_value {
                if max_angles.is_empty() || max_angles.last().unwrap().1 < last_value {
                    max_angles.push((last_angle, last_value));
                }
            }
            if value <= 0. && last_value > 0. {
                *self.theta_last.borrow_mut() = Some(angle_deg);
            }
            last_value = value;
            last_angle = angle_deg
        });
        if max_angles.is_empty() {
            max_angles.push((last_angle, last_value));
        }
        *self.max_angles.borrow_mut() = Some(max_angles);
        //
        let angle_zero = *self
            .angle(0.)
            .first()
            .unwrap_or(&0.);

        let mut ddo = vec![(angle_zero, 0.)];
        let start = angle_zero.ceil() as i32;
        ddo.append(
            &mut (start..=90)
                .map(|angle_deg| {
                    let angle_deg = angle_deg as f64;
                    let value = curve.integral(angle_zero, angle_deg) * std::f64::consts::PI / 180.;
                    (angle_deg, value)
                })
                .collect::<Vec<(f64, f64)>>(),
        );
        *self.ddo.borrow_mut() = Some(ddo);
        self.parameters.add(ParameterID::Roll, angle_zero);
    }
}
///
impl ILeverDiagram for LeverDiagram {
    /// Углы крена судна соответствующие плечу кренящего момента (angle >= 0. && angle <= 90.)
    fn angle(&self, lever_moment: f64) -> Vec<f64> {
        if self.dso_curve.borrow().is_none() {
            self.calculate();
        }
        let binding = self.dso_curve.borrow();
        let curve = binding
            .as_ref()
            .expect("LeverDiagram angle error: no dso_curve!");
        let max_angle = self
            .theta_max
            .borrow()
            .expect("StabilityArm angle error: no max_angle!");
        if curve.value(max_angle) < lever_moment {
            return Vec::new();
        }
        let mut delta_angle = 22.5;
        let mut angles = vec![max_angle - delta_angle, max_angle + delta_angle];
        for _i in 0..30 {
            let last_delta_value = lever_moment - curve.value(angles[0]);
    //        log::info!("StabilityArm calculate: target:{lever_moment} angle1:{} last_delta_value:{last_delta_value} i:{_i} delta_angle:{delta_angle} ", angles[0]);
            if last_delta_value.abs() > 0.00001 {
                angles[0] += delta_angle * last_delta_value.signum();
            }
            let last_delta_value = lever_moment - curve.value(angles[1]);
    //        log::info!("StabilityArm calculate: target:{lever_moment} angle2:{} last_delta_value:{last_delta_value} i:{_i} delta_angle:{delta_angle} ", angles[1]);
            if last_delta_value.abs() > 0.00001 {
                angles[1] -= delta_angle * last_delta_value.signum();
                angles[1] = angles[1].min(90.);
            }
            delta_angle *= 0.5;
            if delta_angle < 0.001 {
                break;
            }
        }
        angles
    }
    /// Плечо кренящего момента соответствующие углу крена судна
    fn lever_moment(&self, angle: f64) -> f64 {
        assert!(
            (0. ..=90.).contains(&angle),
            "angle {angle} >= 0. && angle {angle} <= 90."
        );
        if self.dso_curve.borrow().is_none() {
            self.calculate();
        }
        self.dso_curve
            .borrow()
            .as_ref()
            .expect("LeverDiagram angle error: no dso_curve!")
            .value(angle)
    }
    /// Площадь под положительной частью диаграммы статической остойчивости (rad^2)
    fn dso_area(&self, angle1: f64, angle2: f64) -> f64 {
        assert!(
            angle1 < angle2,
            "LeverDiagram dso_area angle1 {angle1} < angle2 {angle2}"
        );
        if self.dso_curve.borrow().is_none() {
            self.calculate();
        }
        self.dso_curve
            .borrow()
            .as_ref()
            .expect("LeverDiagram dso_area error: no dso_curve!")
            .integral(angle1, angle2)
            * PI
            / 180.
    }
    /// Максимальное плечо диаграммы статической остойчивости в диапазонеб (м)
    fn dso_lever_max(&self, angle1: f64, angle2: f64) -> f64 {
        assert!(
            angle1 < angle2,
            "FakeLeverDiagram dso_lever_max angle1 {angle1} < angle2 {angle2}"
        );
        if self.dso.borrow().is_none() {
            self.calculate();
        }
        let dso = self.dso.borrow();
        let mut segment = dso
            .as_ref()
            .expect("LeverDiagram dso_lever_max error: no dso!")
            .iter()
            .filter(|v| v.0 >= angle1 && v.0 <= angle2)
            .collect::<Vec<_>>();
        segment.sort_by(|v1, v2| {
            v1.1.partial_cmp(&v2.1)
                .expect("LeverDiagram dso_lever_max partial_cmp error!")
        });
        segment
            .last()
            .expect("LeverDiagram dso_lever_max segment error: no values!")
            .1
    }
    /// Углы максимумов диаграммы плеч статической остойчивости
    fn max_angles(&self) -> Vec<(f64, f64)> {
        if self.max_angles.borrow().is_none() {
            self.calculate();
        }
        self.max_angles
            .borrow()
            .clone()
            .expect("StabilityArm max_angles error: no max_angles!")
    }
}
#[doc(hidden)]
pub trait ILeverDiagram {
    /// Углы крена судна соответствующие плечу кренящего момента
    fn angle(&self, lever_moment: f64) -> Vec<f64>;
    /// Плечо кренящего момента соответствующие углу крена судна (angle >= 0. && angle <= 90.)
    fn lever_moment(&self, angle: f64) -> f64;
    /// Площадь под положительной частью диаграммы статической остойчивости (rad^2)
    fn dso_area(&self, angle1: f64, angle2: f64) -> f64;
    /// Максимальное плечо диаграммы статической остойчивости в диапазонеб (м)
    fn dso_lever_max(&self, angle1: f64, angle2: f64) -> f64;
    /// Углы и плечи максимумов диаграммы плеч статической остойчивости
    fn max_angles(&self) -> Vec<(f64, f64)>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeLeverDiagram {
    angle: Vec<f64>,
    lever_moment: f64,
    dso_area: f64,
    dso_lever_max: f64,
    max_angles: Vec<(f64, f64)>,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeLeverDiagram {
    pub fn new(
        angle: Vec<f64>,
        lever_moment: f64,
        dso_area: f64,
        dso_lever_max: f64,
        max_angles: Vec<(f64, f64)>,
    ) -> Self {
        Self {
            angle,
            lever_moment,
            dso_area,
            dso_lever_max,
            max_angles,
        }
    }
}
#[doc(hidden)]
impl ILeverDiagram for FakeLeverDiagram {
    /// Углы крена судна соответствующие плечу кренящего момента
    fn angle(&self, _: f64) -> Vec<f64> {
        self.angle.clone()
    }
    /// Плечо кренящего момента соответствующие углу крена судна (angle >= 0. && angle <= 90.)
    fn lever_moment(&self, _: f64) -> f64 {
        self.lever_moment
    }
    /// Площадь под положительной частью диаграммы статической остойчивости
    fn dso_area(&self, angle1: f64, angle2: f64) -> f64 {
        assert!(
            angle1 < angle2,
            "FakeLeverDiagram dso_area angle1 {angle1} < angle2 {angle2}"
        );
        self.dso_area
    }
    /// Максимальное плечо диаграммы статической остойчивости в диапазонеб (м)
    fn dso_lever_max(&self, angle1: f64, angle2: f64) -> f64 {
        assert!(
            angle1 < angle2,
            "FakeLeverDiagram dso_lever_max angle1 {angle1} < angle2 {angle2}"
        );
        self.dso_lever_max
    }
    /// Углы и плечи максимумов диаграммы плеч статической остойчивости
    fn max_angles(&self) -> Vec<(f64, f64)> {
        self.max_angles.clone()
    }
}
