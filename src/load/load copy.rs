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
    /// * mass Смещение центра масс
    /// * mass_shift: Position,
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
        bound_y: Option<Bound>,
        bound_z: Option<Bound>,
        windage_area: Option<f64>,
        windage_shift: Option<Position>,
        m_f_s_y: f64,
        m_f_s_x: f64,
    ) -> Self {
        if let Some(mass_shift) = mass_shift {
            assert!(
                bound_x.start() <= mass_shift.x(),
                "bound_x.start {} <= pos.x {}",
                bound_x.start(), mass_shift.x()
            );
            assert!(
                bound_x.end() >= mass_shift.x(),
                "bound_x.end {} >= pos.x {}",
                bound_x.end(), mass_shift.x()
            );
            assert!(
                bound_y.start() <= mass_shift.y(),
                "bound_y.start {} <= pos.y {}",
                bound_y.start(), mass_shift.y()
            );
            assert!(
                bound_y.end() >= mass_shift.y(),
                "bound_y.end {} >= pos.y {}",
                bound_y.end(), mass_shift.y()
            );
            assert!(
                bound_z.start() <= mass_shift.z(),
                "bound_z.start {} <= pos.z {}",
                bound_z.start(), mass_shift.z()
            );
            assert!(
                bound_z.end() >= mass_shift.z(),
                "bound_z.end {} >= pos.z {}",
                bound_z.end(), mass_shift.z()
            );
        }
        if let Some(windage_shift) = windage_shift {
            assert!(
                bound_x.start() <= windage_shift.x(),
                "bound_x.start {} <= pos.x {}",
                bound_x.start(), windage_shift.x()
            );
            assert!(
                bound_x.end() >= windage_shift.x(),
                "bound_x.end {} >= pos.x {}",
                bound_x.end(), windage_shift.x()
            );
            assert!(
                bound_y.start() <= windage_shift.y(),
                "bound_y.start {} <= pos.y {}",
                bound_y.start(), windage_shift.y()
            );
            assert!(
                bound_y.end() >= windage_shift.y(),
                "bound_y.end {} >= pos.y {}",
                bound_y.end(), windage_shift.y()
            );
            assert!(
                bound_z.start() <= windage_shift.z(),
                "bound_z.start {} <= pos.z {}",
                bound_z.start(), windage_shift.z()
            );
            assert!(
                bound_z.end() >= windage_shift.z(),
                "bound_z.end {} >= pos.z {}",
                bound_z.end(), windage_shift.z()
            );
        }
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
    ///
    fn calculate(&mut self) {
        if self.mass_shift.is_none() {
            self.mass_shift = Some(Position::new(self.bound_x.center(), self.bound_y.center(),self.bound_y.center(),) );
        }
        if self.windage_area.is_none() {
            self.windage_area = Some(self.bound_x.length()*self.bound_z.length());
        }        
        if self.windage_shift.is_none() {
            self.windage_shift = Some(Position::new(self.bound_x.center(), self.bound_y.center(),self.bound_y.center(),) );
        } 
    }
}

impl ILoad for LoadSpace {
    fn mass(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound_x.part_ratio(&bound) * self.mass
        } else {
            self.mass
        }
    }
    ///
    fn mass_shift(&self) -> Position {
        if self.mass_shift.is_none() {
            self.calculate();
        }
        self.mass_shift.clone().expect("Load mass_shift error: no mass_shift!")
    }
    /// Парусность
    fn windage_area(&self) -> f64 {
        if self.windage_area.is_none() {
            self.calculate();
        }
        self.windage_area.clone().expect("Load windage_area error: no windage_area!")
    }
    /// Смещение центра парусности
    fn windage_shift(&self) -> Position {
        if self.windage_shift.is_none() {
            self.calculate();
        }
        self.windage_shift.clone().expect("Load windage_shift error: no windage_shift!")
    }
    /// момент свободной поверхности - TODO - удалить
    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::new(self.m_f_s_x, self.m_f_s_y)
    }
    /// Площадь парусности
    fn windage_area(&self) -> f64 {
        self.bound_x.length()*self.bound_z.length()
    }
}
