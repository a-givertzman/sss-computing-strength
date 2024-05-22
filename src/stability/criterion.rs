//! Критерии проверки остойчивости

use std::rc::Rc;

use crate::{
    data::structs::{NavigationArea, ShipType},
    Curve, IAcceleration, ICirculation, ICurve, IGrain, ILeverDiagram, IMetacentricHeight,
    IStability, IWind,
};
/// 
enum CriterionID {
    CriterionWheather = 1,
    CriterionWindStaticHeel = 2,
    CriterionAreaLC0_30 = 3,
    CriterionAreaLC0_40 = 4,
    CriterionAreaLC30_40 = 5,
    CriterionMaximumLC = 6,
    CriterionMaximumLcTimber = 7,
    CriterionMaximumLcIcing = 8,
    CriterionHeelMaximumLC = 9,
    CriterionMetacentricHight = 10,
    CriterionAcceleration = 11,
    CriterionHeelTurning = 12,
    CriterionHeelGrainDisplacement = 13,
    CriterionAreaLcGrainDisplacement = 14,
}
///
impl From<CriterionID> for usize {
    fn from(criterion_id: CriterionID) -> Self {
        match criterion_id {
            CriterionID::CriterionWheather => 1,
            CriterionID::CriterionWindStaticHeel => 2,
            CriterionID::CriterionAreaLC0_30 => 3,
            CriterionID::CriterionAreaLC0_40 => 4,
            CriterionID::CriterionAreaLC30_40 => 5,
            CriterionID::CriterionMaximumLC => 6,
            CriterionID::CriterionMaximumLcTimber => 7,
            CriterionID::CriterionMaximumLcIcing => 8,
            CriterionID::CriterionHeelMaximumLC => 9,
            CriterionID::CriterionMetacentricHight => 10,
            CriterionID::CriterionAcceleration => 11,
            CriterionID::CriterionHeelTurning => 12,
            CriterionID::CriterionHeelGrainDisplacement => 13,
            CriterionID::CriterionAreaLcGrainDisplacement => 14,
        }
    }
}
/// Результат проверки критерия
pub struct CriterionData {
    /// id критерия
    pub criterion_id: usize,
    /// Результат расчета
    pub result: f64,
    /// Пороговое значение критерия
    pub target: f64,
    /// Текст ошибки
    pub error_message: Option<String>,
}
///
impl CriterionData {
    /// Конструктор при наличии результата
    pub fn new_result(criterion_id: CriterionID, result: f64, target: f64) -> Self {
        Self {
            criterion_id: criterion_id.into(),
            result,
            target,
            error_message: None,
        }
    }
    /// Конструктор при наличии ошибке расчета
    pub fn new_error(criterion_id: CriterionID, error_message: String) -> Self {
        Self {
            criterion_id: criterion_id.into(),
            result: 0.,
            target: 0.,
            error_message: Some(error_message),
        }
    }
}
/// Критерии проверки остойчивости
pub struct Criterion {
    /// Тип судна
    ship_type: ShipType,
    /// Район плавания судна
    navigation_area: NavigationArea,
    /// Признак наличия леса
    have_timber: bool,
    /// Признак наличия сыпучего груза
    have_grain: bool,
    /// Признак наличия груза или балласта
    have_cargo: bool,
    /// Угол заливания отверстий
    flooding_angle: f64,
    /// Длина судна
    ship_length: f64,
    /// Ширина судна
    breadth: f64,
    /// Средняя осадка
    mean_draught: f64,
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
///
impl Criterion {
    /// Главный конструктор:
    /// * ship_type - Тип судна
    /// * breadth - Ширина судна
    /// * mean_draught - Средняя осадка
    /// * navigation_area - Район плавания судна
    /// * have_timber - Признак наличия леса
    /// * have_grain - Признак наличия сыпучего груза
    /// * have_cargo - Признак наличия груза или балласта
    /// * flooding_angle - Угол заливания отверстий
    /// * ship_length - Длина судна
    /// * wind - Статический угол крена от действия постоянного ветра
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    /// * stability - Критерий погоды K
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота
    /// * acceleration - Расчет критерия ускорения
    /// * circulation - Расчет крена на циркуляции
    /// * grain - Смещение груза при перевозки навалочных смещаемых грузов (зерна)
    pub fn new(
        ship_type: ShipType,
        navigation_area: NavigationArea,
        have_timber: bool,
        have_grain: bool,
        have_cargo: bool,
        flooding_angle: f64,
        ship_length: f64,
        breadth: f64,
        mean_draught: f64,
        wind: Rc<dyn IWind>,
        lever_diagram: Rc<dyn ILeverDiagram>,
        stability: Rc<dyn IStability>,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        acceleration: Rc<dyn IAcceleration>,
        circulation: Rc<dyn ICirculation>,
        grain: Box<dyn IGrain>,
    ) -> Self {
        assert!(mean_draught > 0., "mean_draught {mean_draught} > 0.");
        Self {
            ship_type,
            navigation_area,
            have_timber,
            have_grain,
            have_cargo,
            flooding_angle,
            ship_length,
            breadth,
            mean_draught,
            wind,
            stability,
            lever_diagram,
            metacentric_height,
            acceleration,
            circulation,
            grain,
        }
    }
    ///
    pub fn create(&mut self) -> Vec<CriterionData> {
        let mut out_data = Vec::new();
        if self.navigation_area != NavigationArea::R3Rsn {
            out_data.push(self.weather());
        }
        if self.navigation_area != NavigationArea::R3Rsn {
            out_data.push(self.static_angle());
        }
        out_data.append(&mut self.dso());
        out_data.push(self.dso_lever());
        out_data.append(&mut self.dso_lever_max_angle());
        if self.have_cargo {
            out_data.push(self.metacentric_height());
        }
        if self.navigation_area == NavigationArea::R2Rsn
            || self.navigation_area == NavigationArea::R2Rsn45
            || self.metacentric_height.h_trans_fix().sqrt() / self.breadth > 0.08
            || self.breadth / self.mean_draught > 2.5
        {
            out_data.push(self.accelleration());
        }
        if self.ship_type == ShipType::ContainerShip {
            out_data.push(self.circulation());
        }
        if self.have_grain {
            out_data.append(&mut self.grain());
        }
        out_data
    }
    /// Критерий погоды K
    pub fn weather(&mut self) -> CriterionData {
        let k = self.stability.k();
        match k {
            Ok(k) => CriterionData::new_result(CriterionID::CriterionWheather, k, 1.),
            Err(error) => CriterionData::new_error(CriterionID::CriterionWheather, error.to_string()),
        }
    }
    /// Статический угол крена от действия постоянного ветра.
    /// При расчете плеча кренящего момента от давления ветра 𝑙𝑤1, используемое при
    /// определении угла крена θ𝑤1, предполагаемое давление ветра 𝑝𝑣 принимается как для судна
    /// неограниченного района плавания судна.
    pub fn static_angle(&mut self) -> CriterionData {
        // Для всех судов (кроме района плавания R3):
        // статического угла крена θ𝑤1, вызванного постоянным ветром
        let wind_lever = self.wind.arm_wind_static();
        let binding = self.lever_diagram.angle(wind_lever);
        let angle = binding.first();
        let target_value = match self.ship_type {
            ShipType::TimberCarrier => 16.,
            ShipType::ContainerShip => 16.0f64.min(0.5 * self.flooding_angle),
            _ => 16.0f64.min(0.8 * self.flooding_angle),
        };
        return if let Some(angle) = angle {
            CriterionData::new_result(CriterionID::CriterionWindStaticHeel, *angle, target_value)
        } else {
            CriterionData::new_error(CriterionID::CriterionWindStaticHeel, "Нет угла крена для текущих условий".to_owned())
        };
    }
    /// Площади под диаграммой статической остойчивости
    pub fn dso(&self) -> Vec<CriterionData> {
        let mut result = Vec::new();
        result.push(CriterionData::new_result(
            CriterionID::CriterionAreaLC0_30,
            self.lever_diagram.dso_area(0., 30.),
            0.055,
        ));
        let second_angle_40 = 40.0f64.min(self.flooding_angle);
        let target_area = if self.ship_type != ShipType::TimberCarrier {
            0.09
        } else {
            0.08
        };
        result.push(CriterionData::new_result(
            CriterionID::CriterionAreaLC0_40,
            self.lever_diagram.dso_area(0., second_angle_40),
            target_area,
        ));
        result.push(CriterionData::new_result(
            CriterionID::CriterionAreaLC30_40,
            self.lever_diagram.dso_area(30., second_angle_40),
            0.03,
        ));
        result
    }
    /// Максимум диаграммы статической остойчивости
    pub fn dso_lever(&self) -> CriterionData {
        if !self.have_timber {
            let curve = Curve::new_linear(&vec![(105., 0.25), (80., 0.20)]);
            CriterionData::new_result(
                CriterionID::CriterionMaximumLC,                
                self.lever_diagram.lever_moment(30.),
                curve.value(self.ship_length),
            )
        } else {
            if let Some(angle) = self.lever_diagram.max_angles().first() {
                CriterionData::new_result(CriterionID::CriterionMaximumLC, angle.1, 0.25)
            } else {
                CriterionData::new_error(
                    CriterionID::CriterionMaximumLC,
                    "Нет плеча соответствующего максимуму DSO для текущих условий".to_owned(),
                )
            }
        }
    }
    /// Угол, соответствующий максимуму диаграммы статической остойчивости
    pub fn dso_lever_max_angle(&self) -> Vec<CriterionData> {
        let mut result = Vec::new();
        let angles = self.lever_diagram.max_angles();
        let b_div_d = self.breadth / self.mean_draught;
        let target = if b_div_d <= 2. {
            if angles.len() > 1 {
                25.
            } else {
                30.
            }
        } else {
            let k = match self.stability.k() {
                Ok(k) => k,
                Err(error) => {
                    result.push(CriterionData::new_error(CriterionID::CriterionHeelMaximumLC, error.to_string()));
                    return result;
                }
            };
            (40. * (b_div_d.min(2.5) - 2.) * (k.min(1.5) - 1.) * 0.5).round()
        };

        if let Some(angle) = angles.first() {
            result.push(CriterionData::new_result(CriterionID::CriterionHeelMaximumLC, angle.0, target));
            /*TODO 2.2.3  if b_div_d > 2.5 && angle.0 < target {
                let src_area = self.lever_diagram.dso_area(0., angle.0);
                let target_area = if angle.0 <= 15.0 {
                    0.07
                } else if angle.0 >= 30.0 {
                    0.055
                } else {
                    0.05 + 0.001 * (30.0 - angle.0)
                };
                result.push(
                    format!(
                    "INSERT INTO result_stability
                            (value1, value2, unit)
                        VALUES
                            ('Площадь DSO до угла макс.', {src_area}, {target_area}, '>=', 'm*rad');"
                ));
            }*/
        } else {
            result.push(CriterionData::new_error(
                CriterionID::CriterionHeelMaximumLC,
                "Нет угла соответствующего максимуму DSO для текущих условий".to_owned(),
            ));
        }
        result
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
        CriterionData::new_result(
            CriterionID::CriterionMetacentricHight,
            self.metacentric_height.h_trans_fix(),
            target,
        )
    }
    /// Критерий ускорения 𝐾∗
    pub fn accelleration(&self) -> CriterionData {
        CriterionData::new_result(
            CriterionID::CriterionAcceleration,
            self.acceleration.calculate(),
            1.,
        )
    }
    /// Критерий крена на циркуляции
    pub fn circulation(&self) -> CriterionData {
        let target = 16.0f64.min(self.flooding_angle / 2.);
        if let Some(angle) = self.circulation.angle() {
            CriterionData::new_result(
                CriterionID::CriterionHeelTurning,
                angle,
                target,
            )
        } else {
            CriterionData::new_error(
                CriterionID::CriterionHeelTurning,
                format!(
                    "Крен {target} градусов, рекомендуемая скорость {} m/s');",
                        self.circulation.velocity(target),
                ),
            )
        }

        // TODO: В случаях, когда палубный груз контейнеров размещается только на крышках грузовых
        // люков, вместо угла входа кромки верхней палубы может приниматься меньший из углов
        // входа в воду верхней кромки комингса люка или входа контейнера в воду (в случае, когда
        // контейнеры выходят за пределы этого комингса).
    }
    /// Критерий при перевозки навалочных смещаемых грузов
    pub fn grain(&mut self) -> Vec<CriterionData> {
        let mut result = Vec::new();
        let (angle1, angle2) = self.grain.angle(); 
        result.push(CriterionData::new_result(
            CriterionID::CriterionHeelGrainDisplacement,
            angle1,
            angle2,
        ));
        if let Some(area) = self.grain.area() {
            result.push(CriterionData::new_result(
                CriterionID::CriterionAreaLcGrainDisplacement,
                area,
                0.075,
            ));
        }
        result
    }
}
