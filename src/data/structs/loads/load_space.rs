//! Промежуточные структуры для serde_json для парсинга данных груза
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::data::structs::DataArray;

/// Груз, конструкции корпуса, контейнер или другой твердый груз
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadSpaceData {
    /// ID груза
    pub space_id: usize,
    /// Параметр в виде текста
    pub key: String,
    /// Величина параметра
    pub value: String,
    /// Тип параметра
    pub value_type: String,
}
///
impl std::fmt::Display for LoadSpaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(index:{}, key:{}, value:{} type:{})",
            self.space_id,
            self.key,
            self.value,
            self.value_type,
        )
    }
}
/// Массив данных по грузам
pub type LoadSpaceArray = DataArray<LoadSpaceData>;
///
impl LoadSpaceArray {
    /// Преобразование и возвращает данные в виде мапы id/данные груза
    pub fn data(self) -> HashMap<usize, HashMap<String, (String, String)>> {
        let mut map: HashMap<usize, HashMap<String, (String, String)>> = HashMap::new();
        self.data.into_iter().for_each(|v| {
            if let Some(sub_map) = map.get_mut(&v.space_id) {
                sub_map.insert(v.key.clone(), (v.value, v.value_type));
            } else {
                map.insert(v.space_id, HashMap::from([(v.key.clone(), (v.value, v.value_type))]));       
            }
        });
        map
    }
}

/// Груз
#[derive(Debug)]
pub struct ParsedLoadSpaceData {
    /// Название 
    pub name: String, 
    /// Общая масса
    pub mass: f64,
    /// Границы груза
    pub bound_x: (f64, f64),
    pub bound_y: Option<(f64, f64)>,
    pub bound_z: Option<(f64, f64)>,
    /// Центр масс
    pub mass_shift: Option<(f64, f64, f64)>,
    /// Продольный момент свободной поверхности жидкости
    pub m_f_s_y: Option<f64>,
    /// Поперечный момент инерции свободной поверхности жидкости в цистерне
    pub m_f_s_x: Option<f64>,    
    /// Площадь парусности
    pub windage_area: Option<f64>,
    /// Центр парусности
    pub windage_shift: Option<(f64, f64)>,
}
///
impl std::fmt::Display for ParsedLoadSpaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(name:{}, mass:{} bound_x:{:?}, bound_y:{:?} bound_z:{:?} mass_shift:({} {} {}) m_f_s_y:{:?}, m_f_s_x:{:?} windage_area:{} windage_shift:(x:{}, z:{}))",
            self.name,
            self.mass,
            self.bound_x,
            self.bound_y,
            self.bound_z,
            self.mass_shift.unwrap_or((0.,0.,0.)).0,
            self.mass_shift.unwrap_or((0.,0.,0.)).1,
            self.mass_shift.unwrap_or((0.,0.,0.)).2,
            self.m_f_s_y,
            self.m_f_s_x,
            self.windage_area.unwrap_or(0.),
            self.windage_shift.unwrap_or((0.,0.)).0,
            self.windage_shift.unwrap_or((0.,0.)).1,
        )
    }
}