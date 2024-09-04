//! Структуры для преобразования данных из формата данных DB
//! в формат пригодный для создания объектов.

use loads::{
    compartment, BulkheadArray, CompartmentArray, CompartmentData, LoadCargo, LoadCargoArray, LoadConstantArray, LoadConstantData
};

use crate::{error::Error, icing_stab::IcingStabType, icing_timber::IcingTimberType};

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
    pub icing_stab: IcingStabType,
    /// Тип обледенения палубного груза - леса
    pub icing_timber_stab: IcingTimberType,
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
    /// Overall height up to non-removable parts
    pub overall_height: f64,
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
    pub draft_mark: Vec<DraftMarkParsedData>,
    /// Координаты отметок осадок на корпусе судна
    pub load_line: Vec<LoadLineParsedData>,
    /// Координаты и диаметр винтов судна
    pub screw: Vec<ScrewParsedData>,
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
        load_line: LoadLineDataArray,
        screw: ScrewDataArray,
        cargo_src: LoadCargoArray,
        bulkhead_src: BulkheadArray,
        compartments_src: CompartmentArray,
        hold_compartments_src: CompartmentArray,
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
        let icing = icing.data();
        let ship_type = ShipType::from_str(
            &ship_data
                .get("Type of ship")
                .ok_or(format!(
                    "ParsedShipData parse error: no ship_type for ship id:{}",
                    ship_id
                ))?
                .0,
        ).map_err(|e| format!("Error parse ship_type: {e}"))?;
        let navigation_area = NavigationArea::from_str(
            &ship_data
                .get("Navigation area")
                .ok_or(format!(
                    "ParsedShipData parse error: no navigation_area for ship id:{}",
                    ship_id
                ))?
                .0,
        ).map_err(|e| format!("Error parse navigation_area: {e}"))?;
        let length_lbp = ship_data.get("LBP").ok_or(format!(
            "ParsedShipData parse error: no length for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let length_loa = ship_data.get("L.O.A").ok_or(format!(
            "ParsedShipData parse error: no length_loa for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let width = ship_data.get("MouldedBreadth").ok_or(format!(
            "ParsedShipData parse error: no width for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let midship = ship_data.get("X midship from Fr0").ok_or(format!(
            "ParsedShipData parse error: no midship for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let overall_height = ship_data.get("Overall height up to non-removable parts").ok_or(format!(
            "ParsedShipData parse error: no overall_height for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let velocity = ship_data.get("Ship operating speed").ok_or(format!(
            "ParsedShipData parse error: no velocity for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let keel_area = ship_data.get("Keel area").ok_or(format!(
            "ParsedShipData parse error: no keel_area for ship id:{}",
            ship_id
        ))?.0.parse::<f64>().ok();
        let water_density = ship_data.get("Water Density").ok_or(format!(
            "ParsedShipData parse error: no water_density for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let const_mass_shift_x = ship_data.get("Center of mass shift x").ok_or(format!(
            "ParsedShipData parse error: no const_mass_shift_x for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let const_mass_shift_y = ship_data.get("Center of mass shift y").ok_or(format!(
            "ParsedShipData parse error: no const_mass_shift_y for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let const_mass_shift_z = ship_data.get("Center of mass shift z").ok_or(format!(
            "ParsedShipData parse error: no const_mass_shift_z for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let draught_min = ship_data.get("Minimum draft").ok_or(format!(
            "ParsedShipData parse error: no draught_min for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let moulded_depth = ship_data.get("Moulded depth").ok_or(format!(
            "ParsedShipData parse error: no moulded_depth for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let icing_stab = IcingStabType::from_str(&ship_data.get("Type of icing").ok_or(format!(
            "ParsedShipData parse error: no icing_stab for ship id:{}",
            ship_id
        ))?.0).map_err(|e| format!("Error parse icing_stab: {e}"))?;
        let icing_timber_stab = IcingTimberType::from_str(&ship_data.get("Type of icing timber").ok_or(format!(
            "ParsedShipData parse error: no icing_timber_stab for ship id:{}",
            ship_id
        ))?.0).map_err(|e| format!("Error parse icing_timber_stab: {e}"))?;
        let icing_m_timber = *icing.get("icing_m_timber").ok_or(format!(
            "ParsedShipData parse error: no icing_m_timber for ship id:{}",
            ship_id
        ))?;
        let icing_m_v_full = *icing.get("icing_m_v_full").ok_or(format!(
            "ParsedShipData parse error: no icing_m_v_full for ship id:{}",
            ship_id
        ))?;
        let icing_m_v_half = *icing.get("icing_m_v_half").ok_or(format!(
            "ParsedShipData parse error: no icing_m_v_half for ship id:{}",
            ship_id
        ))?;
        let icing_m_h_full = *icing.get("icing_m_h_full").ok_or(format!(
            "ParsedShipData parse error: no icing_m_h_full for ship id:{}",
            ship_id
        ))?;
        let icing_m_h_half = *icing.get("icing_m_h_half").ok_or(format!(
            "ParsedShipData parse error: no icing_m_h_half for ship id:{}",
            ship_id
        ))?;
        let wetting_timber = ship_data.get("Wetting of deck timber").ok_or(format!(
            "ParsedShipData parse error: no wetting_timber for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        let icing_coef_v_area_full = *icing.get("icing_coef_v_area_full").ok_or(format!(
            "ParsedShipData parse error: no icing_coef_v_area_full for ship id:{}",
            ship_id
        ))?;
        let icing_coef_v_area_half = *icing.get("icing_coef_v_area_half").ok_or(format!(
            "ParsedShipData parse error: no icing_coef_v_area_half for ship id:{}",
            ship_id
        ))?;
        let icing_coef_v_area_zero = *icing.get("icing_coef_v_area_zero").ok_or(format!(
            "ParsedShipData parse error: no icing_coef_v_area_zero for ship id:{}",
            ship_id
        ))?;
        let icing_coef_v_moment_full = *icing.get("icing_coef_v_moment_full").ok_or(format!(
            "ParsedShipData parse error: no icing_coef_v_moment_full for ship id:{}",
            ship_id
        ))?;
        let icing_coef_v_moment_half = *icing.get("icing_coef_v_moment_half").ok_or(format!(
            "ParsedShipData parse error: no icing_coef_v_moment_half for ship id:{}",
            ship_id
        ))?;
        let icing_coef_v_moment_zero = *icing.get("icing_coef_v_moment_zero").ok_or(format!(
            "ParsedShipData parse error: no icing_coef_v_moment_zero for ship id:{}",
            ship_id
        ))?;
        let mut cargoes = Vec::from(cargo_src.data());
        cargoes.append(&mut bulkhead_src.data());        
        let mut compartments = Vec::from(compartments_src.data());
        compartments.append(&mut hold_compartments_src.data());
        log::info!("result parse ok");
        Self {
            ship_type,
            navigation_area,
            navigation_area_param,
            multipler_x1,
            multipler_x2,
            multipler_s,
            coefficient_k,
            coefficient_k_theta,
            length_lbp,
            length_loa,
            width,
            midship,
            overall_height,
            velocity,
            keel_area,
            water_density,
            const_mass_shift_x,
            const_mass_shift_y,
            const_mass_shift_z,
            draught_min,
            moulded_depth,
            icing_stab,
            icing_timber_stab,
            icing_m_timber,
            icing_m_v_full,
            icing_m_v_half,
            icing_m_h_full,
            icing_m_h_half,
            wetting_timber,
            icing_coef_v_area_full,
            icing_coef_v_area_half,
            icing_coef_v_area_zero,
            icing_coef_v_moment_full,
            icing_coef_v_moment_half,
            icing_coef_v_moment_zero,
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
            draft_mark: draft_mark.draft_data(),
            load_line: load_line.load_line_data(),
            screw: screw.data(),
            cargoes,
            compartments,
            load_constants: load_constant_src.data(),
            area_h_stab: area_h_stab.data(),
            area_h_str: area_h_str.data(),
            area_v_stab,
            area_v_str: area_v_str.data(),
        }
        .check()
    }
}
