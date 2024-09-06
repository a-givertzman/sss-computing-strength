//! Критерии проверки посадки судна

pub(crate) mod depth_forward;
pub(crate) mod load_line;
pub(crate) mod minimum_draft;
pub(crate) mod reserve_buoyncy;
pub(crate) mod screw;

pub use depth_forward::*;
pub use load_line::*;
pub use minimum_draft::*;
pub use reserve_buoyncy::*;
pub use screw::*;

use super::{CriterionData, CriterionID};
use crate::{
    data::structs::{NavigationArea, ShipType},
    trim::ITrim,
    Curve, Error, ICurve, 
};
use std::rc::Rc;

/// Критерии проверки посадки судна
pub struct CriterionDraught {
    /// Тип судна
    ship_type: ShipType,
    /// Дедвейт
    deadweight: f64,
    /// Расчет уровня заглубления для осадок судна
    load_line: LoadLine,
    /// Вычисление средней осадки и дифферента
    trim: Rc<dyn ITrim>,
    /// Минимально допустимый дифферент с точки зрения расчета вероятностного индекса деления на отсеки  
    trim_min: f64,
    /// Максимально допустимый дифферент с точки зрения расчета вероятностного индекса деления на отсеки  
    trim_max: f64,
    /// Высота на носовом перпендикуляре
    depth_forward: DepthAtForwardPerpendicular,
    /// Расчетная минимальная высота в носу
    bow_h_min: f64,
    /// Расчет уровня заглубления для винтов судна
    screw: Screw,
    /// Запас плавучести в носу
    reserve_buoyncy: ReserveBuoyncyInBow,
    /// Минимальная осадка
    minimum_draft: MinimumDraft,
}
///
impl CriterionDraught {
    /// Главный конструктор:
    /// * ship_type - Тип судна
    /// * deadweight -Дедвейт
    /// * load_line - Расчет уровня заглубления для осадок судна
    /// * trim - Cредняя осадка и дифферент
    /// * trim_min - Минимально допустимый дифферент с точки зрения расчета вероятностного индекса деления на отсеки  
    /// * trim_max - Максимально допустимый дифферент с точки зрения расчета вероятностного индекса деления на отсеки  
    /// * depth_forward - Высота на носовом перпендикуляре
    /// * bow_h_min - Расчетная минимальная высота в носу
    /// * screw - Расчет уровня заглубления для винтов судна
    /// * reserve_buoyncy - Запас плавучести в носу
    /// * minimum_draft - Минимальная осадка
    pub fn new(
        ship_type: ShipType,
        deadweight: f64,
        load_line: LoadLine,
        trim: Rc<dyn ITrim>,
        trim_min: f64,
        trim_max: f64,
        depth_forward: DepthAtForwardPerpendicular,
        bow_h_min: f64,
        screw: Screw,
        reserve_buoyncy: ReserveBuoyncyInBow,
        minimum_draft: MinimumDraft,
    ) -> Self {
        Self {
            ship_type,
            deadweight,
            load_line,
            trim,
            trim_min,
            trim_max,
            depth_forward,
            bow_h_min,
            screw,
            reserve_buoyncy,
            minimum_draft,
        }
    }
    ///
    pub fn create(&mut self) -> Vec<CriterionData> {
        let mut out_data = Vec::new();
        //        dbg!(self.metacentric_height.z_g_fix().unwrap());
        if self.navigation_area != NavigationArea::R3Rsn {
            out_data.push(self.weather());
        }
        if self.navigation_area != NavigationArea::R3Rsn {
            out_data.push(self.static_angle());
        }
        out_data.append(&mut self.dso());
        out_data.push(self.dso_lever());
        if self.have_timber {
            out_data.push(self.dso_lever_timber());
        }
        if self.navigation_area != NavigationArea::Unlimited && self.have_icing {
            out_data.push(self.dso_lever_icing());
        }
        out_data.append(&mut self.dso_lever_max_angle());
        //       if self.have_cargo {
        out_data.push(self.metacentric_height());
        //    }
        if let Ok(h_trans_fix) = self.metacentric_height.h_trans_fix() {
            if self.navigation_area == NavigationArea::R2Rsn
                || self.navigation_area == NavigationArea::R2Rsn45
                || h_trans_fix.sqrt() / self.breadth > 0.08
                || self.breadth / self.moulded_depth > 2.5
            {
                out_data.push(self.accelleration());
            }
        }
        if self.ship_type == ShipType::ContainerShip {
            out_data.push(self.circulation());
        }
        if self.have_grain {
            out_data.append(&mut self.grain());
        }
        out_data.push(self.metacentric_height_subdivision());
        out_data
    }
    /// Критерий погоды K
    pub fn weather(&mut self) -> CriterionData {
        let k = self.stability.k();
        match k {
            Ok(k) => CriterionData::new_result(CriterionID::Wheather, k, 1.),
            Err(error) => CriterionData::new_error(CriterionID::Wheather, error.to_string()),
        }
    }
    /// Статический угол крена от действия постоянного ветра.
    /// При расчете плеча кренящего момента от давления ветра 𝑙𝑤1, используемое при
    /// определении угла крена θ𝑤1, предполагаемое давление ветра 𝑝𝑣 принимается как для судна
    /// неограниченного района плавания судна.
    pub fn static_angle(&mut self) -> CriterionData {
        // Для всех судов (кроме района плавания R3):
        // статического угла крена θ𝑤1, вызванного постоянным ветром
        let wind_lever = match self.wind.arm_wind_static() {
            Ok(wind_lever) => wind_lever,
            Err(text) => {
                return CriterionData::new_error(
                    CriterionID::WindStaticHeel,
                    "Ошибка расчета кренящего момента постоянного ветра: ".to_owned()
                        + &text.to_string(),
                )
            }
        };
        let binding = match self.lever_diagram.angle(wind_lever) {
            Ok(binding) => binding,
            Err(text) => return CriterionData::new_error(
                CriterionID::WindStaticHeel,
                "Ошибка расчета угла крена судна соответствующего плечу кренящего момента постоянного ветра: ".to_owned() + &text.to_string(),
            ),
        };
        let angle = binding.first();
        let target_value = match self.ship_type {
            ShipType::TimberCarrier => 16.,
            ShipType::ContainerShip => 16.0f64.min(0.5 * self.flooding_angle),
            _ => 16.0f64.min(0.8 * self.flooding_angle),
        };
        if let Some(angle) = angle {
            CriterionData::new_result(CriterionID::WindStaticHeel, *angle, target_value)
        } else {
            CriterionData::new_error(
                CriterionID::WindStaticHeel,
                "Нет угла крена судна для текущих погодных условий".to_owned(),
            )
        }
    }
    /// Площади под диаграммой статической остойчивости
    pub fn dso(&self) -> Vec<CriterionData> {
        let mut results = Vec::new();
        let theta = self.lever_diagram.angle(0.).unwrap_or(vec![0., 0.]);
        let theta_0 = *theta.first().unwrap_or(&0.);
        let theta_max = *theta.last().unwrap_or(&0.);
        let second_angle_30 = theta_max.min(30.).min(self.flooding_angle);
        match self.lever_diagram.dso_area(theta_0, second_angle_30) {
            Ok(result) => results.push(CriterionData::new_result(
                CriterionID::AreaLC0_30,
                result,
                0.055,
            )),
            Err(text) => results.push(CriterionData::new_error(
                CriterionID::AreaLC0_30,
                "Ошибка расчета площади под положительной частью диаграммы статической остойчивости 0-30 градусов: ".to_owned() + &text.to_string(),
            )),
        };
        let second_angle_40 = theta_max.min(40.).min(self.flooding_angle);
        let target_area = if self.ship_type != ShipType::TimberCarrier {
            0.09
        } else {
            0.08
        };
        match self.lever_diagram.dso_area(theta_0, second_angle_40) {
            Ok(result) => results.push(CriterionData::new_result(
                CriterionID::AreaLC0_40,
                result,
                target_area,
            )),
            Err(text) => results.push(CriterionData::new_error(
                CriterionID::AreaLC0_40,
                "Ошибка расчета площади под положительной частью диаграммы статической остойчивости 0-40 градусов: ".to_owned() + &text.to_string(),
            )),
        };
        let first_angle_30 = theta_0.max(30.);
        match self.lever_diagram.dso_area(first_angle_30, second_angle_40) {
            Ok(result) => results.push(CriterionData::new_result(
                CriterionID::AreaLC30_40,
                result,
                0.03,
            )),
            Err(text) => results.push(CriterionData::new_error(
                CriterionID::AreaLC30_40,
                "Ошибка расчета площади под положительной частью диаграммы статической остойчивости 30-40 градусов: ".to_owned() + &text.to_string(),
            )),
        };
        //    log::info!("Criterion dso: zg:{} theta_0:{theta_0} theta_max:{theta_max} first_angle_30:{first_angle_30} second_angle_30:{second_angle_30} second_angle_40:{second_angle_40}", self.metacentric_height.z_g_fix().unwrap_or(-1.));
        results
    }
    /// Максимум диаграммы статической остойчивости
    pub fn dso_lever(&self) -> CriterionData {
        let curve = match Curve::new_linear(&vec![(105., 0.20), (80., 0.25)]) {
            Ok(curve) => curve,
            Err(text) => return CriterionData::new_error(
                CriterionID::MaximumLC,
                "Ошибка создания кривой в расчете максимума диаграммы статической остойчивости: "
                    .to_owned()
                    + &text.to_string(),
            ),
        };
        let target = match curve.value(self.ship_length) {
            Ok(value) => value,
            Err(text) => return CriterionData::new_error(
                CriterionID::MaximumLC,
                "Ошибка вычисления значения кривой в расчете максимума диаграммы статической остойчивости: ".to_owned() + &text.to_string(),
            ),
        };
        let result = match self.lever_diagram.dso_lever_max(30., 90.) {
            Ok(value) => value,
            Err(text) => return CriterionData::new_error(
                CriterionID::MaximumLC,
                "Ошибка вычисления максимального плеча диаграммы статической остойчивости в расчете максимума диаграммы статической остойчивости: ".to_owned() + &text.to_string(),
            ),
        };
        CriterionData::new_result(CriterionID::MaximumLC, result, target)
    }
    /// Максимум диаграммы статической остойчивости для лесовозов
    pub fn dso_lever_timber(&self) -> CriterionData {
        let target = 0.25;
        let result = match self.lever_diagram.dso_lever_max(0., 90.) {
            Ok(value) => value,
            Err(text) => return CriterionData::new_error(
                CriterionID::MaximumLcTimber,
                "Ошибка вычисления максимального плеча диаграммы статической остойчивости в расчете максимума диаграммы статической остойчивости для лесовозов: ".to_owned() + &text.to_string(),
            ),
        };
        CriterionData::new_result(CriterionID::MaximumLcTimber, result, target)
    }
    /// Максимум диаграммы статической остойчивости с учетом обледенения
    pub fn dso_lever_icing(&self) -> CriterionData {
        let target = 0.20;
        let result = match self.lever_diagram.dso_lever_max(25., 90.) {
            Ok(value) => value,
            Err(text) => return CriterionData::new_error(
                CriterionID::MaximumLcIcing,
                "Ошибка вычисления максимального плеча диаграммы статической остойчивости в расчете максимума диаграммы статической остойчивости с учетом обледенения: ".to_owned() + &text.to_string(),
            ),
        };
        CriterionData::new_result(CriterionID::MaximumLcIcing, result, target)
    }
    /// Угол, соответствующий максимуму диаграммы статической остойчивости
    pub fn dso_lever_max_angle(&self) -> Vec<CriterionData> {
        let mut results = Vec::new();
        let angles = match self.lever_diagram.max_angles() {
            Ok(angles) => angles,
            Err(text) => {
                results.push(CriterionData::new_error(
                    CriterionID::HeelMaximumLC,
                    "Ошибка вычисления угла и плеча максимума диаграммы плеч статической остойчивости в расчете метацентрической высоты: ".to_owned() + &text.to_string(),
                ));
                return results;
            }
        };
        let b_div_d = self.breadth / self.moulded_depth;
        let mut target = 30.;
        if b_div_d > 2. {
            let k = match self.stability.k() {
                Ok(k) => k,
                Err(error) => {
                    results.push(CriterionData::new_error(
                        CriterionID::HeelMaximumLC,
                        error.to_string(),
                    ));
                    return results;
                }
            };
            target -= (40. * (b_div_d.min(2.5) - 2.) * (k.min(1.5) - 1.) * 0.5).round();
        }
        if let Some(angle) = angles.first() {
            if b_div_d > 2.5 {
                target = 15.;
                match self.metacentric_height.h_trans_fix() {
                    Ok(src_area) => {
                        let target_area = if angle.0 <= 15.0 {
                            0.07
                        } else if angle.0 >= 30.0 {
                            0.055
                        } else {
                            0.05 + 0.001 * (30.0 - angle.0)
                        };
                        results.push(CriterionData::new_result(
                            CriterionID::AreaLc0Thetalmax,
                            src_area,
                            target_area,
                        ));
                    },
                    Err(text) => results.push(CriterionData::new_error(
                        CriterionID::AreaLc0Thetalmax,
                        "Ошибка вычисления поперечной исправленной метацентрической высоты в расчете угла, соответствующего максимуму диаграммы статической остойчивости: ".to_owned() + &text.to_string(),
                    )),
                };
            } else if angles.len() > 1 {
                //        dbg!(&self.metacentric_height.z_g_fix(), &angles);
                results.push(CriterionData::new_result(
                    CriterionID::HeelFirstMaximumLC,
                    angle.0,
                    25.,
                ));
            }
            results.push(CriterionData::new_result(
                CriterionID::HeelMaximumLC,
                angle.0,
                target,
            ));
        } else {
            results.push(CriterionData::new_error(
                CriterionID::HeelMaximumLC,
                "Нет угла соответствующего максимуму DSO для текущих условий".to_owned(),
            ));
        }
        results
    }
    /// Метацентрическая высота
    pub fn metacentric_height(&self) -> CriterionData {
        // Все суда
        let target = if self.have_grain {
            0.3
        } else if self.ship_type == ShipType::RoRo {
            0.2
        } else if self.have_timber {
            0.1
        } else {
            0.15
        };
        let result = match self.metacentric_height.h_trans_fix() {
            Ok(value) => value,
            Err(text) => return CriterionData::new_error(
                CriterionID::MinMetacentricHight,
                "Ошибка вычисления поперечной исправленной метацентрической высоты в расчете метацентрической высоты: ".to_owned() + &text.to_string(),
            ),
        };
        CriterionData::new_result(CriterionID::MinMetacentricHight, result, target)
    }
    /// Критерий ускорения 𝐾∗
    pub fn accelleration(&self) -> CriterionData {
        let result = match self.acceleration.calculate() {
            Ok(value) => value,
            Err(text) => {
                return CriterionData::new_error(
                    CriterionID::Acceleration,
                    "Ошибка вычисления критерия ускорения: ".to_owned() + &text.to_string(),
                )
            }
        };
        CriterionData::new_result(CriterionID::Acceleration, result, 1.)
    }
    /// Критерий крена на циркуляции
    pub fn circulation(&self) -> CriterionData {
        let target = 16.0f64.min(self.flooding_angle / 2.);
        let angle = match self.circulation.angle() {
            Ok(value) => value,
            Err(text) => {
                return CriterionData::new_error(
                    CriterionID::HeelTurning,
                    "Ошибка вычисления крена на циркуляции: ".to_owned() + &text.to_string(),
                )
            }
        };
        if let Some(angle) = angle {
            CriterionData::new_result(CriterionID::HeelTurning, angle, target)
        } else {
            return match self.circulation.velocity(target) {
                Ok(velocity) => CriterionData::new_error(
                    CriterionID::HeelTurning,
                    format!(
                        "Крен {target} градусов, рекомендуемая скорость {} m/s');",
                        velocity,
                    ),
                ),
                Err(text) => {
                    return CriterionData::new_error(
                        CriterionID::HeelTurning,
                        "Ошибка вычисления рекомендуемой скорости: ".to_owned() + &text.to_string(),
                    )
                }
            };
        }
        // TODO: В случаях, когда палубный груз контейнеров размещается только на крышках грузовых
        // люков, вместо угла входа кромки верхней палубы может приниматься меньший из углов
        // входа в воду верхней кромки комингса люка или входа контейнера в воду (в случае, когда
        // контейнеры выходят за пределы этого комингса).
    }
    /// Критерий при перевозки навалочных смещаемых грузов
    pub fn grain(&mut self) -> Vec<CriterionData> {
        let mut results = Vec::new();
        match self.grain.angle() {
            Ok((angle1, angle2)) => results.push(CriterionData::new_result(
                CriterionID::HeelGrainDisplacement,
                angle1,
                angle2,
            )),
            Err(text) => results.push(CriterionData::new_error(
                CriterionID::HeelGrainDisplacement,
                "Ошибка вычисления расчетного и максимально допустимого угла крена от смещения зерна крена: ".to_owned() + &text.to_string(),
            )),
        };
        if let Ok(area) = self.grain.area() {
            results.push(CriterionData::new_result(
                CriterionID::AreaLcGrainDisplacement,
                area,
                0.075,
            ));
        }
        results
    }
    /// Метацентрическая высота
    pub fn metacentric_height_subdivision(&self) -> CriterionData {
        // Все суда
        let result = match self.metacentric_height.h_trans_fix() {
            Ok(value) => value,
            Err(text) => {
                return CriterionData::new_error(
                    CriterionID::MinMetacentricHeightSubdivIndex,
                    "Ошибка вычисления поперечной исправленной метацентрической высоты: "
                        .to_owned()
                        + &text.to_string(),
                )
            }
        };
        CriterionData::new_result(
            CriterionID::MinMetacentricHeightSubdivIndex,
            result,
            self.h_subdivision,
        )
    }
}
