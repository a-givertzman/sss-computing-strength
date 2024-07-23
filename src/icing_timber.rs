//! Ограничение горизонтальной площади обледенения палубного груза - леса

use crate::{Bound, Error};

use serde::{Deserialize, Serialize};

/// Тип обледенения горизонтальной площади палубного груза - леса
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize,)]
pub enum IcingTimberType {
    #[serde(alias="full")]
    Full,
    #[serde(alias="half left")]
    HalfLeft,
    #[serde(alias="half right")]
    HalfRight,
    #[serde(alias="bow")]
    Bow,
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
    pub fn bound_x(&self) -> Result<Option<Bound>, Error> {
        Ok(match self.icing_timber_stab {
            IcingTimberType::HalfLeft => None,
            IcingTimberType::HalfRight => None, 
            IcingTimberType::Bow => Some(Bound::new(self.length / 6., self.length / 2.)?),
            IcingTimberType::Full => None,
        })
    }
    /// Ограничение по y
    pub fn bound_y(&self) -> Result<Option<Bound>, Error> {
        Ok(match self.icing_timber_stab {
            IcingTimberType::HalfLeft => Some(Bound::new(-self.width / 2., 0.)?),
            IcingTimberType::HalfRight => Some(Bound::new(0., self.width / 2.)?),
            IcingTimberType::Bow => None,
            IcingTimberType::Full => None,
        })
    }
}
