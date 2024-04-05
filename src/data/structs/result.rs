//! Структуры для преобразования данных из формата данных DB
//! в формат пригодный для создания объектов.

use std::collections::HashSet;

use crate::error::Error;

use super::*;

/// Шпангоут
#[derive(Debug)]
pub struct ParsedFrameData {
    /// Порядковый номер шпангоута от кормы
    pub index: usize,
    /// Координата по х относительно кормы
    pub x: f64,
    /// Смещение относительно предыдущего шпангоута
    pub delta_x: f64,
    /// кривая погружаемой площади
    pub immersion_area: Vec<(f64, f64)>,
}
///
impl std::fmt::Display for ParsedFrameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParsedFrameData(index:{}, x:{}, delta_x:{}, immersion_area.len:{} )",
            self.index,
            self.x,
            self.delta_x,
            self.immersion_area.len(),
        )
    }
}
/// Груз
#[derive(Debug)]
pub struct ParsedLoadSpaceData {
    /// Общая масса
    pub mass: f64,
    /// Границы груза
    pub bound: (f64, f64),
    /// Центр масс
    pub mass_shift: (f64, f64, f64),
    /// Продольный момент свободной поверхности жидкости
    pub m_f_s_y: f64,
    /// Поперечный момент инерции свободной поверхности жидкости в цистерне
    pub m_f_s_x: f64,    
    /// Площадь парусности
    pub windage_area: f64,
    /// Центр парусности
    pub windage_shift: (f64, f64),
}
///
impl std::fmt::Display for ParsedLoadSpaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(mass:{} bound:(x1:{}, x2:{}) mass_shift:({} {} {}) m_f_s_y:{}, m_f_s_x:{} windage_area:{} windage_shift:(x:{}, z:{}))",
            self.mass,
            self.bound.0,
            self.bound.1,
            self.mass_shift.0,
            self.mass_shift.1,
            self.mass_shift.2,
            self.m_f_s_y,
            self.m_f_s_x,
            self.windage_area,
            self.windage_shift.0,
            self.windage_shift.1,
        )
    }
}
/// Цистерна
#[derive(Debug)]
pub struct ParsedTankData {
    /// плотность жидкости в цистерне
    pub density: f64,
    /// объем жидкости в цистерне
    pub volume: f64,
    /// границы цистерны, (x1, x2)
    pub bound: (f64, f64),
    /// кривая координат центра объема жидкости в цистерне в системе координат судна по x
    pub center_x: Vec<(f64, f64)>,
    /// кривая координат центра объема жидкости в цистерне в системе координат судна по y
    pub center_y: Vec<(f64, f64)>,
    /// кривая координат центра объема жидкости в цистерне в системе координат судна по z
    pub center_z: Vec<(f64, f64)>,
    /// кривая момента инерции площади свободной поверхности жидкости volume, x - поперечный
    pub free_surf_inertia_x: Vec<(f64, f64)>,
    /// кривая момента инерции площади свободной поверхности жидкостиvolume, y - продольный
    pub free_surf_inertia_y: Vec<(f64, f64)>,
}
///
impl std::fmt::Display for ParsedTankData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(density:{}, volume:{}, bound:(x1:{}, x2:{}), center_x.len:{}, center_y.len:{}, center_z.len:{}, inertia_x.len:{}, inertia_y.len:{}) )",
            self.density,
            self.volume,
            self.bound.0,
            self.bound.1,
            self.center_x.len(),
            self.center_y.len(),
            self.center_z.len(),
            self.free_surf_inertia_x.len(),
            self.free_surf_inertia_y.len(),
        )
    }
}
/// Общая структура для ввода данных. Содержит все данные
/// для расчетов.
#[derive(Debug)]
pub struct ParsedShipData {
    /// Параметры района плавания судна  
    pub navigation_area_name: String,
    /// Параметры района плавания судна  
    pub navigation_area_param: NavigationAreaArray,
    /// Тип обледенения для расчета парусности
    pub icing_stab: String,
    /// Безразмерный множитель Х_1 для расчета качки, Табл. 2.1.5.1-1
    pub multipler_x1: MultiplerX1Array,
    /// Безразмерный множитель Х_2 для расчета качки, Табл. 2.1.5.1-2
    pub multipler_x2: MultiplerX2Array,
    /// Безразмерный множитель S для расчета качки, Табл. 2.1.5.1-3
    pub multipler_s: MultiplerSArray,
    /// Коэффициент k для судов, имеющих скуловые кили или 
    /// брусковый киль для расчета качки, Табл. 2.1.5.2
    pub coefficient_k: CoefficientKArray,
    /// Длинна корпуса судна
    pub length: f64,
    /// Ширина корпуса судна
    pub breadth: f64,
    /// Суммарная масса судна
    pub mass: f64,
    /// Объемное водоизмещение
    pub volume: f64,
    /// Cуммарная габаритная площадь скуловых килей,
    /// либо площадь боковой проекции брускового киля
    pub keel_area: Option<f64>,
    /// разбиение на шпации - фреймы
    pub bounds: Vec<(usize, f64)>,
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
    /// Разница в площадях парусности для осадки по ЛГВЛ и осадки, м²
    pub delta_windage_area: f64,
    /// Разница в статических моментах относительно миделя и ОП по x, м
    pub delta_windage_moment_x: f64,
    /// Разница в статических моментах относительно миделя и ОП по z, м
    pub delta_windage_moment_z: f64,
    /// Осадка по ЛГВЛ
    pub draught_slw: f64,
    /// кривая отстояния центра тяжести ватерлинии по длине от миделя  
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
    /// Шпангоуты судна
    pub frames: Vec<ParsedFrameData>,
    /// Постоянный груз, приходящийся на шпацию
    pub load_constant: LoadConstantArray,
    /// Нагрузка судна без жидких грузов    
    pub load_spaces: Vec<ParsedLoadSpaceData>,
    /// Нагрузка судна, жидкие грузы
    pub tanks: Vec<ParsedTankData>,
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
        frame_src: FrameDataArray,
        frame_area: FrameAreaData,
        load_constant: LoadConstantArray,
        load_spaces_src: LoadSpaceArray,
        tank_data: TankDataArray,
        tank_centetr_volume: CenterVolumeData,
        tanks_free_moment_inertia: FreeMomentInertiaData,
    ) -> Result<Self, Error> {
        log::info!("result parse begin");
        let ship_data = ship_data.data();

        let mut frames = Vec::new();
        
        for (index, map) in frame_src.data() {
            frames.push(ParsedFrameData {
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
        frames.sort_by(|a, b| a.index.cmp(&b.index));
        let mut load_spaces = Vec::new();
        for (space_id, map) in load_spaces_src.data() {
            load_spaces.push(ParsedLoadSpaceData {
                mass: map.get("mass").ok_or(format!(
                    "ParsedShipData parse error: no mass for load_space id:{}",
                    space_id
                ))?.0.parse::<f64>()?,
                bound: (
                    map.get("bound_x1").ok_or(format!(
                        "ParsedShipData parse error: no bound_x1 for load_space id:{}",
                        space_id
                    ))?.0.parse::<f64>()?,
                    map.get("bound_x2").ok_or(format!(
                        "ParsedShipData parse error: no bound_x2 for load_space id:{}",
                        space_id
                    ))?.0.parse::<f64>()?,
                ),
                mass_shift: (
                    map.get("mass_shift_x").ok_or(format!(
                        "ParsedShipData parse error: no mass_shift_x for load_space id:{}",
                        space_id
                    ))?.0.parse::<f64>()?,
                    map.get("mass_shift_y").ok_or(format!(
                        "ParsedShipData parse error: no mass_shift_y for load_space id:{}",
                        space_id
                    ))?.0.parse::<f64>()?,
                    map.get("mass_shift_z").ok_or(format!(
                        "ParsedShipData parse error: no mass_shift_z for load_space id:{}",
                        space_id
                    ))?.0.parse::<f64>()?,
                ),
                m_f_s_y: map.get("m_f_s_y").ok_or(format!(
                    "ParsedShipData parse error: no m_f_s_y for load_space id:{}",
                    space_id
                ))?.0.parse::<f64>()?,
                m_f_s_x: map.get("m_f_s_x").ok_or(format!(
                    "ParsedShipData parse error: no m_f_s_x for load_space id:{}",
                    space_id
                ))?.0.parse::<f64>()?,
                windage_area: map.get("windage_area").ok_or(format!(
                    "ParsedShipData parse error: no windage_area for load_space id:{}",
                    space_id
                ))?.0.parse::<f64>()?,
                windage_shift: (
                    map.get("windage_shift_x").ok_or(format!(
                        "ParsedShipData parse error: no windage_shift_x for load_space id:{}",
                        space_id
                    ))?.0.parse::<f64>()?,
                    map.get("windage_shift_z").ok_or(format!(
                        "ParsedShipData parse error: no windage_shift_z for load_space id:{}",
                        space_id
                    ))?.0.parse::<f64>()?,
                ),
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
        log::info!("result parse ok");
        log::info!("result check begin");
        Self {
            navigation_area_name: ship_data.get("navigation_area").ok_or(format!(
                "ParsedShipData parse error: no navigation_area for ship id:{}",
                ship_id
            ))?.0.clone(),
            navigation_area_param,
            multipler_x1,
            multipler_x2,
            multipler_s,
            coefficient_k,
            length: ship_data.get("length").ok_or(format!(
                "ParsedShipData parse error: no length for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            breadth: ship_data.get("breadth").ok_or(format!(
                "ParsedShipData parse error: no breadth for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            mass: ship_data.get("mass").ok_or(format!(
                "ParsedShipData parse error: no mass for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            volume: ship_data.get("volume").ok_or(format!(
                "ParsedShipData parse error: no volume for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
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
            delta_windage_area: ship_data.get("delta_windage_area").ok_or(format!(
                "ParsedShipData parse error: no delta_windage_area for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            delta_windage_moment_x: ship_data.get("delta_windage_moment_x").ok_or(format!(
                "ParsedShipData parse error: no delta_windage_moment_x for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            delta_windage_moment_z: ship_data.get("delta_windage_moment_z").ok_or(format!(
                "ParsedShipData parse error: no delta_windage_moment_z for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            draught_slw: ship_data.get("draught_slw").ok_or(format!(
                "ParsedShipData parse error: no draught_slw for ship id:{}",
                ship_id
            ))?.0.parse::<f64>()?,
            icing_stab: ship_data.get("icing_stab").ok_or(format!(
                "ParsedShipData parse error: no icing_stab for ship id:{}",
                ship_id
            ))?.0.clone(),
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
            frames,
            load_constant,
            load_spaces,
            tanks,
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
        if self.mass <= 0. {
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
        }
        if self.bounds.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of bounds's points greater or equal to 2 {}",
                self.bounds.len()
            )));
        }
        if self.center_waterline.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of waterline's points greater or equal to 2 {}",
                self.center_waterline.len()
            )));
        }
        if self.rad_long.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of rad_long's points greater or equal to 2 {}",
                self.rad_long.len()
            )));
        }
        if self.rad_cross.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of rad_cross's points greater or equal to 2 {}",
                self.rad_cross.len()
            )));
        }
        if self.mean_draught.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of mean_draught's points greater or equal to 2 {}", self.mean_draught.len())));
        }
        if self.center_draught_shift_x.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of center_draught_shift_x's points greater or equal to 2 {}", self.center_draught_shift_x.len())));
        }
        if self.center_draught_shift_y.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of center_draught_shift_y's points greater or equal to 2 {}", self.center_draught_shift_y.len())));
        }
        if self.center_draught_shift_z.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of center_draught_shift_z's points greater or equal to 2 {}", self.center_draught_shift_z.len())));
        }
        if self.pantocaren.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of pantocaren's points greater or equal to 2 {}", self.pantocaren.len())));
        }
        if let Some((draught, _)) = self.pantocaren.iter().find(|(draught, _)| *draught < 0. ) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught in pantocaren is negative!, {}",
                draught
            )));
        }
        if self.flooding_angle.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of flooding_angle's points greater or equal to 2 {}", self.flooding_angle.len())));
        }
        if let Some((key, value)) = self.flooding_angle.iter().find(|(key, value)| *key < 0. || *value < 0. ) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught or angle in flooding_angle is negative!, draught{key}, angle:{value}")));
        }
        if self.entry_angle.len() <= 1 {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of entry_angle's points greater or equal to 2 {}", self.flooding_angle.len())));
        }
        if let Some((key, value)) = self.entry_angle.iter().find(|(key, value)| *key < 0. || *value < 0. ) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: draught or angle in entry_angle is negative!, draught{key}, angle:{value}")));
        }
        if self.frames.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of frames must be greater or equal to 2 {}",
                self.frames.len()
            )));
        }
        if let Some(frame) = self.frames.iter().find(|f| f.index >= self.frames.len()) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: index of frame bigger or equal then frames.len(), {}",
                frame
            )));
        }
        let qnt_unique_index = self
            .frames
            .iter()
            .map(|f| f.index)
            .collect::<HashSet<_>>()
            .len();
        if self.frames.len() != qnt_unique_index {
            return Err(Error::Parameter(format!("Error check ParsedShipData: index of frame must be unique frames:{}, unique index:{}", self.frames.len(), qnt_unique_index )));
        }
        if self
            .frames
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
        if let Some(frame) = self.frames.iter().find(|f| f.x < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: x of frame must be greater or equal to 0, {}",
                frame
            )));
        }
        if let Some(frame) = self.frames.iter().find(|f| f.delta_x < 0.) {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: delta_x of frame must be greater or equal to 0, {}",
                frame
            )));
        }
        if let Some(frame) = self.frames.iter().find(|f| {
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
        if let Some((index, _)) = load_constant_data.iter().find(|(index, _)| self.frames.iter().find(|frame| &&frame.index == index ).is_none()) {
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
            .find(|s| s.bound.0 >= s.bound.1)
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: load_space bound error! {}", s )));
        }
        if let Some(s) = self
        .load_spaces
        .iter()
        .find(|s| s.bound.0 >= s.mass_shift.0 || s.mass_shift.0 >= s.bound.1)
        {
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
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of center's points must be greater or equal to 2 {}", tank)));
        }
        if let Some(tank) = self
            .tanks
            .iter()
            .find(|t| t.free_surf_inertia_x.len() <= 1 || t.free_surf_inertia_y.len() <= 1)
        {
            return Err(Error::Parameter(format!("Error check ParsedShipData: number of free_surf_inertia's points must be greater or equal to 2 {}", tank)));
        }
        log::info!("result parse ok");
        Ok(self)
    }
}
