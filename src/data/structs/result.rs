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
    pub bound: (f64, f64, f64, f64),
    /// Центер масс
    pub center: (f64, f64, f64),
}
///
impl std::fmt::Display for ParsedLoadSpaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(mass:{}, bound:(x1:{}, x2:{}, y1:{}, y2:{}), center:(x:{}, y:{}, z:{}) )",
            self.mass,
            self.bound.0,
            self.bound.1,
            self.bound.2,
            self.bound.3,
            self.center.0,
            self.center.1,
            self.center.2,
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
    /// границы цистерны, (x1, x2, y1, y2)
    pub bound: (f64, f64, f64, f64),
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
            "LoadSpaceData(density:{}, volume:{}, bound:(x1:{}, x2:{}, y1:{}, y2:{}), center_x.len:{}, center_y.len:{}, center_z.len:{}, inertia_x.len:{}, inertia_y.len:{}) )",
            self.density,
            self.volume,
            self.bound.0,
            self.bound.1,
            self.bound.2,
            self.bound.3,
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
    /// разбиение на шпации - количество
    pub n_parts: f64,
    /// плотность воды
    pub water_density: f64,
    /// длинна корпуса судна
    pub ship_length: f64,
    /// Процент запасов судна
    pub stock: f64,
    /// кривая отстояния центра тяжести ватерлинии по длине от миделя  
    pub center_waterline: Vec<(f64, f64)>,
    /// кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f64, f64)>,
    /// кривая средней осадки
    pub mean_draught: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна по x
    pub center_draught_shift_x: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна по y
    pub center_draught_shift_y: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна по z
    pub center_draught_shift_z: Vec<(f64, f64)>,
    /// Шпангоуты судна
    pub frames: Vec<ParsedFrameData>,
    /// Постоянный груз, приходящийся на шпацию
    pub load_constant: LoadConstantArray,
    /// Переменный груз, приходящийся на шпацию
    pub load_stock: LoadStockArray,
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
        ship_id: usize,
        ship_data: ShipArray,
        center_waterline: CenterWaterlineArray,
        rad_long: RadLongDataArray,
        mean_draught: MeanDraughtDataArray,
        center_draught_shift: CenterDraughtShiftDataArray,
        frame_src: FrameDataArray,
        frame_area: FrameAreaData,
        load_constant: LoadConstantArray,
        load_stock: LoadStockArray,
        load_spaces_src: LoadSpaceArray,
        tank_data: TankDataArray,
        tank_centetr_volume: CenterVolumeData,
        tanks_free_moment_inertia: FreeMomentInertiaData,
    ) -> Result<Self, Error> {
        log::debug!("result parse begin");
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
                mass: *map.get("mass").ok_or(format!(
                    "ParsedShipData parse error: no mass for load_space id:{}",
                    space_id
                ))?,
                bound: (
                    *map.get("bound_x1").ok_or(format!(
                        "ParsedShipData parse error: no bound_x1 for load_space id:{}",
                        space_id
                    ))?,
                    *map.get("bound_x2").ok_or(format!(
                        "ParsedShipData parse error: no bound_x2 for load_space id:{}",
                        space_id
                    ))?,
                    *map.get("bound_y1").ok_or(format!(
                        "ParsedShipData parse error: no bound_y1 for load_space id:{}",
                        space_id
                    ))?,
                    *map.get("bound_y2").ok_or(format!(
                        "ParsedShipData parse error: no bound_y2 for load_space id:{}",
                        space_id
                    ))?,
                ),
                center: (
                    *map.get("center_x").ok_or(format!(
                        "ParsedShipData parse error: no center_x for load_space id:{}",
                        space_id
                    ))?,
                    *map.get("center_y").ok_or(format!(
                        "ParsedShipData parse error: no center_y for load_space id:{}",
                        space_id
                    ))?,
                    *map.get("center_z").ok_or(format!(
                        "ParsedShipData parse error: no center_z for load_space id:{}",
                        space_id
                    ))?,
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
                    *map.get("bound_y1").ok_or(format!(
                        "ParsedShipData parse error: no bound_y1 for tanks id:{}",
                        tank_id
                    ))?,
                    *map.get("bound_y2").ok_or(format!(
                        "ParsedShipData parse error: no bound_y2 for tanks id:{}",
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
        log::debug!("result parse ok");
        log::debug!("result check begin");
        Self {
            n_parts: *ship_data.get("n_parts").ok_or(format!(
                "ParsedShipData parse error: no n_parts for ship id:{}",
                ship_id
            ))?,
            water_density: *ship_data.get("water_density").ok_or(format!(
                "ParsedShipData parse error: no water_density for ship id:{}",
                ship_id
            ))?,
            ship_length: *ship_data.get("ship_length").ok_or(format!(
                "ParsedShipData parse error: no ship_length for ship id:{}",
                ship_id
            ))?,
            stock: *ship_data.get("stock").ok_or(format!(
                "ParsedShipData parse error: no ship_length for ship id:{}",
                ship_id
            ))?,
            center_waterline: center_waterline.data(),
            rad_long: rad_long.data(),
            mean_draught: mean_draught.data(),
            center_draught_shift_x: center_draught_shift.x(),
            center_draught_shift_y: center_draught_shift.y(),
            center_draught_shift_z: center_draught_shift.z(),
            frames,
            load_constant,
            load_stock,
            load_spaces,
            tanks,
        }
        .check()
    }
    /// Проверка данных на корректность
    fn check(self) -> Result<Self, Error> {
        if self.ship_length <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of ship's length must be positive {}",
                self.ship_length
            )));
        }
        if self.n_parts <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of frames must be positive {}",
                self.n_parts
            )));
        }
        if self.water_density <= 0. {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: value of water density must be positive {}",
                self.water_density
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
        if self.frames.len() <= 1 {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: number of frames must be greater or equal to 2 {}",
                self.center_draught_shift_z.len()
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
        let load_stock_data = self.load_stock.data(self.stock);
        if let Some((index, value)) = load_stock_data.iter().find(|(_, value)| **value < 0.) {
            return Err(Error::Parameter(format!(
                "Error check LoadStockArray: mass of load_stock must be greater or equal to 0, index:{}, value:{}",
                index, value, 
            )));
        }
        if let Some((index, _)) = load_stock_data.iter().find(|(index, _)| self.frames.iter().find(|frame| &&frame.index == index ).is_none()) {
            return Err(Error::Parameter(format!(
                "Error check LoadStockArray: index of load_stock must be contained in frames, index:{}", index)));
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
            .find(|s| s.bound.0 >= s.bound.1 || s.bound.2 >= s.bound.3)
        {
            return Err(Error::Parameter(format!(
                "Error check ParsedShipData: load_space Bound error! {}", s )));
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
        log::debug!("result parse ok");
        Ok(self)
    }
}
