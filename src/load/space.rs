//! Нагрузка на судно: постоянный и переменный груз
use crate::{math::*, ILoad};

/// Абстрактный груз: контейнер, трюм или бак.
/// Имеет массу и может вернуть какая его часть попадает в указанные границы

/// Груз, контенер, трюм и т.п. твердый груз, имеет границы, центр масс и значение
pub struct LoadSpace {
    /// Общая масса груза
    mass: f64,
    /// Смещение центра масс
    mass_shift: Option<Position>,
    /// границы груза 
    bound_x: Bound,
    bound_y: Bound,
    bound_z: Bound,
    /// Площадь парусности
    windage_area: Option<f64>,
    /// Смещение центра парусности
    windage_shift: Option<Position>,
    /// TODO: удалить и перенести в цистерны: Продольный момент свободной поверхности жидкости
    m_f_s_y: f64,
    /// TODO: удалить и перенести в цистерны: Поперечный момент свободной поверхности жидкости
    m_f_s_x: f64,
}

#[allow(dead_code)]
impl LoadSpace {
    /// Основной конструктор
    /// * mass - Общая масса груза
    /// * mass_shift - Смещение центра масс
    /// * bound_x - границы груза вдоль продольной оси
    /// * bound_y - границы груза вдоль поперечной оси
    /// * bound_z - границы груза вдоль вертикальной оси
    /// * windage_shift - Смещение центра парусности
    /// * m_f_s_y - Продольный момент свободной поверхности жидкости
    /// * m_f_s_x - Поперечный момент свободной поверхности жидкости
    pub fn new(
        mass: f64,
        mass_shift: Option<Position>,
        bound_x: Bound,
        bound_y: Bound,
        bound_z: Bound,
        windage_area: Option<f64>,
        windage_shift: Option<Position>,
        m_f_s_y: f64,
        m_f_s_x: f64,
    ) -> Self {
        Self {
            mass,
            mass_shift,
            bound_x,
            bound_y,
            bound_z,
            windage_area,
            windage_shift,
            m_f_s_y,
            m_f_s_x,
        }
    }
}
///
impl ILoad for LoadSpace {
    ///
    fn mass(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound_x.part_ratio(&bound) * self.mass
        } else {
            self.mass
        }
    }
    ///
    fn mass_shift(&self) -> Position {
        if let Some(mass_shift) = self.mass_shift.clone() {
            mass_shift
        } else {
            Position::new(self.bound_x.center(), self.bound_y.center(),self.bound_y.center(),)
        }
    }
    /// Парусность попадающая в Bound или вся если Bound отсутствует
    fn windage_area(&self, bound: Option<Bound>) -> f64 {
        self.bound_x.part_ratio(&bound.unwrap_or(self.bound_x)) * 
        self.windage_area.unwrap_or(self.bound_x.length()*self.bound_z.length())
    }
    /// Смещение центра парусности
    fn windage_shift(&self) -> Position {
        if let Some(windage_shift) = self.windage_shift.clone() {
            windage_shift
        } else {
            Position::new(self.bound_x.center(), self.bound_y.center(), self.bound_y.center(),)
        }
    }
    /// Площадь горизонтальной поверхности, м^2
    fn horizontal_area(&self) -> f64 {
        self.bound_x.length()*self.bound_y.length()
    }
    /// Высота груза, м
    fn height(&self) -> f64 {
        self.bound_z.length()
    }
    /// момент свободной поверхности - TODO - удалить
    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::new(self.m_f_s_x, self.m_f_s_y)
    }
}
