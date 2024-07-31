//! Расчет критерия погоды К

use std::{f64::consts::PI, rc::Rc};

use crate::{Error, IParameters, ParameterID, Position};

use super::{
    lever_diagram::ILeverDiagram, rolling_amplitude::IRollingAmplitude, wind::IWind, FakeLeverDiagram, FakeMetacentricHeight, IMetacentricHeight, IShipMoment, IStability, LeverDiagram, Stability
};

/// Расчет критерия погоды К и допустимого возвышения центра тяжести судна
pub struct StabilityComputer {
    /// Угол заливания отверстий
    flooding_angle: f64,
    /// Средняя осадка
    mean_draught: f64,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    ship_moment: Rc<dyn IShipMoment>,
    /// Отстояние центра величины погруженной части судна
    center_draught_shift: Position,
    /// Кривая плечей остойчивости формы для разных осадок
    pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    rolling_amplitude: Rc<dyn IRollingAmplitude>,
    /// Расчет плеча кренящего момента от давления ветра
    wind: Rc<dyn IWind>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
///
impl StabilityComputer {
    /// Конструктор по умолчанию.
    /// * flooding_angle - Угол заливания отверстий
    /// * mean_draught - Средняя осадка
    /// * ship_moment - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * center_draught_shift - Отстояние центра величины погруженной части судна
    /// * pantocaren - Кривая плечей остойчивости формы для разных осадок
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    /// * rolling_amplitude - Амплитуда качки судна с круглой скулой (2.1.5)
    /// * wind - Расчет плеча кренящего момента от давления ветра
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        flooding_angle: f64,
        mean_draught: f64,
        ship_moment: Rc<dyn IShipMoment>,
        center_draught_shift: Position,
        pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        rolling_amplitude: Rc<dyn IRollingAmplitude>,
        wind: Rc<dyn IWind>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            flooding_angle,
            mean_draught,
            ship_moment,
            center_draught_shift,
            pantocaren,
            metacentric_height,
            rolling_amplitude,
            wind,
            parameters,
        }
    }
    /// Расчет критерия погоды К (2.1.2)
    fn k(&self, lever_diagram: Rc<dyn ILeverDiagram>) -> Result<f64, Error> {
        Stability::new(
            self.flooding_angle,
            Rc::clone(&lever_diagram),
            Rc::clone(&self.rolling_amplitude),
            Rc::clone(&self.wind),
            Rc::clone(&self.parameters),
        ).k()
    }
}
///
impl IStabilityComputer for StabilityComputer {
    /// Расчет допустимого возвышения центра тяжести судна
    fn zg(&self, target_k: f64) -> Result<f64, Error> {
        let mut z_g_fix = 50.;
        for _i in 0..30 {
            let delta_k = self.k(Rc::new(LeverDiagram::new(
                Rc::clone(&self.ship_moment),
                self.center_draught_shift.clone(),
                self.pantocaren.clone(),
                self.mean_draught,
                Rc::new(FakeMetacentricHeight::new(           
                    0.,
                    0.,
                    0.,
                    z_g_fix,
                )),
                Rc::clone(&self.parameters),
            ))).unwrap_or(0.) - target_k;
            if delta_k <= 0.01 {
                break;
            }
            z_g_fix = z_g_fix*0.5*delta_k.signum();
        }
        Ok(z_g_fix)
    }
    /// Расчет критерия погоды К (2.1.2)
    fn k(&self) -> Result<f64, Error> {
        self.k( Rc::new(LeverDiagram::new(
                Rc::clone(&self.ship_moment),
                self.center_draught_shift.clone(),
                self.pantocaren.clone(),
                self.mean_draught,
                Rc::clone(&self.metacentric_height),
                Rc::clone(&self.parameters),
        )))
    }
}
#[doc(hidden)]
pub trait IStabilityComputer {
    /// Расчет допустимого возвышения центров тяжести судна
    fn zg(&self, target_k: f64) -> Result<f64, Error>;
    /// Расчет критерия погоды К (2.1.2)
    fn k(&self) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeStabilityComputer {
    zg: Option<f64>, 
    k: Option<f64>,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeStabilityComputer {
    pub fn new(zg: Option<f64>, k: Option<f64>) -> Self {
        Self { zg, k }
    }
}
#[doc(hidden)]
impl IStabilityComputer for FakeStabilityComputer {
    /// Расчет допустимого возвышения центров тяжести судна
    fn zg(&self, _: f64) -> Result<f64, Error> {
        self.zg.ok_or(Error::FromString("Some error!".to_string()))
    }
    /// Расчет критерия погоды К (2.1.2)
    fn k(&self) -> Result<f64, Error> {
        self.k.ok_or(Error::FromString("Some error!".to_string()))
    }
}
