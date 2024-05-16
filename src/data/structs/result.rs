//! Структуры для преобразования данных из формата данных DB
//! в формат пригодный для создания объектов.

use std::collections::{HashMap, HashSet};

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
    /// Длинна корпуса судна
    pub length: f64,
    /// Ширина корпуса судна
    pub breadth: f64,
    /// Эксплуатационная скорость судна, m/s
    pub velocity: f64,
    /// Суммарная масса судна
    //pub mass: f64,
    /// Объемное водоизмещение
    //pub volume: f64,
    /// Cуммарная габаритная площадь скуловых килей,
    /// либо площадь боковой проекции брускового киля
    pub keel_area: Option<f64>,
    /// разбиение на шпации - фреймы
    pub bounds: Vec<(f64, f64,)>,
    /// плотность воды
    pub water_density: f64,
    /// отстояние центра тяжести постоянной массы судна по x  
    pub const_mass_shift_x: f64,
    /// отстояние центра тяжести постоянной массы судна по y
    pub const_mass_shift_y: f64,
    /// отстояние центра тяжести постоянной массы судна по z
    pub const_mass_shift_z: f64,
    /// Суммарная площадь парусности
    pub windage_area: f64,
    /// Отстояние центра парусности по x 
    pub windage_shift_x: f64,
    /// Отстояние центра парусности по z 
    pub windage_shift_z: f64,
    /// Минимальная осадка, м
    pub draught_min: f64,
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
    /// Отстояние по вертикали центра площади проекции подводной части корпуса
    pub volume_shift: Vec<(f64, f64)>,
    /// кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f64, f64)>,
    /// кривая поперечного метацентрического радиуса 
    pub rad_cross: Vec<(f64, f64)>,
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
    /// Практические шпангоуты судна
    pub physical_frame: HashMap<i32, f64>,
    /// Теоретические шпангоуты судна
    pub theoretical_frame: Vec<ParsedFrameData>,
    /// Постоянный груз, приходящийся на шпацию
    pub load_constant: LoadConstantArray,
    /// Нагрузка судна без жидких грузов    
    pub load_spaces: Vec<ParsedLoadSpaceData>,
    /// Нагрузка судна, жидкие грузы
    pub tanks: Vec<ParsedTankData>,
    /// Площадь горизонтальных поверхностей для остойчивости
    pub area_h_stab: Vec<HStabAreaData>,
    /// Площадь горизонтальных поверхностей для прочности
    pub area_h_str: Vec<ParsedHStrArea>,
    /// Площадь поверхности парусности
    pub area_v: Vec<ParsedVerticalArea>,
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
        ship_id: usize,
        ship_data: ShipArray,
        bounds: ComputedFrameDataArray,
        center_waterline: CenterWaterlineArray,
        waterline_length: WaterlineLengthArray,
        waterline_breadth: WaterlineBreadthArray,
        volume_shift: VolumeShiftArray,
        rad_long: RadLongDataArray,
        rad_cross: RadCrossDataArray,
        mean_draught: MeanDraughtDataArray,
        center_draught_shift: CenterDraughtShiftDataArray,
        pantocaren: PantocarenDataArray,
        flooding_angle: FloodingAngleDataArray,
        entry_angle: EntryAngleDataArray,
        delta_windage_area: DeltaWindageAreaDataArray,
        delta_windage_moment: DeltaWindageMomentDataArray,
        physical_frame_src: FrameDataArray,    
        theoretical_frame_src: FrameDataArray,
        frame_area: FrameAreaData,
        load_constant: LoadConstantArray,
        load_spaces_src: LoadSpaceArray,
        tank_data: TankDataArray,
        tank_centetr_volume: CenterVolumeData,
        tanks_free_moment_inertia: FreeMomentInertiaData,
        area_h_stab: HStabAreaDataArray,
        area_h_str: HStrAreaDataArray,
        area_v_src: VerticalAreaDataArray,
    ) -> Result<Self, Error> {
        log::info!("result parse begin");
        let ship_data = ship_data.data();
        let ship_length = ship_data.get("length").ok_or(format!(
            "ParsedShipData parse error: no length for ship id:{}",
            ship_id
        ))?.0.parse::<f64>()?;
        if ship_length <= 0. {
            panic!("Ship length parse error: ship_length <= 0.",);
        }

        let mut physical_frame = HashMap::new();        
        for (index, map) in physical_frame_src.data() {
            let value = map.get("x").ok_or(format!(
                "physical_frame parse error: no x for frame index:{}",
                index
            ))?;
            physical_frame.insert(index, *value);
        }
        // Два варианта задания распределения по х - координата или физический шпангоут.
        // Если тип шпангоут, то находим и подставляем его координату
        // Координата шпангоута задана относительно кормы, считаем ее относительно центра
        let bound_x = |value: &f64, value_type: &str| -> Result<f64, Error> { 
            Ok(if value_type == "frame" {
                *physical_frame.get(&(*value as i32))
                    .ok_or(format!(
                        "load_spaces parse error: no physical_frame for value:{}",
                        value
                    ))? - ship_length/2.
            } else {
                *value 
            }) 
        };

        let mut theoretical_frame = Vec::new();        
        for (index, map) in theoretical_frame_src.data() {
            theoretical_frame.push(ParsedFrameData {
                index,
                x: *map.get("x").ok_or(format!(
                    "ParsedShipData parse error: no x for frame index:{}",
                    index
                ))?,
                delta_x: *map.get("delta_x").ok_or(format!(
                    "ParsedShipData parse error: no delta_x for frame index:{}",
                    index
                ))?,
                immersion_area: frame_area.get(index).ok_or(format!(
                    "ParsedShipData parse error: no immersion_area for frame index:{}",
                    index
                ))?.to_vec(),
            });
        }
        theoretical_frame.sort_by(|a, b| a.index.cmp(&b.index));
        
        let mut load_spaces = Vec::new();
        for load_space in load_spaces_src.data() {
            load_spaces.push(ParsedLoadSpaceData {
                name: load_space.name,
                mass: load_space.mass.unwrap_or(0.),
                bound_x: ( 
                    bound_x(&load_space.bound_x1, &load_space.bound_type)?, 
                    bound_x(&load_space.bound_x2, &load_space.bound_type)?, 
                ),
                bound_y: None,
                bound_z: None,
                mass_shift: if load_space.mass_shift_x.is_some() && 
                                load_space.mass_shift_y.is_some() &&
                                load_space.mass_shift_z.is_some() { Some((
                                        load_space.mass_shift_x.expect("ParsedShipData parse error: no mass_shift_x"),
                                        load_space.mass_shift_y.expect("ParsedShipData parse error: no mass_shift_y"),
                                        load_space.mass_shift_z.expect("ParsedShipData parse error: no mass_shift_z"),
                ))} else {
                    None
                },
                m_f_s_y: load_space.m_f_s_y,
                m_f_s_x: load_space.m_f_s_x,
                windage_area:  None,
                windage_shift: None,
            });
        }

        let mut tanks = Vec::new();
        for (tank_id, map) in tank_data.data() {
            tanks.push(ParsedTankData {
                density: *map.get("density").ok_or(format!(
                    "ParsedShipData parse error: no density for tanks id:{}",
                    tank_id
                ))?,
                volume: *map.get("volume").ok_or(format!(
                    "ParsedShipData parse error: no volume for tanks id:{}",
                    tank_id
                ))?,
                bound: (
                    *map.get("bound_x1").ok_or(format!(
                        "ParsedShipData parse error: no bound_x1 for tanks id:{}",
                        tank_id
                    ))?,
                    *map.get("bound_x2").ok_or(format!(
                        "ParsedShipData parse error: no bound_x2 for tanks id:{}",
                        tank_id
                    ))?,
                ),
                center_x: tank_centetr_volume.x(tank_id).ok_or(format!(
                    "ParsedShipData parse error: no center_x for tanks id:{}",
                    tank_id
                ))?,
                center_y: tank_centetr_volume.y(tank_id).ok_or(format!(
                    "ParsedShipData parse error: no center_y for tanks id:{}",
                    tank_id
                ))?,
                center_z: tank_centetr_volume.z(tank_id).ok_or(format!(
                    "ParsedShipData parse error: no center_z for tanks id:{}",
                    tank_id
                ))?,
                free_surf_inertia_x: tanks_free_moment_inertia.x(tank_id).ok_or(format!(
                    "ParsedShipData parse error: no free_surf_inertia_x for tanks id:{}",
                    tank_id
                ))?,
                free_surf_inertia_y: tanks_free_moment_inertia.y(tank_id).ok_or(format!(
                    "ParsedShipData parse error: no free_surf_inertia_y for tanks id:{}",
                    tank_id
                ))?,
            });
        }

        let area_h_str = area_h_str.data().iter().map(|src_data| {
            Ok(ParsedHStrArea {
                value: src_data.value,
                bound_x: ( 
                    bound_x(&src_data.bound_x1, &src_data.bound_type)?, 
                    bound_x(&src_data.bound_x2, &src_data.bound_type)?, 
                ),
            }) 
        }).collect::<Result<Vec<ParsedHStrArea>, Error>>()?;

        let area_v = area_v_src.data().iter().map(|src_data| {
            // Два варианта задания распределения по х - координата или физический шпангоут.
            // Если тип шпангоут, то находим и подставляем его координату
            // Координата шпангоута задана относительно кормы, считаем ее относительно центра
            let bound_x = |value: &f64, value_type: &str| -> Result<f64, Error> { 
                Ok(if value_type == "frame" {
                    *physical_frame.get(&(*value as i32))
                        .ok_or(format!(
                            "load_spaces parse error: no physical_frame for area:{}",
                            src_data
                        ))? - ship_length/2.
                } else {
                    *value 
                }) 
            };

            Ok(ParsedVerticalArea {
                value: src_data.value,
                shift_z: src_data.shift_z,
                bound_x: ( 
                    bound_x(&src_data.bound_x1, &src_data.bound_type)?, 
                    bound_x(&src_data.bound_x2, &src_data.bound_type)?, 
                ),
            }) 
        }).collect::<Result<Vec<ParsedVerticalArea>, Error>>()?;

        log::info!("result parse ok");
        log::info!("result check begin");
        Self {
            ship_type: ShipType::new(&ship_data.get("ship_type").ok_or(format!(
                "ParsedShipData parse error: no ship_type for ship id:{}",
                ship_id
            ))?.0),
            navigation_area: NavigationArea::new(&ship_data.get("navigation_area").ok_or(format!(
                "ParsedShipData parse error: no navigation_area for ship id:{}",
                ship_id
            ))?.0),
            navigation_area_param,
            multipler_x1,
            multipler_x2,
            multipler_s,
            coefficient_k,
            coefficient_k_theta,
            length: ship_length,
            breadth: ship_data.get("breadth").ok_or(format!(
                "ParsedShipData parse error: no breadth for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            velocity: ship_data.get("velocity").ok_or(format!(
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
            keel_area: ship_data.get("keel_area").ok_or(format!(
                "ParsedShipData parse error: no keel_area for ship id:{}",
                ship_id
            ))?.0.parse::<f64>().ok(),
            water_density: ship_data.get("water_density").ok_or(format!(
                "ParsedShipData parse error: no water_density for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            const_mass_shift_x: ship_data.get("const_mass_shift_x").ok_or(format!(
                "ParsedShipData parse error: no const_mass_shift_x for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            const_mass_shift_y: ship_data.get("const_mass_shift_y").ok_or(format!(
                "ParsedShipData parse error: no const_mass_shift_y for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            const_mass_shift_z: ship_data.get("const_mass_shift_z").ok_or(format!(
                "ParsedShipData parse error: no const_mass_shift_z for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            windage_area: ship_data.get("windage_area").ok_or(format!(
                "ParsedShipData parse error: no windage_area for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            windage_shift_x: ship_data.get("windage_shift_x").ok_or(format!(
                "ParsedShipData parse error: no windage_shift_x for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            windage_shift_z: ship_data.get("windage_shift_z").ok_or(format!(
                "ParsedShipData parse error: no windage_shift_z for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            draught_min: ship_data.get("draught_min").ok_or(format!(
                "ParsedShipData parse error: no draught_min for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_stab: ship_data.get("icing_stab").ok_or(format!(
                "ParsedShipData parse error: no icing_stab for ship id:{}",
                ship_id
            ))?.0.clone(),
            icing_m_timber: ship_data.get("icing_m_timber").ok_or(format!(
                "ParsedShipData parse error: no icing_m_timber for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_m_v_full: ship_data.get("icing_m_v_full").ok_or(format!(
                "ParsedShipData parse error: no icing_m_v_full for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_m_v_half: ship_data.get("icing_m_v_half").ok_or(format!(
                "ParsedShipData parse error: no icing_m_v_half for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_m_h_full: ship_data.get("icing_m_h_full").ok_or(format!(
                "ParsedShipData parse error: no icing_m_h_full for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_m_h_half: ship_data.get("icing_m_h_half").ok_or(format!(
                "ParsedShipData parse error: no icing_m_h_half for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_coef_v_area_full: ship_data.get("icing_coef_v_area_full").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_area_full for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_coef_v_area_half: ship_data.get("icing_coef_v_area_half").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_area_half for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_coef_v_area_zero: ship_data.get("icing_coef_v_area_zero").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_area_zero for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_coef_v_moment_full: ship_data.get("icing_coef_v_moment_full").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_moment_full for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_coef_v_moment_half: ship_data.get("icing_coef_v_moment_half").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_moment_half for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_coef_v_moment_zero: ship_data.get("icing_coef_v_moment_zero").ok_or(format!(
                "ParsedShipData parse error: no icing_coef_v_moment_zero for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            bounds: bounds.data(),
            center_waterline: center_waterline.data(),
            waterline_length: waterline_length.data(),
            waterline_breadth: waterline_breadth.data(),
            volume_shift: volume_shift.data(),
            rad_long: rad_long.data(),
            rad_cross: rad_cross.data(),
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
            physical_frame,
            theoretical_frame,
            load_constant,
            load_spaces,
            tanks,
            area_h_stab: area_h_stab.data(),
            area_h_str,
            area_v,
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
        if self.length <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: length must be positive {}",
                self.length
            )));
        }
        if self.breadth <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: breadth must be positive {}",
                self.breadth
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
                "Error check ParsedShipData: value of volume must be positive {}",
                self.draught_min
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
        if self.rad_cross.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of rad_cross's points {}",
                self.rad_cross.len()
            )));
        }
        if self.mean_draught.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of mean_draught's points {}", self.mean_draught.len())));
        }
        if self.center_draught_shift_x.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of center_draught_shift_x's points {}", self.center_draught_shift_x.len())));
        }
        if self.center_draught_shift_y.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of center_draught_shift_y's points {}", self.center_draught_shift_y.len())));
        }
        if self.center_draught_shift_z.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of center_draught_shift_z's points {}", self.center_draught_shift_z.len())));
        }
        if self.pantocaren.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of pantocaren's points {}", self.pantocaren.len())));
        }
        if let Some((draught, _)) = self.pantocaren.iter().find(|(draught, _)| *draught < 0. ) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught in pantocaren is negative!, {}",
                draught
            )));
        }
        if self.flooding_angle.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of points {}", self.flooding_angle.len())));
        }
        if let Some((key, value)) = self.flooding_angle.iter().find(|(key, value)| *key < 0. || *value < 0. ) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught or angle in flooding_angle is negative!, draught{key}, angle:{value}")));
        }
        if self.entry_angle.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of points {}", self.flooding_angle.len())));
        }
        if self.delta_windage_area.len() <= 1 {
            return Err(Error::Parameter(format!("Error check delta_windage_area: number of points {}", self.flooding_angle.len())));
        }
        if self.delta_windage_moment_x.len() <= 1 {
            return Err(Error::Parameter(format!("Error check delta_windage_moment_x: number of points{}", self.flooding_angle.len())));
        }
        if self.delta_windage_moment_z.len() <= 1 {
            return Err(Error::Parameter(format!("Error check delta_windage_moment_z: number of points{}", self.flooding_angle.len())));
        }
        if let Some((key, value)) = self.entry_angle.iter().find(|(key, value)| *key < 0. || *value < 0. ) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught or angle in entry_angle is negative!, draught{key}, angle:{value}")));
        }
        if self.theoretical_frame.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of frames must be {}",
                self.theoretical_frame.len()
            )));
        }
        if let Some(frame) = self.theoretical_frame.iter().find(|f| f.index >= self.theoretical_frame.len() as i32) {
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
        }
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
        if let Some(frame) = self.theoretical_frame.iter().find(|f| f.x < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: x of frame must be greater or equal to 0, {}",
                frame
            )));
        }
        if let Some(frame) = self.theoretical_frame.iter().find(|f| f.delta_x < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: delta_x of frame must be greater or equal to 0, {}",
                frame
            )));
        }
        if let Some(frame) = self.theoretical_frame.iter().find(|f| {
            f.immersion_area
                .iter()
                .find(|v| v.0 < 0. || v.1 < 0.)
                .is_some()
        }) {
            return Err(Error::Parameter(format!("Error check ParsedShipData: values of immersion_area in frame must be greater or equal to 0, {}", frame)));
        }
        let load_constant_data = self.load_constant.data();
        if let Some((index, value)) = load_constant_data.iter().find(|(_, value)| **value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check LoadConstantArray: mass of load_constant must be greater or equal to 0, index:{}, value:{}",
                index, value, 
            )));
        } 
        if let Some((index, _)) = load_constant_data.iter().find(|(index, _)| self.theoretical_frame.iter().find(|frame| frame.index == **index as i32 ).is_none()) {
            return Err(Error::Parameter(format!(
                "Error check LoadConstantArray: index of load_constant must be contained in frames, index:{}", index)));
        }
        if let Some(s) = self.load_spaces.iter().find(|s| s.mass < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: mass of load_space must be greater or equal to 0, {}",
                s
            )));
        }        
        if let Some(s) = self
            .load_spaces
            .iter()
            .find(|s| {
                s.bound_x.0 >= s.bound_x.1 || 
                (s.bound_y.is_some() && s.bound_y.unwrap().0 >= s.bound_y.unwrap().1) || 
                (s.bound_z.is_some() && s.bound_z.unwrap().0 >= s.bound_z.unwrap().1)
            }) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: load_space bound error! {}", s )));
        }
        if let Some(s) = self
        .load_spaces
        .iter()
        .find(|s| { 
            s.mass > 0. && s.mass_shift.is_some() && (
                s.bound_x.0 >= s.mass_shift.unwrap().0 ||
                s.mass_shift.unwrap().0 >= s.bound_x.1 ||
                (s.bound_y.is_some() && s.bound_y.unwrap().0 >= s.mass_shift.unwrap().1 )||
                (s.bound_y.is_some() && s.mass_shift.unwrap().1 >= s.bound_y.unwrap().1) ||
                (s.bound_z.is_some() && s.bound_z.unwrap().0 >= s.mass_shift.unwrap().2) ||
                (s.bound_z.is_some() && s.mass_shift.unwrap().2 >= s.bound_z.unwrap().1) 
            )
        }) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: load_space center out of bound error! {}", s )));
        }
        if let Some(tank) = self.tanks.iter().find(|t| t.density <= 0.) {
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
        log::info!("result parse ok");
        if self.area_h_stab.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of area_h_stab's points {}",
                self.area_h_stab.len()
            )));
        }
        if let Some(area) = self.area_h_stab.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of area_h_stab must be greater or equal to 0, {}",
                area
            )));
        }    
        if self.area_h_str.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of area_h's points {}",
                self.area_h_str.len()
            )));
        }
        if let Some(area) = self.area_h_str.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of area_h must be greater or equal to 0, {}",
                area
            )));
        }          
        if let Some(area) = self.area_h_str.iter().find(|f| f.bound_x.1 < f.bound_x.0) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of area_h must be greater or equal to 0, {}",
                area
            )));
        }      
        if self.area_v.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of area_v's points {}",
                self.area_v.len()
            )));
        }
        if let Some(area) = self.area_v.iter().find(|f| f.value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of area_v must be greater or equal to 0, {}",
                area
            )));
        } 
        Ok(self)
    }
}
