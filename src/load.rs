//! Нагрузка на судно: постоянный и переменный груз
use crate::math::*;

/// Абстрактный груз: контейнер, трюм или бак.
/// Имеет массу и может вернуть какая его часть попадает в указанные границы
pub trait ILoad {
    /// Смещение центра масс груза
    fn mass_shift(&self) -> Position;
    /// масса груза
    fn mass(&self, bound: Option<Bound>) -> f64;
    /// момент массы
    fn moment_mass(&self) -> Moment {
        Moment::from_pos(self.mass_shift(), self.mass(None))
    }
    /// момент свободной поверхности
    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::new(0., 0.)
    }
    /// Площадь парусности
    fn windage_area(&self) -> f64 {
        0.
    }
    /// Смещение центра парусности
    fn windage_shift(&self) -> Position {
        Position::new(0., 0., 0.)
    }
}

/// Груз, контенер, трюм и т.п. твердый груз, имеет границы, центр масс и значение
pub struct LoadSpace {
    /// общая масса
    mass: f64,
    /// границы груза
    bound: Bound,
    /// Смещение центра масс
    mass_shift: Position,
    /// TODO: удалить и перенести в цистерны: Продольный момент свободной поверхности жидкости
    m_f_s_y: f64,
    /// TODO: удалить и перенести в цистерны: Поперечный момент свободной поверхности жидкости
    m_f_s_x: f64,
    /// Площадь парусности
    windage_area: f64,
    /// Смещение центра парусности
    windage_shift: Position,
}

#[allow(dead_code)]
impl LoadSpace {
    ///
    pub fn new(
        mass: f64,
        bound: Bound,
        mass_shift: Position,
        m_f_s_y: f64,
        m_f_s_x: f64,
        windage_area: f64,
        windage_shift: Position,
    ) -> Self {
        assert!(
            bound.start() < mass_shift.x(),
            "bound.start {} < pos.x {}",
            bound.start(),
            mass_shift.x()
        );
        assert!(
            bound.end() > mass_shift.x(),
            "bound.end {} > pos.x {}",
            bound.end(),
            mass_shift.x()
        );
        assert!(windage_area >= 0., "windage_area {windage_area} >= 0",);
        Self {
            bound,
            mass_shift,
            mass,
            m_f_s_y,
            m_f_s_x,
            windage_area,
            windage_shift,
        }
    }
}

impl ILoad for LoadSpace {
    fn mass(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound.part_ratio(&bound) * self.mass
        } else {
            self.mass
        }
    }
    ///
    fn mass_shift(&self) -> Position {
        self.mass_shift.clone()
    }
    /// момент свободной поверхности - TODO - удалить
    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::new(self.m_f_s_x, self.m_f_s_y)
    }
    /// Площадь парусности
    fn windage_area(&self) -> f64 {
        self.windage_area
    }
    /// Смещение центра парусности
    fn windage_shift(&self) -> Position {
        self.windage_shift.clone()
    }
}
