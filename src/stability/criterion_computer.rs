//!

use std::{collections::HashMap, rc::Rc};

use crate::{
    data::structs::{NavigationArea, ShipType},
    Error,
};

use super::{
    Acceleration, Circulation, Criterion, CriterionID, FakeMetacentricHeight, Grain, ILeverDiagram, IMetacentricHeight, IParameters, IRollingAmplitude, IRollingPeriod, IStability, IWind, LeverDiagram, RollingAmplitude, Stability, Wind
};

///
pub struct CriterionComputer {
    /// Максимальное исправленное отстояние центра масс судна по высоте
    max_zg: f64,
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
    /// Продольная и поперечная исправленная метацентрическая высота
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Период качки судна
    roll_period: Rc<dyn IRollingPeriod>,
    /// zg + Vec<id, delta>
    results: HashMap<f64, Vec<(CriterionID, f64)>>,
    /// Набор результатов расчетов для записи в БД
    parameters: Rc<dyn IParameters>,
}
///
impl CriterionComputer {
    /// Главный конструктор:
    /// * max_zg - Максимальное исправленное отстояние центра масс судна по высоте
    /// * ship_type - Тип судна
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
    /// * roll_period - Период качки судна
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        max_zg: f64,
        ship_type: ShipType,
        navigation_area: NavigationArea,
        have_timber: bool,
        have_grain: bool,
        have_cargo: bool,
        have_icing: bool,
        flooding_angle: f64,
        ship_length: f64,
        breadth: f64,
        moulded_depth: f64,
        h_subdivision: f64,
        roll_period: Rc<dyn IRollingPeriod>,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        parameters: Rc<dyn IParameters>,
    ) -> Result<Self, Error> {
        if max_zg <= 0. {
            return Err(Error::FromString(
                "CriterionComputer new error: max_zg <= 0.".to_string(),
            ));
        }
        if moulded_depth <= 0. {
            return Err(Error::FromString(
                "CriterionComputer new error: moulded_depth <= 0.".to_string(),
            ));
        }
        Ok(Self {
            max_zg,
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
            roll_period,
            metacentric_height,
            parameters,
        })
    }
    ///
    pub fn calculate(&mut self) -> Result<(), Error> {
        let delta = 0.001;
        let max_index = (self.max_zg / delta).ceil() as i32;
        for index in 0..=max_index {
            let z_g_fix = index as f64 * delta;
            let metacentric_height: Rc<dyn IMetacentricHeight> =
                Rc::new(FakeMetacentricHeight::new(
                    self.metacentric_height.h_long_fix()?,
                    self.metacentric_height.h_trans_0()?,
                    self.metacentric_height.h_trans_fix()?,
                    z_g_fix,
                ));
            let lever_diagram: Rc<dyn ILeverDiagram> = Rc::new(LeverDiagram::new(
                Rc::clone(&self.ship_moment),
                self.center_draught_shift.clone(),
                self.pantocaren.clone(),
                self.mean_draught,
                Rc::clone(&metacentric_height),
                Rc::clone(&self.parameters),
            ));
            // период качки судна
            let roll_period: Rc<dyn IRollingPeriod> = Rc::new(RollingPeriod::new(
                length_wl,
                data.width,
                mean_draught,
                Rc::clone(&metacentric_height),
            ));
            let rolling_amplitude: Rc<dyn IRollingAmplitude> = Rc::new(RollingAmplitude::new(
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
                Rc::clone(&self.roll_period),
            )?);
            let wind: Rc<dyn IWind> = Rc::new(Wind::new(
                data.navigation_area_param
                    .get_area(&data.navigation_area)
                    .ok_or("main error no area data!".to_string())?,
                Rc::new(Windage::new(
                    Rc::clone(&icing_stab),
                    Rc::clone(&area_stability),
                    Curve::new_linear(&data.delta_windage_area)?.value(mean_draught)?,
                    Moment::new(
                        Curve::new_linear(&data.delta_windage_moment_x)?.value(mean_draught)?,
                        0.,
                        Curve::new_linear(&data.delta_windage_moment_z)?.value(mean_draught)?,
                    ),
                    volume_shift,
                )),
                gravity_g,
                Rc::clone(&ship_mass),
                Rc::clone(&self.parameters),
            ));

            let results = Criterion::new(
                self.ship_type,
                self.navigation_area,
                self.have_timber,
                self.have_grain,
                self.have_cargo,
                self.have_icing,
                self.flooding_angle,
                self.ship_length,
                self.breadth,
                self.moulded_depth,
                self.h_subdivision,
                Rc::clone(&wind),
                Rc::clone(&lever_diagram),
                Rc::new(Stability::new(
                    self.flooding_angle,
                    Rc::clone(&lever_diagram),
                    Rc::clone(&rolling_amplitude),
                    Rc::clone(&wind),
                    Rc::clone(&self.parameters),
                )),
                Rc::clone(&metacentric_height),
                Rc::new(Acceleration::new(
                    self.width,
                    self.mean_draught,
                    Rc::clone(&self.coefficient_k_theta),
                    Rc::clone(&roll_period),
                    Rc::clone(&rolling_amplitude),
                    Rc::clone(&metacentric_height),
                )),
                Rc::new(Circulation::new(
                    data.velocity,
                    length_wl,
                    mean_draught,
                    Rc::clone(&self.ship_mass),
                    Rc::clone(&self.ship_moment),
                    Rc::clone(&lever_diagram),
                    Rc::clone(&self.parameters),
                )?),
                Box::new(Grain::new(
                    self.flooding_angle,
                    self.bulks.clone(),
                    Rc::clone(&self.ship_mass),
                    Rc::clone(&lever_diagram),
                    Rc::clone(&self.parameters),
                )),
            )
            .create();
        }
        Ok(())
    }
}
