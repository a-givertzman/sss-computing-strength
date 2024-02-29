//! Структура для ввода данных
use serde::{de::Error, de::Unexpected, Deserialize, Serialize};

/// Общая структура для ввода данных. Содержит все данные 
/// для расчетов, при заполнении проверяет данные на корректность.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputData {
    /// Количество разбиений для расчета эпюров
    pub n_parts: f64,
    /// Плотность воды
    pub water_density: f64,
    /// Длинна корпуса судна
    pub ship_length: f64,
    /// Кривая отстояния центра тяжести ватерлинии по длине от миделя  
    pub center_waterline: Vec<(f64, f64)>,
    /// Кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f64, f64)>,
    /// Кривая средней осадки
    pub mean_draught: Vec<(f64, f64)>,
    /// Кривая отстояния центра величины погруженной части судна от центра координат
    pub center_shift: Vec<(f64, f64, f64, f64)>,
    /// Шпангоуты судна
    pub frames: Vec<FrameData>,
    /// Нагрузка судна без жидких грузов
    pub load_space: Vec<LoadSpaceData>,
    /// Нагрузка судна, жидкие грузы
    pub tanks: Vec<TankData>,
}
/// Шпангоут
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrameData {
    /// Порядковый номер шпангоута от кормы
    pub index: f64,
    /// Расстояние в продолньной плоскости от миделя
    pub delta_x: f64,
    /// Кривая погружаемой площади от осадки 
    pub immersion_area: Vec<(f64, f64)>,
}
/// Груз, конструкции корпуса, контейнер или другой твердый груз
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadSpaceData {
    /// Общая масса
    pub mass: f64,
    /// Границы груза
    pub bound: (f64, f64, f64, f64),
    /// Центер масс 
    pub center: (f64, f64, f64),
}
/// /// Цистерна
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TankData {
    /// плотность жидкости в цистерне
    pub density: f64,
    /// объем жидкости в цистерне
    pub volume: f64,
    /// границы цистерны, (x1, x2, y1, y2)
    pub bound: (f64, f64, f64, f64),
    /// кривая координат центра объема жидкости в цистерне в системе координат судна
    /// (volume, x, y, z)
    pub center: Vec<(f64, f64, f64, f64)>,
    /// кривая момента инерции площади свободной поверхности жидкости
    /// (volume, x - поперечный, y - продольный)
    pub free_surf_inertia: Vec<(f64, f64, f64)>,
}
