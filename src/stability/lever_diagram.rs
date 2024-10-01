//! Диаграмма плеч статической и динамической остойчивости.
use crate::{
    math::{Curve, Curve2D, ICurve, ICurve2D},
    Error, IParameters, IShipMoment, ParameterID, Position,
};

use super::metacentric_height::IMetacentricHeight;
use std::{cell::RefCell, f64::consts::PI, rc::Rc, sync::Arc};

type RcOpt<T> = Rc<RefCell<Option<T>>>;

/// Диаграмма плеч статической и динамической остойчивости – зависимость  
/// плеча восстанавливающего момента от угла крена судна.
#[derive(Clone)]
pub struct LeverDiagram {
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    ship_moment: Arc<dyn IShipMoment>,
    /// Отстояние центра величины погруженной части судна
    center_draught_shift: Position,
    /// Кривая плечей остойчивости формы для разных осадок
    pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
    /// Средняя осадка
    mean_draught: f64,
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Arc<dyn IMetacentricHeight>,
    /// Набор результатов расчетов для записи в БД
    parameters: Arc<dyn IParameters>,
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
        ship_moment: Arc<dyn IShipMoment>,
        center_draught_shift: Position,
        pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
        mean_draught: f64,
        metacentric_height: Arc<dyn IMetacentricHeight>,
        parameters: Arc<dyn IParameters>,
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
    fn calculate(&self) -> Result<(), Error> {
        if self.pantocaren.len() < 2 {
            return Err(Error::FromString(
                "LeverDiagram calculate error: pantocaren.len() < 2".to_string(),
            ));
        }
        // Проверяем есть ли пантокарены в отрицательной области углов
        // Если нет то считаем что судно симметорично и зеркально
        // копируем данные в отрицательную область углов крена
        let mut pantocaren = self.pantocaren.clone();
        let mut tmp = pantocaren.first().unwrap().1.clone();
        tmp.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        if tmp.first().unwrap().0 <= 0. {
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
        let curve = Curve2D::from_values_catmull_rom(pantocaren)?;
        // расчет диаграммы
        //      log::info!("StabilityArm calculate mean_draught:{}, z_g_fix:{} ", self.mean_draught, self.metacentric_height.z_g_fix()?);
        let theta: &dyn Fn(i32) -> Result<(f64, f64), Error> = &|angle_deg: i32| {
            let angle_deg = angle_deg as f64 * 0.1;
            let angle_rad = angle_deg * std::f64::consts::PI / 180.;
            let v1 = curve.value(self.mean_draught, angle_deg)?;
            let v2 = self.metacentric_height.z_g_fix()? * angle_rad.sin();
            let v3 =
                (self.ship_moment.shift()?.y() - self.center_draught_shift.y()) * angle_rad.cos();
            let value = v1 - v2 - v3;
            //                log::info!("StabilityArm calculate расчет диаграммы: {angle_deg}, {angle_rad}, {v1}, {v2}, {v3}, {value}");
            Ok((angle_deg, value))
        };
        let mut dso = (-900..=900)
            .filter_map(|angle_deg| theta(angle_deg).ok())
            .collect::<Vec<(f64, f64)>>();
        // знак статического угла крена
        let mut angle_zero_signum = 1.;
        // если крен на левый борт то переворачиваем диаграмму
        if theta(0)?.1 > 0. {
            dso = dso.into_iter().map(|(a, v)| (-a, -v)).collect();
            dso.sort_by(|(a1, _), (a2, _)| {
                a1.partial_cmp(a2)
                    .expect("LeverDiagram calculate error: sort dso!")
            });
            angle_zero_signum = -1.; // сохраняем знак угла
                                     /*      log::info!("StabilityArm rotate dso:");
                                     for (angle, value) in dso.iter() {
                                         log::info!("angle:{angle} value:{value}");
                                     }*/
        }
        /*      log::info!("StabilityArm calculate dso:");
        for (angle, value) in dso.iter() {
            log::info!("angle:{angle} value:{value}");
        }*/
        // нахождение максимума диаграммы
        let mut tmp_dso: Vec<&(f64, f64)> = dso.iter().filter(|(a, _)| *a >= 0.).collect();
        tmp_dso.sort_by(|(_, v1), (_, v2)| {
            v2.partial_cmp(v1)
                .expect("LeverDiagram calculate error: sort dso!")
        });
        let curve = Curve::new_linear(&dso)?;
        let mut angle = tmp_dso
            .first()
            .expect("LeverDiagram calculate error, no dso values!")
            .0;
        let mut max_angle = angle;
        let mut value = curve.value(angle)?;
        let mut max_value = value;
        let mut delta_angle = 1.;
        for _i in 0..10 {
            let delta_angle_l = angle - delta_angle;
            let value_l = curve.value(delta_angle_l)?;
            let delta_angle_r = angle + delta_angle;
            let value_r = curve.value(delta_angle_r)?;
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
            //                log::info!("StabilityArm calculate: value:{value} angle:{angle} max_value:{max_value} max_angle:{max_angle} delta_angle:{delta_angle} i:{_i} ");
        }
        *self.theta_max.borrow_mut() = Some(max_angle);
        *self.dso.borrow_mut() = Some(dso.clone());
        *self.dso_curve.borrow_mut() = Some(curve.clone());
        // нахождение углов максимумов и угла пересечения с 0
        let mut max_angles: Vec<(f64, f64)> = Vec::new();
        let mut last_value = curve.value(0.)?;
        let mut last_value2 = last_value + 1.;
        let mut last_angle = 0.;
        for &(angle_deg, value) in dso.iter().filter(|(a, _)| *a >= 0.) {
            if value < last_value && last_value > last_value2 {
                //           dbg!(last_value2, last_value, value, last_angle, angle_deg);
                max_angles.push((last_angle, last_value));
            }
            if last_value != value {
                last_value2 = last_value;
                last_value = value;
                last_angle = angle_deg
            }
        }
        if max_angles.is_empty() {
            max_angles.push((max_angle, curve.value(max_angle)?));
        }
        *self.max_angles.borrow_mut() = Some(max_angles);
        //
        let angle_zero = *self.angle(0.)?.first().unwrap_or(&0.);
        let mut ddo = Vec::new();
        for &(angle_deg, _) in dso
            .iter()
            .filter(|(a, _)| *a >= 0. && a.fract().abs() < 0.001)
        {
            let value = if angle_deg < angle_zero {
                curve.integral(angle_deg, angle_zero)? * std::f64::consts::PI / 180.
            } else if angle_deg > angle_zero {
                curve.integral(angle_zero, angle_deg)? * std::f64::consts::PI / 180.
            } else {
                0.
            };
            ddo.push((angle_deg, value));
        }
        /*   log::info!("StabilityArm calculate ddo:");
        for &(angle, value) in ddo.iter() {
            log::info!("angle:{angle} value:{value}");
        }  */
        let diagram = dso
            .iter()
            .filter(|(a, _)| *a >= 0. && a.fract().abs() < 0.001)
            .zip(ddo.iter())
            .map(|((a1, v1), (_, v2))| (*a1, *v1, *v2))
            .collect::<Vec<_>>();
        /*     log::info!(
            "StabilityArm calculate z_g_fix:{} angle_zero:{} len dso:{} ddo:{} diagram:{}, [angle, dso, ddo]",
            self.metacentric_height.z_g_fix()?,
            angle_zero * angle_zero_signum,
            dso.len(),
            ddo.len(),
            diagram.len(),
        );*/
     /*   for &(angle, dso, ddo) in diagram.iter() {
            log::info!("{angle} {dso} {ddo};");
        }*/
        *self.diagram.borrow_mut() = Some(diagram);
        *self.ddo.borrow_mut() = Some(ddo);
        self.parameters
            .add(ParameterID::Roll, angle_zero * angle_zero_signum);
        Ok(())
    }
}
///
impl ILeverDiagram for LeverDiagram {
    /// Углы крена судна соответствующие плечу кренящего момента (angle >= 0. && angle <= 90.)
    fn angle(&self, lever_moment: f64) -> Result<Vec<f64>, Error> {
        if self.dso_curve.borrow().is_none() {
            self.calculate()?;
        }
        let binding = self.dso_curve.borrow();
        let curve = binding.as_ref().ok_or(Error::FromString(
            "LeverDiagram angle error: no dso_curve!".to_string(),
        ))?;
        let max_angle = self.theta_max.borrow().ok_or(Error::FromString(
            "StabilityArm angle error: no max_angle!".to_string(),
        ))?;
        if curve.value(max_angle)? < lever_moment {
            return Err(Error::FromString(
                "StabilityArm angle error: curve.value(max_angle) < lever_moment!".to_string(),
            ));
        }
        let mut delta_angle = 22.5;
        let mut angles = vec![max_angle - delta_angle, max_angle + delta_angle];
        for _i in 0..30 {
            let last_delta_value = lever_moment - curve.value(angles[0])?;
            //        log::info!("StabilityArm calculate: target:{lever_moment} angle1:{} last_delta_value:{last_delta_value} i:{_i} delta_angle:{delta_angle} ", angles[0]);
            if last_delta_value.abs() > 0.00001 {
                angles[0] += delta_angle * last_delta_value.signum();
            }
            let last_delta_value = lever_moment - curve.value(angles[1])?;
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
        Ok(angles)
    }
    /// Плечо кренящего момента соответствующие углу крена судна
    fn lever_moment(&self, angle: f64) -> Result<f64, Error> {
        if !(0. ..=90.).contains(&angle) {
            return Err(Error::FromString(format!(
                "FakeLeverDiagram lever_moment error: angle {angle} >= 0. && angle {angle} <= 90."
            )));
        }
        if self.dso_curve.borrow().is_none() {
            self.calculate()?;
        }
        self.dso_curve
            .borrow()
            .as_ref()
            .ok_or(Error::FromString(
                "LeverDiagram angle error: no dso_curve!".to_string(),
            ))?
            .value(angle)
    }
    /// Площадь под положительной частью диаграммы статической остойчивости (rad^2)
    fn dso_area(&self, angle1: f64, angle2: f64) -> Result<f64, Error> {
        if angle1 > angle2 {
            return Err(Error::FromString(format!(
                "FakeLeverDiagram dso_area error: angle1 {angle1} > angle2 {angle2}"
            )));
        }
        if self.dso_curve.borrow().is_none() {
            self.calculate()?;
        }
        Ok(self
            .dso_curve
            .borrow()
            .as_ref()
            .ok_or(Error::FromString(
                "LeverDiagram dso_area error: no dso_curve!".to_string(),
            ))?
            .integral(angle1, angle2)?
            * PI
            / 180.)
    }
    /// Максимальное плечо диаграммы статической остойчивости в диапазонеб (м)
    fn dso_lever_max(&self, angle1: f64, angle2: f64) -> Result<f64, Error> {
        if angle1 > angle2 {
            return Err(Error::FromString(format!(
                "FakeLeverDiagram dso_lever_max error: angle1 {angle1} > angle2 {angle2}"
            )));
        }
        if self.dso.borrow().is_none() {
            self.calculate()?;
        }
        let dso = self.dso.borrow();
        let mut segment = dso
            .as_ref()
            .ok_or(Error::FromString(
                "LeverDiagram dso_lever_max error: no dso!".to_string(),
            ))?
            .iter()
            .filter(|v| v.0 >= angle1 && v.0 <= angle2)
            .collect::<Vec<_>>();
        segment.sort_by(|v1, v2| {
            v1.1.partial_cmp(&v2.1)
                .expect("LeverDiagram dso_lever_max partial_cmp error!")
        });
        Ok(segment
            .last()
            .ok_or(Error::FromString(
                "LeverDiagram dso_lever_max segment error: no values!".to_string(),
            ))?
            .1)
    }
    /// Диаграммы остойчивости, зависимость от угла, градусы
    fn diagram(&self) -> Result<Vec<(f64, f64, f64)>, Error> {
        if self.diagram.borrow().is_none() {
            self.calculate()?;
        }
        self.diagram.borrow().clone().ok_or(Error::FromString(
            "StabilityArm diagram error: no diagram!".to_string(),
        ))
    }
    /// Углы максимумов диаграммы плеч статической остойчивости
    fn max_angles(&self) -> Result<Vec<(f64, f64)>, Error> {
        if self.max_angles.borrow().is_none() {
            self.calculate()?;
        }
        self.max_angles.borrow().clone().ok_or(Error::FromString(
            "StabilityArm max_angles error: no max_angles!".to_string(),
        ))
    }
}
#[doc(hidden)]
pub trait ILeverDiagram {
    /// Углы крена судна соответствующие плечу кренящего момента
    fn angle(&self, lever_moment: f64) -> Result<Vec<f64>, Error>;
    /// Плечо кренящего момента соответствующие углу крена судна (angle >= 0. && angle <= 90.)
    fn lever_moment(&self, angle: f64) -> Result<f64, Error>;
    /// Площадь под положительной частью диаграммы статической остойчивости (rad^2)
    fn dso_area(&self, angle1: f64, angle2: f64) -> Result<f64, Error>;
    /// Максимальное плечо диаграммы статической остойчивости в диапазоне (м)
    fn dso_lever_max(&self, angle1: f64, angle2: f64) -> Result<f64, Error>;
    /// Диаграммы остойчивости, зависимость от угла, градусы
    fn diagram(&self) -> Result<Vec<(f64, f64, f64)>, Error>;
    /// Углы и плечи максимумов диаграммы плеч статической остойчивости
    fn max_angles(&self) -> Result<Vec<(f64, f64)>, Error>;
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
    fn angle(&self, _: f64) -> Result<Vec<f64>, Error> {
        Ok(self.angle.clone())
    }
    /// Плечо кренящего момента соответствующие углу крена судна (angle >= 0. && angle <= 90.), м
    fn lever_moment(&self, _: f64) -> Result<f64, Error> {
        Ok(self.lever_moment)
    }
    /// Площадь под положительной частью диаграммы статической остойчивости, м*rad
    fn dso_area(&self, angle1: f64, angle2: f64) -> Result<f64, Error> {
        if angle1 > angle2 {
            return Err(Error::FromString(format!(
                "FakeLeverDiagram dso_area error: angle1 {angle1} > angle2 {angle2}"
            )));
        }
        Ok(self.dso_area)
    }
    /// Максимальное плечо диаграммы статической остойчивости в диапазоне, м
    fn dso_lever_max(&self, angle1: f64, angle2: f64) -> Result<f64, Error> {
        if angle1 > angle2 {
            return Err(Error::FromString(format!(
                "FakeLeverDiagram dso_lever_max error: angle1 {angle1} > angle2 {angle2}"
            )));
        }
        Ok(self.dso_lever_max)
    }
    /// Диаграммы остойчивости, зависимость от угла, градусы
    fn diagram(&self) -> Result<Vec<(f64, f64, f64)>, Error> {
        Ok(self.diagram.clone())
    }
    /// Углы и плечи максимумов диаграммы плеч статической остойчивости
    fn max_angles(&self) -> Result<Vec<(f64, f64)>, Error> {
        Ok(self.max_angles.clone())
    }
}
