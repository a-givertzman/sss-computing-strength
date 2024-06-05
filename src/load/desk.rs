//! Палубный груз
use crate::{math::*, ILoad};

use crate::load::ILoadMass;

/// Палубный груз, имеет площадь и парусность
pub trait IDesk: ILoad {
    /// Парусность попадающая в Bound или вся если Bound отсутствует
    fn windage_area(&self, bound: Option<Bound>) -> f64;
    /// Статический момент площади парусности палубного груза, м^3
    fn windage_moment(&self) -> Moment {
        Moment::from_pos(self.shift(), self.windage_area(None))
    }
    /// Площадь горизонтальной поверхности, м^2
    fn horizontal_area(&self, bound: Option<Bound>) -> f64;
    /// Высота груза, м
    fn height(&self) -> f64;
    /// Признак палубного груза: лес
    fn is_timber(&self) -> bool;
}
/// Палубный груз, имеет площадь и парусность  
pub struct Desk { 
    /// Масса груза   
    mass: f64,
    /// Границы груза   
    bound_x: Bound,
    /// Площадь парусности  
    windage_area: f64,
    /// Площадь горизонтальной поверхности 
    horizontal_area: f64,
    /// Смещение центра  
    shift: Position,
    /// Признак палубного груза: лес  
    is_timber: bool,
}
///
impl Desk {
    /// Основной конструктор  
    /// * mass - Масса груза  
    /// * bound_x - границы груза вдоль продольной оси  
    /// * windage_area - Площадь парусности  
    /// * horizontal_area - Площадь горизонтальной поверхности  
    /// * shift - Смещение центра  
    /// * is_timber - Признак палубного груза: лес  
    pub fn new(
        mass: f64,
        bound_x: Bound,
        windage_area: f64,
        horizontal_area: f64,
        shift: Position,
        is_timber: bool,
    ) -> Self {
        Self {
            mass,
            bound_x,
            windage_area,
            horizontal_area,
            shift,
            is_timber,
        }
    }
}
///
///
impl IDesk for Desk {
    /// Парусность попадающая в Bound или вся если Bound отсутствует
    fn windage_area(&self, bound: Option<Bound>) -> f64 {
        self.bound_x.part_ratio(&bound.unwrap_or(self.bound_x)) * 
        self.windage_area
    }
    /// Площадь горизонтальной поверхности, м^2
    fn horizontal_area(&self, bound: Option<Bound>) -> f64 {
        self.bound_x.part_ratio(&bound.unwrap_or(self.bound_x)) *
        self.horizontal_area
    }
    /// Высота груза, м
    fn height(&self) -> f64 {
        self.windage_area/self.bound_x.length()
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
        self.shift.clone()
    }
}
///
impl ILoadMass for Desk {}

