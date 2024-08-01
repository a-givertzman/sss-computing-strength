//! Расчет критерия погоды К

use std::rc::Rc;

use crate::{Curve, Error, ICurve, IParameters, Position};

use super::{
    lever_diagram::ILeverDiagram, rolling_amplitude::IRollingAmplitude, wind::IWind,
    FakeMetacentricHeight, IMetacentricHeight, IShipMoment, IStability, LeverDiagram,
    RollingAmplitude, RollingPeriod, Stability,
};

/// Расчет критерия погоды К и допустимого возвышения центра тяжести судна
pub struct StabilityComputer {
    /// Угол заливания отверстий
    flooding_angle: f64,
    /// Средняя осадка
    mean_draught: f64,
    /// Объемное водоизмещение
    volume: f64,
    /// Длина судна по ватерлинии при текущей осадке
    length_wl: f64,
    /// Ширина судна полная
    width: f64,
    /// Ширина судна по ватерлинии ватерлинии при текущей осадке
    breadth_wl: f64,
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
    /// Коэффициент k для судов, имеющих скуловые кили или
    /// брусковый киль. Табл. 2.1.5.2
    coefficient_k: Rc<dyn ICurve>,
    /// Безразмерный множитель Х_1 Табл. 2.1.5.1-1
    multipler_x1: Rc<dyn ICurve>,
    /// Безразмерный множитель Х_2 Табл. 2.1.5.1-2
    multipler_x2: Rc<dyn ICurve>,
    /// Безразмерный множитель S Табл. 2.1.5.1-3
    multipler_s_area: Rc<dyn ICurve>,
    /// Суммарная габаритная площадь скуловых килей
    keel_area: Option<f64>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
///
impl StabilityComputer {
    /// Конструктор по умолчанию.
    /// * flooding_angle - Угол заливания отверстий
    /// * mean_draught - Средняя осадка
    /// * volume - Объемное водоизмещение
    /// * length_wl - Длина судна по ватерлинии при текущей осадке
    /// * width - Ширина судна полная
    /// * breadth_wl - Ширина судна по ватерлинии ватерлинии при текущей осадке
    /// * ship_moment - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * center_draught_shift - Отстояние центра величины погруженной части судна
    /// * pantocaren - Кривая плечей остойчивости формы для разных осадок
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    /// * wind - Расчет плеча кренящего момента от давления ветра
    /// * coefficient_k - Коэффициент k для судов, имеющих скуловые кили или
    /// брусковый киль. Табл. 2.1.5.2
    /// * multipler_x1 - Безразмерный множитель Х_1 Табл. 2.1.5.1-1
    /// * multipler_x2 - Безразмерный множитель Х_2 Табл. 2.1.5.1-2
    /// * multipler_s_area - Безразмерный множитель S Табл. 2.1.5.1-3
    /// * keel_area - Суммарная габаритная площадь скуловых килей
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        flooding_angle: f64,
        mean_draught: f64,
        volume: f64,
        length_wl: f64,
        width: f64,
        breadth_wl: f64,
        ship_moment: Rc<dyn IShipMoment>,
        center_draught_shift: Position,
        pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        rolling_amplitude: Rc<dyn IRollingAmplitude>,
        wind: Rc<dyn IWind>,
        coefficient_k: Rc<dyn ICurve>,
        multipler_x1: Rc<dyn ICurve>,
        multipler_x2: Rc<dyn ICurve>,
        multipler_s_area: Rc<dyn ICurve>,
        keel_area: Option<f64>,
        parameters: Rc<dyn IParameters>,
    ) -> Self {
        Self {
            flooding_angle,
            mean_draught,
            volume,
            length_wl,
            width,
            breadth_wl,
            ship_moment,
            center_draught_shift,
            pantocaren,
            metacentric_height,
            rolling_amplitude,
            wind,
            coefficient_k,
            multipler_x1,
            multipler_x2,
            multipler_s_area,
            keel_area,
            parameters,
        }
    }
    /// Расчет критерия погоды К (2.1.2)
    fn k(
        &self,
        lever_diagram: Rc<dyn ILeverDiagram>,
        rolling_amplitude: Rc<dyn IRollingAmplitude>,
    ) -> Result<f64, Error> {
        Stability::new(
            self.flooding_angle,
            Rc::clone(&lever_diagram),
            Rc::clone(&rolling_amplitude),
            Rc::clone(&self.wind),
            Rc::clone(&self.parameters),
        )
        .k()
    }
}
///
impl IStabilityComputer for StabilityComputer {
    /// Расчет допустимого возвышения центра тяжести судна
    fn zg(&self, target: f64) -> Result<f64, Error> {
        let mut z_g_fix = 50.;
        for _i in 0..30 {
            let metacentric_height: Rc<dyn IMetacentricHeight> =
                Rc::new(FakeMetacentricHeight::new(
                    self.metacentric_height.h_long_fix()?,
                    self.metacentric_height.h_trans_0()?,
                    self.metacentric_height.h_trans_fix()?,
                    z_g_fix,
                ));
            let delta = self
                .k(
                    Rc::new(LeverDiagram::new(
                        Rc::clone(&self.ship_moment),
                        self.center_draught_shift.clone(),
                        self.pantocaren.clone(),
                        self.mean_draught,
                        Rc::clone(&metacentric_height),
                        Rc::clone(&self.parameters),
                    )),
                    Rc::new(RollingAmplitude::new(
                        self.keel_area,
                        Rc::clone(&metacentric_height),
                        self.volume,     // Объемное водоизмещение (1)
                        self.length_wl,  // длинна по ватерлинии при текущей осадке
                        self.width,      // ширина полная
                        self.breadth_wl, // ширина по ватерлинии при текущей осадке
                        self.mean_draught,
                        Rc::clone(&self.coefficient_k),
                        Rc::clone(&self.multipler_x1),
                        Rc::clone(&self.multipler_x2),
                        Rc::clone(&self.multipler_s_area),
                        Rc::new(RollingPeriod::new(
                            self.length_wl,
                            self.width,
                            self.mean_draught,
                            Rc::clone(&metacentric_height),
                        )),
                    )?),
                )
                .unwrap_or(0.)
                - target;
            if delta <= 0.01 {
                break;
            }
            z_g_fix = z_g_fix * 0.5 * delta.signum();
        }
        Ok(z_g_fix)
    }
    /// Расчет критерия погоды К (2.1.2)
    fn k(&self) -> Result<f64, Error> {
        self.k(
            Rc::new(LeverDiagram::new(
                Rc::clone(&self.ship_moment),
                self.center_draught_shift.clone(),
                self.pantocaren.clone(),
                self.mean_draught,
                Rc::clone(&self.metacentric_height),
                Rc::clone(&self.parameters),
            )),
            Rc::clone(&self.rolling_amplitude),
        )
    }
}
#[doc(hidden)]
pub trait IStabilityComputer {
    /// Расчет допустимого возвышения центров тяжести судна
    fn zg(&self, target: f64) -> Result<f64, Error>;
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
