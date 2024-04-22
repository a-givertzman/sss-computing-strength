//! Палубный груз
use crate::math::*;

/// Палубный груз, имеет площадь и парусность
pub struct DeskLoad {
    /// границы груза 
    bound_x: Bound,
    bound_y: Bound,
    bound_z: Bound,
    /// Площадь парусности
    windage_area: Option<f64>,
    /// Смещение центра парусности
    windage_shift: Option<Position>,
}

#[allow(dead_code)]
impl DeskLoad {
    /// Основной конструктор
    /// * bound_x - границы груза вдоль продольной оси
    /// * bound_y - границы груза вдоль поперечной оси
    /// * bound_z - границы груза вдоль вертикальной оси
    /// * windage_shift - Смещение центра парусности
    pub fn new(
        bound_x: Bound,
        bound_y: Bound,
        bound_z: Bound,
        windage_area: Option<f64>,
        windage_shift: Option<Position>,
    ) -> Self {
        Self {
            bound_x,
            bound_y,
            bound_z,
            windage_area,
            windage_shift,
        }
    }
}
///
impl DeskLoad {
    /// Парусность попадающая в Bound или вся если Bound отсутствует
    pub fn windage_area(&self, bound: Option<Bound>) -> f64 {
        self.bound_x.part_ratio(&bound.unwrap_or(self.bound_x)) * 
        self.windage_area.unwrap_or(self.bound_x.length()*self.bound_z.length())
    }
    /// Смещение центра парусности
    pub fn shift(&self) -> Position {
        if let Some(windage_shift) = self.windage_shift.clone() {
            windage_shift
        } else {
            Position::new(self.bound_x.center(), self.bound_y.center(), self.bound_y.center(),)
        }
    }
    /// Площадь горизонтальной поверхности, м^2
    pub fn horizontal_area(&self, bound: Option<Bound>) -> f64 {
        self.bound_x.part_ratio(&bound.unwrap_or(self.bound_x)) *
        self.bound_y.length()
    }
    /// Высота груза, м
    pub fn height(&self) -> f64 {
        self.bound_z.length()
    }
}
