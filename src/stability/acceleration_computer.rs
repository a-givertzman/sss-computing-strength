//! Расчет критерия ускорения

use std::rc::Rc;

use crate::{Error, ICurve, IMetacentricHeight, IRollingAmplitude, IRollingPeriod};

use super::{FakeMetacentricHeight, FakeRollingAmplitude, RollingAmplitude};

/// Расчет критерия ускорения
pub struct AccelerationComputer {
    /// Ширина судна полная
    width: f64,
    /// Осадка судна полная
    draught: f64,
    /// Средняя осадка судна
    mean_draught: f64,
    /// Объемное водоизмещение
    volume: f64,
    /// Длина судна по ватерлинии при текущей осадке
    length_wl: f64,
    /// Ширина судна по ватерлинии ватерлинии при текущей осадке
    breadth_wl: f64,
    /// Коэффициент, учитывающий особенности качки судов смешанного типа
    k_theta: Rc<dyn ICurve>,
    /// Период качки судна
    rolling_period: Rc<dyn IRollingPeriod>,
    /// Амплитуда качки судна с круглой скулой (2.1.5)
    rolling_amplitude: Rc<dyn IRollingAmplitude>,
    /// Продольная и поперечная исправленная метацентрическая высота.
    metacentric_height: Rc<dyn IMetacentricHeight>,
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
}
///
impl AccelerationComputer {
    /// Основной конструктор
    /// * width - Ширина судна полная
    /// * draught - Осадка судна полная
    /// * mean_draught - Средняя осадка
    /// * volume - Объемное водоизмещение
    /// * length_wl - Длина судна по ватерлинии при текущей осадке
    /// * width - Ширина судна полная
    /// * breadth_wl - Ширина судна по ватерлинии ватерлинии при текущей осадке
    /// * k_theta - Коэффициент, учитывающий особенности качки судов смешанного типа
    /// * rolling_period - Период качки судна
    /// * rolling_amplitude - Амплитуда качки судна с круглой скулой (2.1.5)
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота.
    /// * coefficient_k - Коэффициент k для судов, имеющих скуловые кили или
    /// брусковый киль. Табл. 2.1.5.2
    /// * multipler_x1 - Безразмерный множитель Х_1 Табл. 2.1.5.1-1
    /// * multipler_x2 - Безразмерный множитель Х_2 Табл. 2.1.5.1-2
    /// * multipler_s_area - Безразмерный множитель S Табл. 2.1.5.1-3
    /// * keel_area - Суммарная габаритная площадь скуловых килей
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        width: f64,
        draught: f64,
        mean_draught: f64,
        volume: f64,
        length_wl: f64,
        breadth_wl: f64,
        k_theta: Rc<dyn ICurve>,
        rolling_period: Rc<dyn IRollingPeriod>,
        rolling_amplitude: Rc<dyn IRollingAmplitude>,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        coefficient_k: Rc<dyn ICurve>,
        multipler_x1: Rc<dyn ICurve>,
        multipler_x2: Rc<dyn ICurve>,
        multipler_s_area: Rc<dyn ICurve>,
        keel_area: Option<f64>,
    ) -> Self {
        Self {
            width,
            draught,
            mean_draught,
            volume,
            length_wl,
            breadth_wl,
            k_theta,
            rolling_period,
            rolling_amplitude,
            metacentric_height,
            coefficient_k,
            multipler_x1,
            multipler_x2,
            multipler_s_area,
            keel_area,
        }
    }
    /// Расчет критерия ускорения
    fn value(&self, rolling_amplitude: Rc<dyn IRollingAmplitude>) -> Result<f64, Error> {
        let h_trans_0 = self.metacentric_height.h_trans_0()?;
        let k_theta = self.k_theta.value(self.width / self.draught)?;
        let c = self.rolling_period.c();
        let (_, theta_1_r) = rolling_amplitude.calculate()?;
        let a = 0.0105 * h_trans_0 / (c * c * self.width) * k_theta * theta_1_r;
        let k = 0.3 / a; // >= 1;
        Ok(k)
    }
}
///
impl IAccelerationComputer for AccelerationComputer {
    /// Расчет допустимого возвышения центра тяжести судна
    fn zg(&self, target: f64) -> Result<f64, Error> {
        let mut z_g_fix = 50.;
        for _i in 0..30 {
            let delta = self
                .value(Rc::new(RollingAmplitude::new(
                    self.keel_area,
                    Rc::new(FakeMetacentricHeight::new(
                        self.metacentric_height.h_long_fix()?,
                        self.metacentric_height.h_trans_0()?,
                        self.metacentric_height.h_trans_fix()?,
                        z_g_fix,
                    )),
                    self.volume,     // Объемное водоизмещение (1)
                    self.length_wl,  // длинна по ватерлинии при текущей осадке
                    self.width,      // ширина полная
                    self.breadth_wl, // ширина по ватерлинии при текущей осадке
                    self.mean_draught,
                    Rc::clone(&self.coefficient_k),
                    Rc::clone(&self.multipler_x1),
                    Rc::clone(&self.multipler_x2),
                    Rc::clone(&self.multipler_s_area),
                    Rc::clone(&self.rolling_period),
                )?))
                .unwrap_or(0.)
                - target;
            if delta <= 0.01 {
                break;
            }
            z_g_fix = z_g_fix * 0.5 * delta.signum();
        }
        Ok(0.)
    }
    /// Расчет критерия ускорения
    fn value(&self) -> Result<f64, Error> {
        self.value(Rc::clone(&self.rolling_amplitude))
    }
}
#[doc(hidden)]
pub trait IAccelerationComputer {
    /// Расчет допустимого возвышения центров тяжести судна
    fn zg(&self, target_k: f64) -> Result<f64, Error>;
    /// Расчет критерия ускорения
    fn value(&self) -> Result<f64, Error>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeAccelerationComputer {
    zg: f64,
    value: f64,
}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeAccelerationComputer {
    pub fn new(zg: f64, value: f64) -> Self {
        Self { zg, value }
    }
}
#[doc(hidden)]
impl IAccelerationComputer for FakeAccelerationComputer {
    /// Расчет допустимого возвышения центров тяжести судна
    fn zg(&self, _: f64) -> Result<f64, Error> {
        Ok(self.zg)
    }
    ///
    fn value(&self) -> Result<f64, Error> {
        Ok(self.value)
    }
}
