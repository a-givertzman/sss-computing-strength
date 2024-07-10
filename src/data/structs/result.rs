//! Структуры для преобразования данных из формата данных DB
//! в формат пригодный для создания объектов.

use loads::{
    CompartmentArray, CompartmentData, LoadCargo, LoadCargoArray, LoadConstantArray, LoadConstantData,
};

use crate::error::Error;

use super::*;

/// Общая структура для ввода данных. Содержит все данные
/// для расчетов.
#[derive(Debug)]
pub struct ParsedShipData {
    /// Тип судна
    pub ship_type: ShipType,
    /// Параметры района плавания судна  
    pub navigation_area: NavigationArea,
    /// Параметры района плавания судна  
    pub navigation_area_param: NavigationAreaArray,
    /// Тип обледенения
    pub icing_stab: String,
    /// Тип обледенения палубного груза - леса
    pub icing_timber_stab: String,
    /// Масса льда на квадратный метр площади горизонтальной поверхности
    /// палубного лесного груза
    pub icing_m_timber: f64,
    /// Масса льда на квадратный метр площади парусности
    /// при учете полного обледенения
    pub icing_m_v_full: f64,
    /// Масса льда на квадратный метр площади парусности  
    /// при учете частичного обледенения
    pub icing_m_v_half: f64,
    /// Масса льда на квадратный метр площади горизонтальной
    /// поверхности при учете полного обледенения
    pub icing_m_h_full: f64,
    /// Масса льда на квадратный метр площади горизонтальной  
    /// поверхности при учете частичного обледенения
    pub icing_m_h_half: f64,
    /// Cтепень намокания палубного лесного груза, %
    pub wetting_timber: f64,
    /// Безразмерный множитель Х_1 для расчета качки, Табл. 2.1.5.1-1
    pub multipler_x1: MultiplerX1Array,
    /// Безразмерный множитель Х_2 для расчета качки, Табл. 2.1.5.1-2
    pub multipler_x2: MultiplerX2Array,
    /// Безразмерный множитель S для расчета качки, Табл. 2.1.5.1-3
    pub multipler_s: MultiplerSArray,
    /// Коэффициент k для судов, имеющих скуловые кили или
    /// брусковый киль для расчета качки, Табл. 2.1.5.2
    pub coefficient_k: CoefficientKArray,
    /// Коэффициент k_theta учитывающий особенности качки судов смешанного типа
    pub coefficient_k_theta: CoefficientKThetaArray,
    /// Длинна корпуса судна между перпендикулярами
    pub length_lbp: f64,
    /// Длинна корпуса судна полная
    pub length_loa: f64,
    /// Ширина корпуса судна
    pub width: f64,
    /// Отстояние миделя от нулевого шпангоута
    pub midship: f64,
    /// Эксплуатационная скорость судна, m/s
    pub velocity: f64,
    /// Cуммарная габаритная площадь скуловых килей,
    /// либо площадь боковой проекции брускового киля
    pub keel_area: Option<f64>,
    /// разбиение на шпации - фреймы
    pub bounds: Vec<(f64, f64)>,
    /// плотность воды
    pub water_density: f64,
    /// отстояние центра тяжести постоянной массы судна по x  
    pub const_mass_shift_x: f64,
    /// отстояние центра тяжести постоянной массы судна по y
    pub const_mass_shift_y: f64,
    /// отстояние центра тяжести постоянной массы судна по z
    pub const_mass_shift_z: f64,
    /// Минимальная осадка, м
    pub draught_min: f64,
    /// Высота борта, м
    pub moulded_depth: f64,
    /// Коэффициент увеличения площади парусности несплощной
    /// поверхности при учете обледенения
    pub icing_coef_v_area_full: f64,
    pub icing_coef_v_area_half: f64,
    pub icing_coef_v_area_zero: f64,
    /// Коэффициент увеличения статического момента
    /// площади парусности несплощной поверхности
    /// при учете обледенения
    pub icing_coef_v_moment_full: f64,
    pub icing_coef_v_moment_half: f64,
    pub icing_coef_v_moment_zero: f64,
    /// Кривая разницы в площадях парусности для минимальной осадки, м²
    pub delta_windage_area: Vec<(f64, f64)>,
    /// Кривая разницы в статических моментах относительно миделя, м
    pub delta_windage_moment_x: Vec<(f64, f64)>,
    /// Кривая разницы в статических моментах относительно ОП, м
    pub delta_windage_moment_z: Vec<(f64, f64)>,
    /// Кривая отстояния центра тяжести ватерлинии по длине от миделя  
    pub center_waterline: Vec<(f64, f64)>,
    /// Длинна корпуса судна по ватерлинии
    pub waterline_length: Vec<(f64, f64)>,
    /// Ширина корпуса судна по ватерлинии
    pub waterline_breadth: Vec<(f64, f64)>,
    /// Площадь ватерлинии
    pub waterline_area: Vec<(f64, f64)>,
    /// Отстояние по вертикали центра площади проекции подводной части корпуса
    pub volume_shift: Vec<(f64, f64)>,
    /// кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f64, f64)>,
    /// кривая поперечного метацентрического радиуса
    pub rad_trans: Vec<(f64, f64)>,
    /// Минимальная допустимая метацентрическая высота деления на отсеки
    pub h_subdivision: Vec<(f64, f64)>,
    /// кривая средней осадки
    pub mean_draught: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна по x
    pub center_draught_shift_x: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна по y
    pub center_draught_shift_y: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна по z
    pub center_draught_shift_z: Vec<(f64, f64)>,
    /// Кривые плечей остойчивости формы
    pub pantocaren: Vec<(f64, Vec<(f64, f64)>)>,
    /// Угол заливания отверстий
    pub flooding_angle: Vec<(f64, f64)>,
    /// Угол входа верхней палубы в воду
    pub entry_angle: Vec<(f64, f64)>,
    /// Погруженная площадь шпангоута
    pub frame_area: Vec<ParsedFrameData>,
    /// Координаты отметок заглубления на корпусе судна
    pub draft_mark: DraftMarkDataArray,
    /// Нагрузка судна без жидких грузов   
    pub cargoes: Vec<LoadCargo>,
    /// Нагрузка судна: цистерны и трюмы   
    pub compartments: Vec<CompartmentData>,
    /// Постоянная нагрузка на судно
    pub load_constants: Vec<LoadConstantData>,
    /// Площадь горизонтальных поверхностей для остойчивости
    pub area_h_stab: Vec<HStabArea>,
    /// Площадь горизонтальных поверхностей для прочности
    pub area_h_str: Vec<HStrArea>,
    /// Площадь и моменты поверхности парусности для остойчивости
    pub area_v_stab: stability::VerticalAreaArray,
    /// Площадь поверхности парусности для прочности
    pub area_v_str: Vec<strength::VerticalArea>,
}
///
impl ParsedShipData {
    /// Парсинг данных в общую структуру. Включает в себя  
    /// проверку данных на корректность.
    pub fn parse(
        navigation_area_param: NavigationAreaArray,
        multipler_x1: MultiplerX1Array,
        multipler_x2: MultiplerX2Array,
        multipler_s: MultiplerSArray,
        coefficient_k: CoefficientKArray,
        coefficient_k_theta: CoefficientKThetaArray,
        icing: IcingArray,
        ship_id: usize,
        ship_parameters: ShipArray,
        bounds: ComputedFrameDataArray,
        center_waterline: CenterWaterlineArray,
        waterline_length: WaterlineLengthArray,
        waterline_breadth: WaterlineBreadthArray,
        waterline_area: WaterlineAreaArray,
        volume_shift: VolumeShiftArray,
        rad_long: RadLongDataArray,
        rad_trans: RadTransDataArray,
        h_subdivision: MetacentricHeightSubdivisionArray,
        mean_draught: MeanDraughtDataArray,
        center_draught_shift: CenterDraughtShiftDataArray,
        pantocaren: PantocarenDataArray,
        flooding_angle: FloodingAngleDataArray,
        entry_angle: EntryAngleDataArray,
        delta_windage_area: DeltaWindageAreaDataArray,
        delta_windage_moment: DeltaWindageMomentDataArray,
        bonjean_frame: FrameIndexDataArray,
        frame_area: FrameAreaDataArray,
        draft_mark: DraftMarkDataArray,
        cargo_src: LoadCargoArray,
        compartments_src: CompartmentArray,
        load_constant_src: LoadConstantArray,
        area_h_stab: HStabAreaArray,
        area_h_str: HStrAreaArray,
        area_v_stab: stability::VerticalAreaArray,
        area_v_str: strength::VerticalAreaArray,
    ) -> Result<Self, Error> {
        log::info!("result parse begin");
        let ship_data = ship_parameters.data();
        let bonjean_frame = bonjean_frame.data();        
        let frame_area = frame_area.data();
        let mut parsed_frame_area = Vec::new();        
        for (index, x) in bonjean_frame {
            parsed_frame_area.push(ParsedFrameData {
                x,
                immersion_area: frame_area.get(&index).ok_or(format!(
                    "ParsedShipData parse error: no immersion_area for frame index:{}",
                    index
                ))?.to_vec(),
            });
        }
        parsed_frame_area.sort_by(|a, b| a.x.partial_cmp(&b.x).expect("result parsed_frame_area cpm error!"));
        
/*
        let mut cargoes = Vec::new();
        for cargo in cargo_src.data() {
            cargoes.push(ParsedCargoData {
                name: cargo.name,
                mass: cargo.mass.unwrap_or(0.),
                bound_x: ( cargo.bound_x1, cargo.bound_x2 ),
                bound_y: if cargo.bound_y1.is_some() && 
                            cargo.bound_y2.is_some() { Some(( 
                                cargo.bound_y1.expect("ParsedShipData parse error: no bound_y1"),
                                cargo.bound_y2.expect("ParsedShipData parse error: no bound_y2"),
                ))} else {
                    None
                },
                bound_z: if cargo.bound_z1.is_some() && cargo.bound_z2.is_some() {
                    Some((
                        cargo
                            .bound_z1
                            .expect("ParsedShipData parse error: no bound_z1"),
                        cargo
                            .bound_z2
                            .expect("ParsedShipData parse error: no bound_z2"),
                    ))
                } else {
                    None
                },
                mass_shift: if cargo.mass_shift_x.is_some()
                    && cargo.mass_shift_y.is_some()
                    && cargo.mass_shift_z.is_some()
                {
                    Some((
                        cargo
                            .mass_shift_x
                            .expect("ParsedShipData parse error: no mass_shift_x"),
                        cargo
                            .mass_shift_y
                            .expect("ParsedShipData parse error: no mass_shift_y"),
                        cargo
                            .mass_shift_z
                            .expect("ParsedShipData parse error: no mass_shift_z"),
                    ))
                } else {
                    None
                },
                horizontal_area: cargo.horizontal_area,
                horizontal_area_shift: if cargo.horizontal_area_shift_x.is_some()
                    && cargo.horizontal_area_shift_y.is_some()
                    && cargo.horizontal_area_shift_z.is_some()
                {
                    Some((
                        cargo
                            .horizontal_area_shift_x
                            .expect("ParsedShipData parse error: no horizontal_area_shift_x"),
                        cargo
                            .horizontal_area_shift_y
                            .expect("ParsedShipData parse error: no horizontal_area_shift_y"),
                        cargo
                            .horizontal_area_shift_z
                            .expect("ParsedShipData parse error: no horizontal_area_shift_z"),
                    ))
                } else {
                    None
                },
                vertical_area: cargo.vertical_area,
                vertical_area_shift: if cargo.vertical_area_shift_x.is_some()
                    && cargo.vertical_area_shift_y.is_some()
                    && cargo.vertical_area_shift_z.is_some()
                {
                    Some((
                        cargo
                            .vertical_area_shift_x
                            .expect("ParsedShipData parse error: no vertical_area_shift_x"),
                        cargo
                            .vertical_area_shift_y
                            .expect("ParsedShipData parse error: no vertical_area_shift_y"),
                        cargo
                            .vertical_area_shift_z
                            .expect("ParsedShipData parse error: no vertical_area_shift_z"),
                    ))
                } else {
                    None
                },
                loading_type: cargo.loading_type,
            });
        }
*/
        let icing = icing.data();

        log::info!("result parse ok");
        log::info!("result check begin");
        Self {
            ship_type: ShipType::new(
                &ship_data
                    .get("Type of ship")
                    .ok_or(format!(
                        "ParsedShipData parse error: no ship_type for ship id:{}",
                        ship_id
                    ))?
                    .0,
            ),
            navigation_area: NavigationArea::new(
                &ship_data
                    .get("Navigation area")
                    .ok_or(format!(
                        "ParsedShipData parse error: no navigation_area for ship id:{}",
                        ship_id
                    ))?
                    .0,
            ),
            navigation_area_param,
            multipler_x1,
            multipler_x2,
            multipler_s,
            coefficient_k,
            coefficient_k_theta,
            length_lbp: ship_data.get("LBP").ok_or(format!(
                "ParsedShipData parse error: no length for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            length_loa: ship_data.get("L.O.A").ok_or(format!(
                "ParsedShipData parse error: no length_loa for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            width: ship_data.get("Hull width").ok_or(format!(
                "ParsedShipData parse error: no width for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            midship: ship_data.get("X midship from Fr0").ok_or(format!(
                "ParsedShipData parse error: no midship for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            velocity: ship_data.get("Ship operating speed").ok_or(format!(
                "ParsedShipData parse error: no velocity for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
         /*   mass: ship_data.get("mass").ok_or(format!(
                "ParsedShipData parse error: no mass for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            volume: ship_data.get("volume").ok_or(format!(
                "ParsedShipData parse error: no volume for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,*/
            keel_area: ship_data.get("Keel area").ok_or(format!(
                "ParsedShipData parse error: no keel_area for ship id:{}",
                ship_id
            ))?.0.parse::<f64>().ok(),
            water_density: ship_data.get("Water Density").ok_or(format!(
                "ParsedShipData parse error: no water_density for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            const_mass_shift_x: ship_data.get("Center of mass shift x").ok_or(format!(
                "ParsedShipData parse error: no const_mass_shift_x for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            const_mass_shift_y: ship_data.get("Center of mass shift y").ok_or(format!(
                "ParsedShipData parse error: no const_mass_shift_y for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            const_mass_shift_z: ship_data.get("Center of mass shift z").ok_or(format!(
                "ParsedShipData parse error: no const_mass_shift_z for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            draught_min: ship_data.get("Minimum draft").ok_or(format!(
                "ParsedShipData parse error: no draught_min for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            moulded_depth: ship_data.get("Moulded depth").ok_or(format!(
                "ParsedShipData parse error: no moulded_depth for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_stab: ship_data.get("Type of icing").ok_or(format!(
                "ParsedShipData parse error: no icing_stab for ship id:{}",
                ship_id
            ))?.0.clone(),
            icing_timber_stab: ship_data.get("Type of icing timber").ok_or(format!(
                "ParsedShipData parse error: no icing_timber_stab for ship id:{}",
                ship_id
            ))?.0.clone(),
            icing_m_timber: *icing.get("icing_m_timber").ok_or(format!(
                "ParsedShipData parse error: no icing_m_timber for ship id:{}",
                ship_id
            ))?,
            icing_m_v_full: *icing.get("icing_m_v_full").ok_or(format!(
                "ParsedShipData parse error: no icing_m_v_full for ship id:{}",
                ship_id
            ))?,
            icing_m_v_half: *icing.get("icing_m_v_half").ok_or(format!(
                "ParsedShipData parse error: no icing_m_v_half for ship id:{}",
                ship_id
            ))?,
            icing_m_h_full: *icing.get("icing_m_h_full").ok_or(format!(
                "ParsedShipData parse error: no icing_m_h_full for ship id:{}",
                ship_id
            ))?,
            icing_m_h_half: *icing.get("icing_m_h_half").ok_or(format!(
                "ParsedShipData parse error: no icing_m_h_half for ship id:{}",
                ship_id
            ))?,
            wetting_timber: ship_data.get("Wetting of deck timber").ok_or(format!(
                "ParsedShipData parse error: no wetting_timber for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_coef_v_area_full: *icing.get("icing_coef_v_area_full").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_area_full for ship id:{}",
                ship_id
            ))?,
            icing_coef_v_area_half: *icing.get("icing_coef_v_area_half").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_area_half for ship id:{}",
                ship_id
            ))?,
            icing_coef_v_area_zero: *icing.get("icing_coef_v_area_zero").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_area_zero for ship id:{}",
                ship_id
            ))?,
            icing_coef_v_moment_full: *icing.get("icing_coef_v_moment_full").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_moment_full for ship id:{}",
                ship_id
            ))?,
            icing_coef_v_moment_half: *icing.get("icing_coef_v_moment_half").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_moment_half for ship id:{}",
                ship_id
            ))?,
            icing_coef_v_moment_zero: *icing.get("icing_coef_v_moment_zero").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_moment_zero for ship id:{}",
                ship_id
            ))?,
            bounds: bounds.data(),
            center_waterline: center_waterline.data(),
            waterline_length: waterline_length.data(),
            waterline_breadth: waterline_breadth.data(),
            waterline_area: waterline_area.data(),
            volume_shift: volume_shift.data(),
            rad_long: rad_long.data(),
            rad_trans: rad_trans.data(),
            h_subdivision: h_subdivision.data(),
            mean_draught: mean_draught.data(),
            center_draught_shift_x: center_draught_shift.x(),
            center_draught_shift_y: center_draught_shift.y(),
            center_draught_shift_z: center_draught_shift.z(),
            pantocaren: pantocaren.data(),
            flooding_angle: flooding_angle.data(),
            entry_angle: entry_angle.data(),
            delta_windage_area: delta_windage_area.data(),
            delta_windage_moment_x: delta_windage_moment.x(),
            delta_windage_moment_z: delta_windage_moment.z(),
            frame_area: parsed_frame_area,
            draft_mark,
            cargoes: cargo_src.data(),
            compartments: compartments_src.data(),
            load_constants: load_constant_src.data(),
            area_h_stab: area_h_stab.data(),
            area_h_str: area_h_str.data(),
            area_v_stab,
            area_v_str: area_v_str.data(),
        }
        .check()
    }
    /// Проверка данных на корректность
    fn check(self) -> Result<Self, Error> {
        if self.navigation_area_param.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check NavigationAreaArray: no data"
            )));
        }
        if self.multipler_x1.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check MultiplerX1Array: no data"
            )));
        }
        if self.multipler_x2.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check MultiplerX2Array: no data"
            )));
        }
        if self.multipler_s.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check MultiplerSArray: no data"
            )));
        }
        if self.coefficient_k.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check CoefficientKArray: no data"
            )));
        }
        if self.coefficient_k_theta.data.is_empty() {
            return Err(Error::Parameter(format!(
                "Error check CoefficientKThetaArray: no data"
            )));
        }
        if self.length_lbp <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: length must be positive {}",
                self.length_lbp
            )));
        }
        if self.width <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: width must be positive {}",
                self.width
            )));
        }
        if self.midship <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: midship must be positive {}",
                self.midship
            )));
        }
        if self.velocity <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: velocity must be positive {}",
                self.velocity
            )));
        }
        if let Some(keel_area) = self.keel_area {
            if keel_area < 0. {
                return Err(Error::Parameter(format!(
                    "Error check ParsedShipData: keel_area must be non-negative {keel_area}",
                )));
            }
        }
        if self.water_density <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of water density must be positive {}",
                self.water_density
            )));
        }
        /*   if self.mass <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of mass must be positive {}",
                self.mass
            )));
        }
        if self.volume <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of volume must be positive {}",
                self.volume
            )));
        }*/
        if self.draught_min <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of draught_min must be positive {}",
                self.draught_min
            )));
        }
        if self.moulded_depth <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of moulded_depth must be positive {}",
                self.moulded_depth
            )));
        }
        if self.icing_m_timber <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_timber must be positive {}",
                self.icing_m_timber
            )));
        }
        if self.icing_m_v_full <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_v_full must be positive {}",
                self.icing_m_v_full
            )));
        }
        if self.icing_m_v_half <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_v_half must be positive {}",
                self.icing_m_v_half
            )));
        }
        if self.icing_m_h_full <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_h_full must be positive {}",
                self.icing_m_h_full
            )));
        }
        if self.icing_m_h_half <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of icing_m_h_half must be positive {}",
                self.icing_m_h_half
            )));
        }
        if self.icing_m_v_full <= self.icing_m_v_half {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: icing_m_v_full {} <= icing_m_v_half {}",
                self.icing_m_v_full, self.icing_m_v_half
            )));
        }
        if self.icing_m_h_full <= self.icing_m_h_half {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: icing_m_h_full {} <= icing_m_h_half {}",
                self.icing_m_h_full, self.icing_m_h_half
            )));
        }
        if self.icing_coef_v_area_full <= 0. || self.icing_coef_v_area_full > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_area_full < 1 {}",
                self.icing_coef_v_area_full
            )));
        }
        if self.icing_coef_v_area_half <= 0. || self.icing_coef_v_area_half > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_area_half < 1 {}",
                self.icing_coef_v_area_half
            )));
        }
        if self.icing_coef_v_moment_full <= 0. || self.icing_coef_v_moment_full > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_moment_full < 1 {}",
                self.icing_coef_v_moment_full
            )));
        }
        if self.icing_coef_v_moment_half <= 0. || self.icing_coef_v_moment_half > 1. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: 0 <= icing_coef_v_moment_half < 1 {}",
                self.icing_coef_v_moment_half
            )));
        }
        if self.bounds.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of bounds's points {}",
                self.bounds.len()
            )));
        }
        if self.center_waterline.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of waterline's points {}",
                self.center_waterline.len()
            )));
        }
        if self.rad_long.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of rad_long's points {}",
                self.rad_long.len()
            )));
        }
        if self.rad_trans.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of rad_trans's points {}",
                self.rad_trans.len()
            )));
        }
        if self.h_subdivision.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of h_subdivision's points {}",
                self.h_subdivision.len()
            )));
        }
        if self.mean_draught.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of mean_draught's points {}",
                self.mean_draught.len()
            )));
        }
        if self.center_draught_shift_x.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of center_draught_shift_x's points {}",
                self.center_draught_shift_x.len()
            )));
        }
        if self.center_draught_shift_y.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of center_draught_shift_y's points {}",
                self.center_draught_shift_y.len()
            )));
        }
        if self.center_draught_shift_z.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of center_draught_shift_z's points {}",
                self.center_draught_shift_z.len()
            )));
        }
        if self.pantocaren.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of pantocaren's points {}",
                self.pantocaren.len()
            )));
        }
        if let Some((draught, _)) = self.pantocaren.iter().find(|(draught, _)| *draught < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught in pantocaren is negative!, {}",
                draught
            )));
        }
        if self.flooding_angle.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of points {}",
                self.flooding_angle.len()
            )));
        }
        if let Some((key, value)) = self
            .flooding_angle
            .iter()
            .find(|(key, value)| *key < 0. || *value < 0.)
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught or angle in flooding_angle is negative!, draught{key}, angle:{value}")));
        }
        if self.entry_angle.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of points {}",
                self.flooding_angle.len()
            )));
        }
        if self.delta_windage_area.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check delta_windage_area: number of points {}",
                self.flooding_angle.len()
            )));
        }
        if self.delta_windage_moment_x.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check delta_windage_moment_x: number of points{}",
                self.flooding_angle.len()
            )));
        }
        if self.delta_windage_moment_z.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check delta_windage_moment_z: number of points{}",
                self.flooding_angle.len()
            )));
        }
        if let Some((key, value)) = self
            .entry_angle
            .iter()
            .find(|(key, value)| *key < 0. || *value < 0.)
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught or angle in entry_angle is negative!, draught{key}, angle:{value}")));
        }
        if self.frame_area.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of frames must be {}",
                self.frame_area.len()
            )));
        }
        if self.draft_mark.data().is_empty() {
            return Err(Error::Parameter(format!(
                "Error check draft_mark: draft_mark.data().is_empty()"
            )));
        }

        /*      if let Some(frame) = self.theoretical_frame.iter().find(|f| f.index >= self.frame_area.len() as i32) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: index of frame bigger or equal then frames.len(), {}",
                frame
            )));
        }
        if let Some(frame) = self.theoretical_frame.iter().find(|f| f.index < 0) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: index of frame is negative, {}",
                frame
            )));
        }
        let qnt_unique_index = self
            .theoretical_frame
            .iter()
            .map(|f| f.index)
            .collect::<HashSet<_>>()
            .len();
        if self.theoretical_frame.len() != qnt_unique_index {
            return Err(Error::Parameter(format!("Error check ParsedShipData: index of frame must be unique frames:{}, unique index:{}", self.theoretical_frame.len(), qnt_unique_index )));
        }
        if self
            .theoretical_frame
            .iter()
            .find(|f| f.index == 0)
            .ok_or(format!(
                "ParsedShipData parse error: no frame with index = 0"
            ))?
            .x
            != 0.
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: x of frame with index = 0 must be equal to 0"
            )));
        }*/
        /* длинна судна не обязательно совпадает с расстоянием между крайними шпангоутами
        if (self
            .frames
            .iter()
            .find(|f| f.index == self.frames.len() - 1)
            .ok_or(format!(
                "ParsedShipData parse error: no frame with last index = len-1"
            ))?
            .x - self.ship_length).abs() > 0.01
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: x of frame with last index must be equal to ship_length"
            )));
        }*/
        /*     if let Some(frame) = self.theoretical_frame.iter().find(|f| f.x < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: x of frame must be greater or equal to 0, {}",
                frame
            )));
        }*/
        if let Some(frame) = self.frame_area.iter().find(|f| {
            f.immersion_area
                .iter()
                .find(|v| v.0 < 0. || v.1 < 0.)
                .is_some()
        }) {
            return Err(Error::Parameter(format!("Error check ParsedShipData: values of immersion_area in frame must be greater or equal to 0, {}", frame)));
        }
        /*      let cargo_data = self.cargo.data();
        if let Some((index, value)) = cargo_data.iter().find(|(_, value)| **value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check LoadConstantArray: mass of load_constant must be greater or equal to 0, index:{}, value:{}",
                index, value,
            )));
        } */
        /*     if let Some((index, _)) = load_constant_data.iter().find(|(index, _)| self.theoretical_frame.iter().find(|frame| frame.index == **index as i32 ).is_none()) {
            return Err(Error::Parameter(format!(
                "Error check LoadConstantArray: index of load_constant must be contained in frames, index:{}", index)));
        }*/
        /*    if self.compartments.len() < 1 {
            return Err(Error::Parameter(format!(
                "Error check compartments: number of compartments: {}",
                self.compartments.len()
            )));
        }*/
        if let Some(s) = self.compartments.iter().find(|s| s.mass.unwrap() <= 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: mass of compartment must be greater or equal to 0, {}",
                s
            )));
        }
        if let Some(s) = self.compartments.iter().find(|s| s.bound_x1 >= s.bound_x2) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: compartment bound error! {}",
                s
            )));
        }
        if let Some(s) = self.compartments.iter().find(|s| {
            s.mass.unwrap() > 0.
                && s.mass_shift_x.is_some()
                && (s.bound_x1 >= s.mass_shift_x.unwrap() || s.mass_shift_x.unwrap() >= s.bound_x2)
        }) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: compartment center out of bound error! {}",
                s
            )));
        }
        if self.load_constants.len() < 1 {
            return Err(Error::Parameter(format!(
                "Error check load_constants: number of load_constants: {}",
                self.load_constants.len()
            )));
        }

        /*       if let Some(tank) = self.tanks.iter().find(|t| t.density <= 0.) {
                  return Err(Error::Parameter(format!(
                      "Error check ParsedShipData: density of liquid must be greater or equal to 0 {}",
                      tank
                  )));
              }
              if let Some(tank) = self.tanks.iter().find(|t| t.volume <= 0.) {
                  return Err(Error::Parameter(format!(
                      "Error check ParsedShipData: volume of liquid must be greater or equal to 0 {}",
                      tank
                  )));
              }
              if let Some(tank) = self
                  .tanks
                  .iter()
                  .find(|t| t.center_x.len() <= 1 || t.center_y.len() <= 1 || t.center_z.len() <= 1)
              {
                  return Err(Error::Parameter(format!("Error check ParsedShipData: number of center's points must be {}", tank)));
              }
              if let Some(tank) = self
                  .tanks
                  .iter()
                  .find(|t| t.free_surf_inertia_x.len() <= 1 || t.free_surf_inertia_y.len() <= 1)
              {
                  return Err(Error::Parameter(format!("Error check ParsedShipData: number of free_surf_inertia's points must be {}", tank)));
              }
        */
        if self.waterline_length.len() <= 1 {
            return Err(Error::Parameter(format!("Error check waterline_length")));
        }
        if self.waterline_breadth.len() <= 1 {
            return Err(Error::Parameter(format!("Error check waterline_breadth")));
        }
        if self.waterline_area.len() <= 1 {
            return Err(Error::Parameter(format!("Error check waterline_area")));
        }
        if self.area_h_stab.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check area_h_stab: number of area_h_stab's points {}",
                self.area_h_stab.len()
            )));
        }
        if let Some(area) = self.area_h_stab.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check area_h_stab: value of area_h_stab must be greater or equal to 0, {}",
                area
            )));
        }
        if self.area_h_str.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check area_h_str: number of points {}",
                self.area_h_str.len()
            )));
        }
        if let Some(area) = self.area_h_str.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check area_h_str: value of area_h_str must be greater or equal to 0, {}",
                area
            )));
        }
        if let Some(area) = self.area_h_str.iter().find(|f| f.bound_x1 >= f.bound_x2) {
            return Err(Error::Parameter(format!(
                "Error check area_h_str: f.bound_x1 >= f.bound_x2 {}",
                area
            )));
        }      
        if self.area_v_str.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check area_v_str: number of points {}",
                self.area_v_str.len()
            )));
        }
        if let Some(area) = self.area_v_str.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check area_v_str: value of area_v must be greater or equal to 0, {}",
                area
            )));
        }
        if self.area_v_stab.area().len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check area_v_stab.area(): number of points {}",
                self.area_v_stab.area().len() 
            )));
        }
        if let Some(v) = self.area_v_stab.area().iter().find(|(d, a)| *a < 0. || *d < 0. ) {
            return Err(Error::Parameter(format!(
                "Error check area_v_stab: draught:{},  area:{}",
                v.0, v.1
            )));
        }  
        Ok(self)
    }
}
