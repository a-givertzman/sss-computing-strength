//! Ограничение горизонтальной площади обледенения палубного груза - леса

use crate::Bound;
/// Тип обледенения горизонтальной площади палубного груза - леса
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IcingTimberType {
    Full,
    HalfLeft,
    HalfRight,
    Bow,
}
///
impl From<String> for IcingTimberType {
    fn from(value: String) -> Self {
        match value.trim().to_lowercase().as_str() {
            "half_left" => IcingTimberType::HalfLeft,
            "half_right" => IcingTimberType::HalfRight,
            "bow" => IcingTimberType::Bow,
            _ => IcingTimberType::Full,
        }
    }
}
/// Ограничение горизонтальной площади обледенения палубного груза - леса
#[derive(Clone)]
pub struct IcingTimberBound {
    /// Ширина корпуса судна  
    width: f64,
    /// Длинна корпуса судна  
    length: f64,
    /// Тип обледенения  
    icing_timber_stab: IcingTimberType,
}
///
impl IcingTimberBound {
    /// Основной конструктор
    /// * width - Ширина корпуса судна  
    /// * length - Длинна корпуса судна    
    /// * icing_timber_stab - Тип обледенения   
    pub fn new(width: f64, length: f64, icing_timber_stab: IcingTimberType) -> Self {
        Self {
            width,
            length,
            icing_timber_stab,
        }
    }
    /// Ограничение по x
    pub fn bound_x(&self) -> Option<Bound> {
        match self.icing_timber_stab {
            IcingTimberType::HalfLeft => None,
            IcingTimberType::HalfRight => None, 
            IcingTimberType::Bow => Some(Bound::new(self.length / 6., self.length / 2.)),
            _ => None,
        }
    }
    /// Ограничение по y
    pub fn bound_y(&self) -> Option<Bound> {
        match self.icing_timber_stab {
            IcingTimberType::HalfLeft => Some(Bound::new(-self.width / 2., 0.)),
            IcingTimberType::HalfRight => Some(Bound::new(0., self.width / 2.)),
            IcingTimberType::Bow => None,
            _ => None,
        }
    }
}
