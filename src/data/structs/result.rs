//! Структуры для ввода данных

use crate::error::Error;

use super::{
    CenterShiftDataArray, CenterVolumeData, CenterWaterlineArray, FrameAreaData, FrameDataArray,
    FreeMomentInertiaData, LoadSpaceData, MeanDraughtDataArray, RadLongDataArray, ShipData,
    TankDataArray,
};

/// Шпангоут
#[derive(Debug)]
pub struct ParsedFrameData {
    /// Порядковый номер шпангоута от кормы
    pub index: usize,
    /// Смещение относительно предыдущего шпангоута
    pub delta_x: f64,
    /// кривая погружаемой площади
    pub immersion_area: Vec<(f64, f64)>,
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
/// Общая структура для ввода данных. Содержит все данные
/// для расчетов.
#[derive(Debug)]
pub struct ParsedShipData {
    /// разбиение на шпации - количество
    pub n_parts: usize,
    /// плотность воды
    pub water_density: f64,
    /// длинна корпуса судна
    pub ship_length: f64,
    /// кривая отстояния центра тяжести ватерлинии по длине от миделя  
    pub center_waterline: Vec<(f64, f64)>,
    /// кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f64, f64)>,
    /// кривая средней осадки
    pub mean_draught: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна по x
    pub center_shift_x: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна по y
    pub center_shift_y: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна по z
    pub center_shift_z: Vec<(f64, f64)>,
    /// Шпангоуты судна
    pub frames: Vec<ParsedFrameData>,
    /// Нагрузка судна без жидких грузов
    pub load_spaces: Vec<ParsedLoadSpaceData>,
    /// Нагрузка судна, жидкие грузы
    pub tanks: Vec<ParsedTankData>,
}
///
impl ParsedShipData {
    ///
    pub fn parse(
        ship_id: usize,
        ship_data: ShipData,
        center_vaterline: CenterWaterlineArray,
        rad_long: RadLongDataArray,
        mean_draught: MeanDraughtDataArray,
        center_shift: CenterShiftDataArray,
        frame_src: FrameDataArray,
        frame_area_src: FrameAreaData,
        load_space: LoadSpaceData,
        tank_data: TankDataArray,
        tank_centetr_volume: CenterVolumeData,
        tanks_free_moment_inertia: FreeMomentInertiaData,
    ) -> Result<Self, Error> {
        let frames = frame_src
        .data()
        .into_iter()
        .map(|(index, map)| ParsedFrameData {
            index,
            delta_x: *map.get("delta_x"),
            immersion_area: frame_area_src.get(index),
        })
        .collect();
        let load_spaces = todo!();
        let tanks = todo!();

        Ok(Self {
            n_parts: ship_data.n_parts,
            water_density: ship_data.water_density,
            ship_length: ship_data.ship_length,
            center_waterline: center_vaterline.data(),
            rad_long: rad_long.data(),
            mean_draught: mean_draught.data(),
            center_shift_x: center_shift.x(),
            center_shift_y: center_shift.y(),
            center_shift_z: center_shift.z(),
            frames,
            load_spaces,
            tanks,
        })
    }
}
