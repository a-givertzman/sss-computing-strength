//! Палубный груз
use crate::{math::*, Error, ILoad};

use crate::load::ILoadMass;

/// Палубный груз, имеет площадь и парусность
pub trait IDesk: ILoad {
    /// Парусность попадающая в Bound или вся если Bound отсутствует
    fn windage_area(&self, bound: &Bound) -> Result<f64, Error>;
    /// Статический момент площади парусности палубного груза, м^3
    fn windage_moment(&self) -> Result<Moment, Error>;
    /// Площадь горизонтальной поверхности, м^2
    fn horizontal_area(&self, bound_x: &Bound, bound_y: &Bound) -> Result<f64, Error>;
    /// Высота груза, м
    fn height(&self) -> Result<f64, Error>;
    /// Признак палубного груза: лес
    fn is_timber(&self) -> bool;
}
/// Палубный груз, имеет площадь и парусность  
pub struct Desk {
    /// Масса груза   
    mass: f64,
    /// Смещение центра массы  
    mass_shift: Position,
    /// Ограничение по оси Х
    bound_x: Bound,
    /// Ограничение по оси Y
    bound_y: Bound,
    /// Площадь парусности  
    windage_area: f64,
    /// Смещение центра парусности  
    windage_shift: Position,
    /// Площадь горизонтальной поверхности
    horizontal_area: f64,
    /// Признак палубного груза: лес  
    is_timber: bool,
}
///
impl Desk {
    /// Основной конструктор  
    /// * mass - Масса груза  
    /// * shift - Смещение центра массы
    /// * bound_x - Ограничение по оси Х
    /// * bound_y - Ограничение по оси Y
    /// * windage_area - Площадь парусности  
    /// * windage_shift - Смещение центра парусности
    /// * horizontal_area - Площадь горизонтальной поверхности  
    /// * is_timber - Признак палубного груза: лес  
    pub fn new(
        mass: f64,
        mass_shift: Position,
        bound_x: Bound,
        bound_y: Bound,
        windage_area: f64,
        windage_shift: Position,
        horizontal_area: f64,
        is_timber: bool,
    ) -> Self {
        Self {
            mass,
            mass_shift,
            bound_x,
            bound_y,
            windage_area,
            windage_shift,
            horizontal_area,
            is_timber,
        }
    }
}
///
///
impl IDesk for Desk {
    /// Парусность попадающая в Bound или вся если Bound отсутствует
    fn windage_area(&self, bound: &Bound) -> Result<f64, Error> {
        Ok(self.bound_x.part_ratio(bound)? * self.windage_area)
    }
    /// Статический момент площади парусности палубного груза, м^3
    fn windage_moment(&self) -> Result<Moment, Error> {
        Ok(Moment::from_pos(
            self.windage_shift.clone(),
            self.windage_area(&Bound::Full)?,
        ))
    }
    /// Площадь горизонтальной поверхности, м^2
    fn horizontal_area(&self, bound_x: &Bound, bound_y: &Bound) -> Result<f64, Error> {
        let part_x = self.bound_x.part_ratio(bound_x)?;
        let part_y = self.bound_y.part_ratio(bound_y)?;
        Ok(part_x * part_y * self.horizontal_area)
    }
    /// Высота груза, м,
    /// TODO: после того, как в базе появятся палубные грузы добавить нормальную высоту
    fn height(&self) -> Result<f64, Error> {
        Ok(self.windage_area
            / self.bound_x.length().ok_or(Error::FromString(
                "Desk height error: no self.bound_x.length".to_owned(),
            ))?)
    }
    /// Признак палубного груза: лес
    fn is_timber(&self) -> bool {
        self.is_timber
    }
}
///
impl ILoad for Desk {
    ///
    fn mass(&self) -> f64 {
        self.mass
    }
    ///
    fn bound_x(&self) -> Bound {
        self.bound_x
    }
    ///
    fn shift(&self) -> Position {
        self.mass_shift.clone()
    }
}
///
impl ILoadMass for Desk {}
