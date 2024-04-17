//! Нагрузка на судно: постоянный и переменный груз
use crate::{math::*, DeskLoad, ILoad, LoadMass, Tank};

/// Абстрактный груз: контейнер, трюм или бак.
/// Имеет массу и может вернуть какая его часть попадает в указанные границы

/// Груз, контенер, трюм и т.п. твердый груз, имеет границы, центр масс и значение
pub struct LoadSpace {
    /// Масса груза
    mass: Option<LoadMass>,
    /// Палубный груз
    desk: Option<DeskLoad>,
    /// Цистерна с жидкостью
    tank: Option<Tank>,
}

#[allow(dead_code)]
impl LoadSpace {
    /// Основной конструктор
    /// * mass - Масса груза
    /// * desk - Палубный груз
    /// * tank - Цистерна с жидкостью
    pub fn new(
        mass: Option<LoadMass>,
        desk: Option<DeskLoad>,
        tank: Option<Tank>,
    ) -> Self {
        Self {
            mass,
            desk,
            tank,
        }
    }
    /// Дополнительный конструктор
    /// * mass - Общая масса груза
    /// * mass_shift - Смещение центра масс
    /// * bound_x - границы груза вдоль продольной оси
    /// * bound_y - границы груза вдоль поперечной оси
    /// * bound_z - границы груза вдоль вертикальной оси
    /// * windage_area - Площадь парусности
    /// * windage_shift - Смещение центра парусности
    /// * m_f_s_y - Продольный момент свободной поверхности жидкости
    /// * m_f_s_x - Поперечный момент свободной поверхности жидкости
    pub fn from(
        mass: f64,
        mass_shift: Option<Position>,
        bound_x: (f64, f64),
        bound_y: Option<(f64, f64)>,
        bound_z: Option<(f64, f64)>,
        windage_area: Option<f64>,
        windage_shift: Option<Position>,
        m_f_s_y: Option<f64>,
        m_f_s_x: Option<f64>,
    ) -> Self {
        let mass = if mass != 0. {
            Some(LoadMass::from(
                mass,
                mass_shift,
                bound_x,
                bound_y,
                bound_z,
            ))
        } else {
            None
        };
        let desk = if bound_y.is_some() && bound_z.is_some()  {
            Some(DeskLoad::new(
                Bound::from(bound_x),
                Bound::from(bound_y.unwrap()),
                Bound::from(bound_z.unwrap()),
                windage_area,
                windage_shift,
            ))
        } else {
            None
        };
        let tank = if m_f_s_y.is_some() && m_f_s_x.is_some() {
            Some(Tank::new(
            // TODO
          //      density,
           //     volume,
           //     center,
           //     free_surf_inertia,
                m_f_s_y,
                m_f_s_x,
            ))
        } else {
            None
        };
        Self::new(
            mass,
            desk,
            tank,
        )
    }
}
///
impl ILoad for LoadSpace {
    ///
    fn mass(&self, bound: Option<Bound>) -> f64 {
        self.mass.as_ref().map(|mass| mass.value(bound)).unwrap_or(0.)
    }
    ///
    fn mass_shift(&self) -> Position {
        if let Some(mass) = &self.mass {
            return mass.shift();
        }

        if let Some(desk) = &self.desk {
            return desk.shift();
        } 
        //TODO
    /*    if let Some(tank) = self.tank {
            return tank.shift();
        } 
*/
        Position::new(0., 0., 0.)
    }
    /// Парусность попадающая в Bound или вся если Bound отсутствует
    fn windage_area(&self, bound: Option<Bound>) -> f64 {
        self.desk.as_ref().map(|desk| desk.windage_area(bound)).unwrap_or(0.)
    }
    /// Смещение центра парусности
    fn windage_shift(&self) -> Position {
        self.desk.as_ref().map(|desk| desk.shift()).unwrap_or(Position::new(0., 0., 0.,))
    }
    /// Площадь горизонтальной поверхности, м^2
    fn horizontal_area(&self, bound: Option<Bound>) -> f64 {
        self.desk.as_ref().map(|desk| desk.horizontal_area(bound)).unwrap_or(0.)
    }
    /// Высота груза, м
    fn height(&self) -> f64 {
        self.desk.as_ref().map(|desk| desk.height()).unwrap_or(0.)
    }
    /// момент свободной поверхности
    fn moment_surface(&self) -> SurfaceMoment {
        self.tank.as_ref().map(|tank| tank.moment_surface()).unwrap_or(SurfaceMoment::new(0., 0.,))
    }
}
