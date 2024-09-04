//!

use std::{collections::HashMap, rc::Rc};

use crate::{
    data::structs::{NavigationArea, ShipType}, Error, IBulk, ICurve, IMass, Position
};

use super::{
    Acceleration, Circulation, Criterion, FakeMetacentricHeight, Grain, ILeverDiagram,
    IMetacentricHeight, IParameters, IRollingAmplitude, IRollingPeriod, IShipMoment, 
    IWind, LeverDiagram, Parameters, RollingAmplitude, RollingPeriod, Stability, 
};

///
pub struct CriterionComputer {
    /// Максимальное исправленное отстояние центра масс судна по высоте
    max_zg: f64,
    /// Тип судна
    ship_type: ShipType,
    /// Минимальная допустимая метацентрическая высота деления на отсеки
    h_subdivision: f64,
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
    /// Длинна корпуса судна между перпендикулярами
    ship_length: f64,
    /// Высота борта, м
    moulded_depth: f64,
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
    /// Эксплуатационная скорость судна, m/s
    velocity: f64,
    /// Момент массы судна: сумма моментов конструкции, груз, экипаж и т.п. для расчета остойчивости
    ship_moment: Rc<dyn IShipMoment>,
    /// Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    ship_mass: Rc<dyn IMass>,
    /// Все навалочные смещаемые грузы судна
    loads_bulk: Rc<Vec<Rc<dyn IBulk>>>,
    /// Коэффициент k для судов, имеющих скуловые кили или
    /// брусковый киль. Табл. 2.1.5.2
    coefficient_k: Rc<dyn ICurve>,
    /// Безразмерный множитель Х_1 Табл. 2.1.5.1-1
    multipler_x1: Rc<dyn ICurve>,
    /// Безразмерный множитель Х_2 Табл. 2.1.5.1-2
    multipler_x2: Rc<dyn ICurve>,
    /// Безразмерный множитель S Табл. 2.1.5.1-3
    multipler_s_area: Rc<dyn ICurve>,
    /// Коэффициент, учитывающий особенности качки судов смешанного типа
    coefficient_k_theta: Rc<dyn ICurve>,
    /// Суммарная габаритная площадь скуловых килей
    keel_area: Option<f64>,
    /// Поперечный метацентрические радиус
    rad_trans: f64,
    /// Отстояние центра величины погруженной части судна
    center_draught_shift: Position,
    /// Кривая плечей остойчивости формы для разных осадок
    pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
    /// Продольная и поперечная исправленная метацентрическая высота
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Расчет плеча кренящего момента от давления ветра
    wind: Rc<dyn IWind>,
}
///
impl CriterionComputer {
    /// Главный конструктор:
    /// * max_zg - Максимальное исправленное отстояние центра масс судна по высоте
    /// * ship_type - Тип судна
    /// * h_subdivision - Минимальная допустимая метацентрическая высота деления на отсеки
    /// * navigation_area - Район плавания судна
    /// * have_timber - Признак наличия леса
    /// * have_grain - Признак наличия сыпучего груза
    /// * have_cargo - Признак наличия груза или балласта
    /// * have_icing - Признак учета обледенения
    /// * flooding_angle - Угол заливания отверстий
    /// * ship_length - Длинна корпуса судна между перпендикулярами
    /// * moulded_depth - Высота борта, м
    /// * mean_draught - Средняя осадка
    /// * volume - Объемное водоизмещение
    /// * length_wl - Длина судна по ватерлинии при текущей осадке
    /// * width - Ширина судна полная
    /// * breadth_wl - Ширина судна по ватерлинии ватерлинии при текущей
    /// * velocity - Эксплуатационная скорость судна, m/s
    /// * ship_moment - Момент массы судна: сумма моментов конструкции, груз, экипаж и т.п. для расчета остойчивости
    /// * ship_mass - Нагрузка на корпус судна: конструкции, груз, экипаж и т.п.
    /// * loads_bulk - Все навалочные смещаемые грузы судна
    /// * coefficient_k - Коэффициент k для судов, имеющих скуловые кили или
    /// брусковый киль. Табл. 2.1.5.2
    /// * multipler_x1 - Безразмерный множитель Х_1 Табл. 2.1.5.1-1
    /// * multipler_x2 - Безразмерный множитель Х_2 Табл. 2.1.5.1-2
    /// * multipler_s_area - Безразмерный множитель S Табл. 2.1.5.1-3
    /// * coefficient_k_theta - Коэффициент, учитывающий особенности качки судов смешанного типа
    /// * keel_area - Суммарная габаритная площадь скуловых килей
    /// * rad_trans - Поперечный метацентрические радиус
    /// * center_draught_shift - Отстояние центра величины погруженной части судна
    /// * pantocaren - Кривая плечей остойчивости формы для разных осадок
    /// * wind - Расчет плеча кренящего момента от давления ветра
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота
    /// * parameters - Набор результатов расчетов для записи в БД
    pub fn new(
        max_zg: f64,
        ship_type: ShipType,
        h_subdivision: f64,
        navigation_area: NavigationArea,
        have_timber: bool,
        have_grain: bool,
        have_cargo: bool,
        have_icing: bool,
        flooding_angle: f64,
        ship_length: f64,
        moulded_depth: f64,
        mean_draught: f64,
        volume: f64,
        length_wl: f64,
        width: f64,
        breadth_wl: f64,
        velocity: f64,
        ship_moment: Rc<dyn IShipMoment>,
        ship_mass: Rc<dyn IMass>,
        loads_bulk: Rc<Vec<Rc<dyn IBulk>>>,
        coefficient_k: Rc<dyn ICurve>,
        multipler_x1: Rc<dyn ICurve>,
        multipler_x2: Rc<dyn ICurve>,
        multipler_s_area: Rc<dyn ICurve>,
        coefficient_k_theta: Rc<dyn ICurve>,
        keel_area: Option<f64>,
        rad_trans: f64,
        center_draught_shift: Position,
        pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
        wind: Rc<dyn IWind>,
        metacentric_height: Rc<dyn IMetacentricHeight>,
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
            h_subdivision,
            navigation_area,
            have_timber,
            have_grain,
            have_cargo,
            have_icing,
            flooding_angle,
            ship_length,
            moulded_depth,
            mean_draught,
            volume,
            length_wl,
            width,
            breadth_wl,
            velocity,
            ship_moment,
            ship_mass,
            loads_bulk,
            rad_trans,
            center_draught_shift,
            pantocaren,
            coefficient_k,
            multipler_x1,
            multipler_x2,
            multipler_s_area,
            coefficient_k_theta,
            keel_area,
            wind,
            metacentric_height,
        })
    }
    /// criterion_id, zg, result, target
    pub fn calculate(&mut self) -> Result<Vec<(usize, f64, f64, f64)>, Error> {
        let parameters: Rc<dyn IParameters> = Rc::new(Parameters::new());
        // zg + Vec<id, delta>
        let mut results = Vec::new(); //<(f64, Vec<(usize, Option<f64>)>)>'
        let delta = 0.01;
        let max_index = (self.max_zg / delta).ceil() as i32;
        for index in 0..=max_index {
            let z_g_fix = index as f64 * delta;
            let z_m = self.center_draught_shift.z() + self.rad_trans;
            let delta_m_h = self.metacentric_height.delta_m_h()?;
            let h = z_m - z_g_fix;            
            let h_0 = h + delta_m_h.trans();
            let metacentric_height: Rc<dyn IMetacentricHeight> =
                Rc::new(FakeMetacentricHeight::new(
                    self.metacentric_height.h_long_fix()?,
                    h_0,
                    h,
                    z_g_fix,
                    delta_m_h,
                ));
            let lever_diagram: Rc<dyn ILeverDiagram> = Rc::new(LeverDiagram::new(
                Rc::clone(&self.ship_moment),
                self.center_draught_shift.clone(),
                self.pantocaren.clone(),
                self.mean_draught,
                Rc::clone(&metacentric_height),
                Rc::clone(&parameters),
            ));
            // период качки судна
            let roll_period: Rc<dyn IRollingPeriod> = Rc::new(RollingPeriod::new(
                self.length_wl,
                self.width,
                self.mean_draught,
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
                Rc::clone(&roll_period),
            )?);
            // релузьтат расчета критериев для текущего zg
            let tmp = Criterion::new(
                self.ship_type,
                self.navigation_area,
                self.have_timber,
                self.have_grain,
                self.have_cargo,
                self.have_icing,
                self.flooding_angle,
                self.ship_length,
                self.width,
                self.moulded_depth,
                self.h_subdivision,
                Rc::clone(&self.wind),
                Rc::clone(&lever_diagram),
                Rc::new(Stability::new(
                    self.flooding_angle,
                    Rc::clone(&lever_diagram),
                    Rc::clone(&rolling_amplitude),
                    Rc::clone(&self.wind),
                    Rc::clone(&parameters),
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
                    self.velocity,
                    self.length_wl,
                    self.mean_draught,
                    Rc::clone(&self.ship_mass),
                    Rc::clone(&self.ship_moment),
                    Rc::clone(&lever_diagram),
                    Rc::clone(&parameters),
                )?),
                Box::new(Grain::new(
                    self.flooding_angle,
                    self.loads_bulk.clone(),
                    Rc::clone(&self.ship_mass),
                    Rc::clone(&lever_diagram),
                    Rc::clone(&parameters),
                )),
            )?
            .create();
            // отбрасываем ошибки, оставляем только значения, считаем дельту с целевым значением 
            let tmp: Vec<(usize, Option<(f64, f64)>)> = tmp
                .iter()
                .map(|v| {
                    let delta = if v.error_message.is_none() {
                        //                 dbg!(z_g_fix, v.criterion_id, v.result, v.target);
                      //  Some(v.result - v.target)
                      Some((v.result, v.target))
                    } else {
                        None
                    };
                    (v.criterion_id, delta)
                })
                .collect();
            results.push((z_g_fix, tmp));
        }
        // создаем коллекцию векторов, сортируем значения по id
        let mut values: HashMap<usize, Vec<(f64, (f64, f64))>> = HashMap::new();
        for (z_g_fix, tmp) in results.into_iter() {
            tmp.into_iter()
                .filter(|(_, value)| value.is_some())
                .for_each(|(id, value)| {
                    values
                        .entry(id)
                        .and_modify(|v| v.push((z_g_fix, value.unwrap())))
                        .or_insert(vec![(z_g_fix, value.unwrap())]);
                });
        }
        let mut result = Vec::new();
        for (id, mut values) in values.into_iter() {
            // сортируем значения по увеличению дельты с целевым
            values.sort_by(|&(_, v1), &(_, v2)| {
                (v1.0 - v1.1).abs()
                    .partial_cmp(&(v2.0 - v2.1).abs())
                    .expect("CriterionComputer calculate error: sort values!")
            });
            // берем первое значение как ближайшее значение к целевому
            let closest_value = values
                .first()
                .expect("CriterionComputer calculate error, no values!");
            result.push((id, closest_value.0, closest_value.1.0, closest_value.1.1));
        }
        Ok(result)
    }
}
