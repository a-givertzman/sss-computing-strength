//! Диаграмма плеч статической и динамической остойчивости.
use crate::{
    math::{Curve, Curve2D, ICurve, ICurve2D}, IShipMoment, IParameters, ParameterID, Position
};

use super::metacentric_height::IMetacentricHeight;
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

type RcOpt<T> = Rc<RefCell<Option<T>>>;

/// Диаграмма плеч статической и динамической остойчивости – зависимость  
/// плеча восстанавливающего момента от угла крена судна.
#[derive(Clone)]
pub struct LeverDiagram {
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    ship_moment: Rc<dyn IShipMoment>,
    /// Отстояние центра величины погруженной части судна
    center_draught_shift: Position,
    /// Кривая плечей остойчивости формы для разных осадок
    pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
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
    /// Результат расчета - диаграммы остойчивости, зависимость от угла, градусы
    diagram: RcOpt<Vec<(f64, f64, f64)>>,
    /// Угол максимума диаграммы плеч статической остойчивости
    theta_max: RcOpt<f64>,
    /// Углы максимумов диаграммы плеч статической остойчивости
    max_angles: RcOpt<Vec<(f64, f64)>>,
}
///
impl LeverDiagram {
    /// Основной конструктор.
    /// * ship_moment - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * center_draught_shift - Отстояние центра величины погруженной части судна
    /// * pantocaren - Кривая плечей остойчивости формы для разных осадок
    /// * mean_draught - Средняя осадка
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        ship_moment: Rc<dyn IShipMoment>,
        center_draught_shift: Position,
        pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
        mean_draught: f64,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        parameters: Rc<dyn IParameters>, 
    ) -> Self {
        Self {
            ship_moment,
            center_draught_shift,
            pantocaren,
            mean_draught,
            metacentric_height,
            parameters,
            dso: Rc::new(RefCell::new(None)),
            dso_curve: Rc::new(RefCell::new(None)),
            ddo: Rc::new(RefCell::new(None)),
            diagram: Rc::new(RefCell::new(None)),
            theta_max: Rc::new(RefCell::new(None)),
            max_angles: Rc::new(RefCell::new(None)),
        }
    }
    /// Расчет диаграммы статической остойчивости l, м,  
    /// для каждого угла крена (11) + расчет плеча
    /// динамической остойчивости (13)
    fn calculate(&self) {
        // Проверяем есть ли пантокарены в отрицательной области углов
        // Если нет то считаем что судно симметорично и зеркально
        // копируем данные в отрицательную область углов крена
        let mut pantocaren = self.pantocaren.clone();
        let mut tmp = pantocaren
            .first()
            .expect("Main pantocaren error: no data!")
            .1
            .clone();
        tmp.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        if tmp.first().expect("Main pantocaren error: no data!").0 <= 0. {
            pantocaren.iter_mut().for_each(|(_, vector)| {
                let mut negative = vector
                    .iter()
                    .filter(|(angle, _)| *angle > 0.)
                    .map(|(angle, moment)| (-angle, -moment))
                    .collect();
                vector.append(&mut negative);
                vector.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            });
        }
        let curve = Curve2D::from_values_catmull_rom(pantocaren);
        // расчет диаграммы
        let theta = |angle_deg: i32| {
            let angle_deg = angle_deg as f64;
            let angle_rad = angle_deg * std::f64::consts::PI / 180.;
            let v1 = curve.value(self.mean_draught, angle_deg);
            let v2 = self.metacentric_height.z_g_fix() * angle_rad.sin();
            let v3 = (self.ship_moment.shift().y() - self.center_draught_shift.y()) * angle_rad.cos();
            let value = v1 - v2 - v3;
 //           log::info!("StabilityArm calculate расчет диаграммы: {angle_deg}, {angle_rad}, {v1}, {v2}, {v3}, {value}");
            (angle_deg, value)
        };
        let mut dso = (-90..=90)
            .map(|angle_deg| {
                theta(angle_deg)
            })
            .collect::<Vec<(f64, f64)>>();

        // dbg!(&dso);
        // знак статического угла крена
        let mut angle_zero_signum = 1.;
        // если крен на левый борт то переворачиваем диаграмму
        if theta(0).1 > 0. {
            dso = dso.into_iter().map(|(a, v)| (-a, -v) ).collect();
            dso.sort_by(|(a1, _), (a2, _)| a1.partial_cmp(a2).expect("LeverDiagram calculate error: sort dso!") );
            angle_zero_signum = -1.; // сохраняем знак угла
        //    dbg!("theta(0).1 > 0.", &dso);
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
    //    dso.push((max_angle, max_value));
    //    dso.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        *self.dso.borrow_mut() = Some(dso.clone());
        *self.dso_curve.borrow_mut() = Some(curve.clone());
        // нахождение углов максимумов и угла пересечения с 0
        let mut max_angles: Vec<(f64, f64)> = Vec::new();
        let mut last_value = curve.value(0.);
        let mut last_angle = 0.;
        (0..=9000).for_each(|angle_deg| {
            let angle_deg = angle_deg as f64 * 0.01;
            let value = curve.value(angle_deg);
            if value < last_value {
                if max_angles.is_empty() || max_angles.last().expect("LeverDiagram calculate error: no max_angle!").1 < last_value {
                    max_angles.push((last_angle, last_value));
                }
            }
        // Угол пересечения с нулем диаграммы плеч статической остойчивости
        //    if value <= 0. && last_value > 0. {
        //        *self.theta_last.borrow_mut() = Some(angle_deg);
        //    }
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
   //     dbg!(angle_zero);
        let ddo = (-90..=90)
                .map(|angle_deg| {
                    let angle_deg = angle_deg as f64;
                    let value = if angle_deg < angle_zero { 
                        curve.integral(angle_deg, angle_zero) * std::f64::consts::PI / 180.
                    } else if angle_deg > angle_zero {
                        curve.integral(angle_zero, angle_deg) * std::f64::consts::PI / 180.
                    } else {
                        0.
                    };
          //          dbg!(angle_deg, value);
                    (angle_deg, value)
                })
                .collect::<Vec<(f64, f64)>>();
   //     dbg!(&ddo);
        *self.diagram.borrow_mut() = Some(dso.iter().zip(ddo.iter()).map(|((a1, v1), (_, v2))| (*a1, *v1, *v2) ).collect::<Vec<_>>());
    //    dbg!(&self.diagram.clone());
        *self.ddo.borrow_mut() = Some(ddo);
        self.parameters.add(ParameterID::Roll, angle_zero * angle_zero_signum);
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
    /// Диаграммы остойчивости, зависимость от угла, градусы
    fn diagram(&self) -> Vec<(f64, f64, f64)> {
        if self.diagram.borrow().is_none() {
            self.calculate();
        }
        self.diagram
            .borrow()
            .clone()
            .expect("StabilityArm diagram error: no diagram!")
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
    /// Диаграммы остойчивости, зависимость от угла, градусы
    fn diagram(&self) -> Vec<(f64, f64, f64)>;
    /// Углы и плечи максимумов диаграммы плеч статической остойчивости
    fn max_angles(&self) -> Vec<(f64, f64)>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeLeverDiagram {
    angle: Vec<f64>,
    lever_moment: f64,
    diagram: Vec<(f64, f64, f64)>,
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
        diagram: Vec<(f64, f64, f64)>,
        dso_area: f64,
        dso_lever_max: f64,
        max_angles: Vec<(f64, f64)>,
    ) -> Self {
        Self {
            angle,
            lever_moment,
            diagram,
            dso_area,
            dso_lever_max,
            max_angles,
        }
    }
}
#[doc(hidden)]
impl ILeverDiagram for FakeLeverDiagram {
    /// Углы крена судна соответствующие плечу кренящего момента, градусы
    fn angle(&self, _: f64) -> Vec<f64> {
        self.angle.clone()
    }
    /// Плечо кренящего момента соответствующие углу крена судна (angle >= 0. && angle <= 90.), м
    fn lever_moment(&self, _: f64) -> f64 {
        self.lever_moment
    }
    /// Площадь под положительной частью диаграммы статической остойчивости, м*rad
    fn dso_area(&self, angle1: f64, angle2: f64) -> f64 {
        assert!(
            angle1 < angle2,
            "FakeLeverDiagram dso_area angle1 {angle1} < angle2 {angle2}"
        );
        self.dso_area
    }
    /// Максимальное плечо диаграммы статической остойчивости в диапазоне, м
    fn dso_lever_max(&self, angle1: f64, angle2: f64) -> f64 {
        assert!(
            angle1 < angle2,
            "FakeLeverDiagram dso_lever_max angle1 {angle1} < angle2 {angle2}"
        );
        self.dso_lever_max
    }
    /// Диаграммы остойчивости, зависимость от угла, градусы
    fn diagram(&self) -> Vec<(f64, f64, f64)> {
        self.diagram.clone()
    }
    /// Углы и плечи максимумов диаграммы плеч статической остойчивости
    fn max_angles(&self) -> Vec<(f64, f64)> {
        self.max_angles.clone()
    }
}
