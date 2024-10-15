//! Критерии проверки остойчивости судна
pub(crate) mod acceleration;
pub(crate) mod circulation;
pub(crate) mod grain;
pub(crate) mod stab;

pub use acceleration::*;
pub use circulation::*;
pub use grain::*;
use log::info;
pub use stab::*;
use std::rc::Rc;

use crate::{
    data::structs::{NavigationArea, ShipType},
    Curve, Error, ICurve, ILeverDiagram, IMetacentricHeight, IWind,
};

use super::{CriterionData, CriterionID};
/// Критерии проверки остойчивости судна
pub struct CriterionStability {
    /// Тип судна
    ship_type: ShipType,
    /// Район плавания судна
    navigation_area: NavigationArea,
    /// Признак наличия леса
    have_timber: bool,
    /// Признак наличия сыпучего груза
    have_grain: bool,
    /// Признак наличия груза или
    #[allow(unused)]
    have_cargo: bool,
    /// Признак учета обледенения
    have_icing: bool,
    /// Угол заливания отверстий
    flooding_angle: f64,
    /// Длина судна
    ship_length: f64,
    /// Ширина судна
    breadth: f64,
    /// Высота борта, м
    moulded_depth: f64,
    /// Минимальная допустимая метацентрическая высота деления на отсеки
    h_subdivision: f64,
    /// Статический угол крена от действия постоянного ветра.
    /// Предполагаемое давление ветра 𝑝𝑣 принимается как для судна
    /// неограниченного района плавания судна.
    wind: Rc<dyn IWind>,
    /// Диаграмма плеч статической и динамической остойчивости
    lever_diagram: Rc<dyn ILeverDiagram>,
    /// Критерий погоды K
    stability: Rc<dyn IStability>,
    /// Продольная и поперечная исправленная метацентрическая высота
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Расчет критерия ускорения
    acceleration: Rc<dyn IAcceleration>,
    /// Расчет крена на циркуляции
    circulation: Rc<dyn ICirculation>,
    /// Смещение груза при перевозки навалочных смещаемых грузов (зерна)
    grain: Box<dyn IGrain>,
}
//
impl CriterionStability {
    /// Главный конструктор:
    /// * ship_type - Тип судна
    /// * navigation_area - Район плавания судна
    /// * breadth - Ширина судна
    /// * moulded_depth - Высота борта, м
    /// * h_subdivision - Минимальная допустимая метацентрическая высота деления на отсеки
    /// * navigation_area - Район плавания судна
    /// * have_timber - Признак наличия леса
    /// * have_grain - Признак наличия сыпучего груза
    /// * have_cargo - Признак наличия груза или балласта
    /// * have_icing - Признак учета обледенения
    /// * flooding_angle - Угол заливания отверстий
    /// * ship_length - Длина судна
    /// * wind - Статический угол крена от действия постоянного ветра
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    /// * stability - Критерий погоды K
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота
    /// * acceleration - Расчет критерия ускорения
    /// * circulation - Расчет крена на циркуляции
    /// * grain - Смещение груза при перевозки навалочных смещаемых грузов (зерна)
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ship_type: ShipType,
        navigation_area: NavigationArea,
        breadth: f64,
        moulded_depth: f64,
        h_subdivision: f64,
        have_timber: bool,
        have_grain: bool,
        have_cargo: bool,
        have_icing: bool,
        flooding_angle: f64,
        ship_length: f64,
        wind: Rc<dyn IWind>,
        lever_diagram: Rc<dyn ILeverDiagram>,
        stability: Rc<dyn IStability>,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        acceleration: Rc<dyn IAcceleration>,
        circulation: Rc<dyn ICirculation>,
        grain: Box<dyn IGrain>,
    ) -> Result<Self, Error> {
        if moulded_depth <= 0. {
            let error = Error::FromString("Criterion new error: moulded_depth <= 0.".to_owned());
            log::error!("{error}");
            return Err(error);
        }
        Ok(Self {
            ship_type,
            navigation_area,
            have_timber,
            have_grain,
            have_cargo,
            have_icing,
            flooding_angle,
            ship_length,
            breadth,
            moulded_depth,
            h_subdivision,
            wind,
            stability,
            lever_diagram,
            metacentric_height,
            acceleration,
            circulation,
            grain,
        })
    }
    //
    pub fn create(&mut self) -> Vec<CriterionData> {
        info!("Criterion begin");
        let mut out_data = Vec::new();
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
        info!("Criterion end");
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
        let curve = match Curve::new_linear(&[(105., 0.20), (80., 0.25)]) {
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
            match self.circulation.velocity(target) {
                Ok(velocity) => CriterionData::new_error(
                    CriterionID::HeelTurning,
                    format!(
                        "Крен {target} градусов, рекомендуемая скорость {} m/s');",
                        velocity,
                    ),
                ),
                Err(text) => {
                    CriterionData::new_error(
                        CriterionID::HeelTurning,
                        "Ошибка вычисления рекомендуемой скорости: ".to_owned() + &text.to_string(),
                    )
                }
            }
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
