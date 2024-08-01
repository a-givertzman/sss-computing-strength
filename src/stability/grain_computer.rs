//! Критерий крена от смещения зерна

use crate::{Error, IBulk, ILeverDiagram, IMass, IParameters, ParameterID, Position};
use core::f64;
use std::{f64::consts::PI, rc::Rc};

use super::{FakeMetacentricHeight, IMetacentricHeight, IShipMoment, LeverDiagram};

/// Критерий крена от смещения зерна
pub struct GrainComputer {
    /// Угол заливания отверстий
    flooding_angle: f64,
    /// Средняя осадка
    mean_draught: f64,
    /// Все навалочные смещаемые грузы судна
    loads_bulk: Rc<Vec<Rc<dyn IBulk>>>,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    mass: Rc<dyn IMass>,
    /// Диаграмма плеч статической и динамической остойчивости
    lever_diagram: Rc<dyn ILeverDiagram>,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    ship_moment: Rc<dyn IShipMoment>,
    /// Отстояние центра величины погруженной части судна
    center_draught_shift: Position,
    /// Кривая плечей остойчивости формы для разных осадок
    pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
    /// Угол крена от смещения зерна
    angle: Option<(f64, f64)>,
    /// Остаточная площадь между кривой кренящих и
    /// кривой восстанавливающих плеч
    area: Option<f64>,
}
///
impl GrainComputer {
    /// Основной конструктор
    /// * flooding_angle - Угол заливания отверстий
    /// * mean_draught - Средняя осадка
    /// * loads_bulk - Все навалочные смещаемые грузы судна
    /// * mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    /// * ship_moment - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * center_draught_shift - Отстояние центра величины погруженной части судна
    /// * pantocaren - Кривая плечей остойчивости формы для разных осадок
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        flooding_angle: f64,
        mean_draught: f64,
        loads_bulk: Rc<Vec<Rc<dyn IBulk>>>,
        mass: Rc<dyn IMass>,
        lever_diagram: Rc<dyn ILeverDiagram>,
        ship_moment: Rc<dyn IShipMoment>,
        center_draught_shift: Position,
        pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            flooding_angle,
            mean_draught,
            loads_bulk,
            mass,
            lever_diagram,
            ship_moment,
            center_draught_shift,
            pantocaren,
            parameters,
            angle: None,
            area: None,
        }
    }
    /// Расчет угла крена и остаточной площади между кривой кренящих и
    /// кривой восстанавливающих плеч
    fn calculate(&mut self, lever_diagram: Rc<dyn ILeverDiagram>) -> Result<(), Error> {
        let m_grain: f64 = self.loads_bulk.iter().map(|v| v.moment()).sum();
        let lambda_0 = m_grain / self.mass.sum()?;
        // Первая точка апроксимирующей прямой
        let first_point_ab = (0.0f64, lambda_0);
        // Вторая точка апроксимирующей прямой
        let second_point_ab = (40.0f64, 0.8 * lambda_0);
        // Изменение апроксимирующей прямой на один градус угла крена
        let delta_ab =
            (second_point_ab.1 - first_point_ab.1) / (second_point_ab.0 - first_point_ab.0);
        let precision = 0.1; // Точность определения пересечения в градусах
                             // Точка пересечения кривых. Проходим по кривой плечей и ищем точку пересечения как
                             // точку, в которой значение кривой плеч момента зерна меньше чем значение dso
                             // Если точка отсутствует (момент от зерна слишком большй) то принимаем
                             // за точку 90 градусов
        let max_i: f64 = 90. / precision;
        let max_i = max_i.ceil() as usize;
        let first_angle = (0..=max_i)
            .find(|i| {
                let i = *i as f64;
                // значение угла крена в текущей точке
                let angle = i * precision;
                // значение апроксимирующей прямой плеч момента зерна в текущей точке
                let lever_ab = delta_ab * angle;
                // значение восстанавливающего момента в текущей точке
                let lever_dso = lever_diagram.lever_moment(angle).unwrap_or(f64::MIN);
                lever_dso >= lever_ab
            })
            .unwrap_or((90. / precision) as usize) as f64
            * precision;
        // угол соответствующий максимальной разности между ординатами двух кривых
        let mut angles: Vec<(f64, f64)> = (0..=max_i)
            .map(|i| {
                let i = i as f64;
                // значение угла крена в текущей точке
                let angle = i * precision;
                // значение апроксимирующей прямой плеч момента зерна в текущей точке
                let lever_ab = delta_ab * angle;
                // значение восстанавливающего момента в текущей точке
                let lever_dso = lever_diagram.lever_moment(angle).unwrap_or(lever_ab);
                (angle, lever_dso - lever_ab)
            })
            .collect();
        angles.sort_by(|v1, v2| v1.1.partial_cmp(&v2.1).expect("Grain calculate cmp error"));
        let angle_delta_max = angles.last().unwrap_or(&(0., 0.)).0;
        let second_angle = self.flooding_angle.min(40.).min(angle_delta_max);
        self.angle = Some((first_angle, second_angle));
        // Площадь кривой восстанавливающих плеч
        let dso_area = lever_diagram.dso_area(first_angle, second_angle)?;
        // Площадь кривой кренящих плеч от смещения зерна
        let first_grain_lever = lever_diagram.lever_moment(first_angle)?;
        let second_grain_lever = delta_ab * second_angle;
        let grain_area =
            ((second_grain_lever - first_grain_lever) / 2.) * (second_angle - first_angle) * PI
                / 180.;
        let result_area = dso_area - grain_area;
        self.area = Some(result_area);
        //    log::info!("\t Grain area m_grain:{m_grain} lambda_0:{lambda_0} first_angle:{first_angle} angle_delta_max:{angle_delta_max}
        //        second_angle:{second_angle} dso_area:{dso_area} grain_area:{grain_area} result_area:{result_area}");
        self.parameters.add(
            ParameterID::HeelingMomentDueToTheTransverseShiftOfGrain,
            m_grain,
        );
        self.parameters.add(
            ParameterID::HeelingAngleWithMaximumDifference,
            angle_delta_max,
        );
        Ok(())
    }
}
///
impl IGrainComputer for GrainComputer {
    /// Расчет допустимого возвышения центра тяжести судна
    fn zg(&mut self, target: f64) -> Result<f64, Error> {
        let mut z_g_fix = 50.;
        for _i in 0..30 {
            self.calculate(Rc::new(LeverDiagram::new(
                Rc::clone(&self.ship_moment),
                self.center_draught_shift.clone(),
                self.pantocaren.clone(),
                self.mean_draught,
                Rc::new(FakeMetacentricHeight::new(0., 0., 0., z_g_fix)),
                Rc::clone(&self.parameters),
            )))?;
            let delta = self.area().unwrap_or(0.) - target;
            if delta <= 0.01 {
                break;
            }
            z_g_fix = z_g_fix * 0.5 * delta.signum();
        }
        self.angle = None;
        self.area = None;
        Ok(z_g_fix)
    }
    /// Расчетный и максимально допустимый углы крена от смещения зерна
    fn angle(&mut self) -> Result<(f64, f64), Error> {
        if self.angle.is_none() {
            self.calculate(Rc::clone(&self.lever_diagram))?;
        }
        self.angle
            .ok_or(Error::FromString("Grain angle error!".to_string()))
    }
    /// Остаточная площадь между кривой кренящих и
    /// кривой восстанавливающих плеч
    fn area(&mut self) -> Result<f64, Error> {
        if self.area.is_none() {
            self.calculate(Rc::clone(&self.lever_diagram))?;
        }
        self.area
            .ok_or(Error::FromString("Grain area error!".to_string()))
    }
}
#[doc(hidden)]
pub trait IGrainComputer {
    /// Расчет допустимого возвышения центров тяжести судна
    fn zg(&mut self, target: f64) -> Result<f64, Error>;
    /// Расчетный и максимально допустимый углы крена от смещения зерна
    fn angle(&mut self) -> Result<(f64, f64), Error>;
    /// Остаточная площадь между кривой кренящих и
    /// кривой восстанавливающих плеч
    fn area(&mut self) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeGrainComputer {
    zg: f64,
    angle: (f64, f64),
    area: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeGrainComputer {
    pub fn new(zg: f64, angle: (f64, f64), area: f64) -> Self {
        Self { zg, angle, area }
    }
}
#[doc(hidden)]
impl IGrainComputer for FakeGrainComputer {
    /// Расчет допустимого возвышения центров тяжести судна
    fn zg(&mut self, _: f64) -> Result<f64, Error> {
        Ok(self.zg)
    }
    ///
    fn angle(&mut self) -> Result<(f64, f64), Error> {
        Ok(self.angle)
    }
    ///
    fn area(&mut self) -> Result<f64, Error> {
        Ok(self.area)
    }
}
