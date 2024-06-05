//! Набор результатов расчетов для записи в БД

use std::{cell::RefCell, collections::HashMap};

///
#[derive(Hash, Eq, PartialEq)]
pub enum ParameterID {
    CenterMassZFix,
    Displacement,
    DraughtMean,
    DraughtBow,
    DraughtStern,
    Trim,
    Pitch,
    TonesPerCm,
    MomentRollPerDeg,
    MomentTrimPerCm,
    CenterVolumeZ,    
    CenterMassZ,
    MetacentricTransRad,
    MetacentricTransRadZ,
    MetacentricTransHeight,
    MetacentricTransStore,
    MetacentricTransBallast,
    MetacentricTransHeightFix,       
    MetacentricLongRad,
    MetacentricLongRadZ, 
    MetacentricLongHeight,
    MetacentricLongStore,
    MetacentricLongBallast,    
    MetacentricLongHeightFix,   
    MassBallast,
    MassStores,
    MassCargo,
    MassDeadweight,
    MassLightship,
    MassIcing,
    MassWetting,
}
///
impl From<ParameterID> for usize {
    fn from(id: ParameterID) -> Self {
        match id {
            ParameterID::CenterMassZFix => 1,
            ParameterID::Displacement => 2,
            ParameterID::DraughtMean => 3,
            ParameterID::DraughtBow => 4,
            ParameterID::DraughtStern => 5,
            ParameterID::Trim => 6,
            ParameterID::Pitch => 7,
            ParameterID::TonesPerCm => 8,
            ParameterID::MomentRollPerDeg => 9,
            ParameterID::MomentTrimPerCm => 10,
            ParameterID::CenterVolumeZ => 11,    
            ParameterID::CenterMassZ => 12,
            ParameterID::MetacentricTransRad => 13,
            ParameterID::MetacentricTransRadZ => 14,
            ParameterID::MetacentricTransHeight => 15,
            ParameterID::MetacentricTransStore => 16,
            ParameterID::MetacentricTransBallast => 17,
            ParameterID::MetacentricTransHeightFix => 18,       
            ParameterID::MetacentricLongRad => 19,
            ParameterID::MetacentricLongRadZ => 20, 
            ParameterID::MetacentricLongHeight => 21,
            ParameterID::MetacentricLongStore => 22,
            ParameterID::MetacentricLongBallast => 23,    
            ParameterID::MetacentricLongHeightFix => 24, 
            ParameterID::MassBallast => 25, 
            ParameterID::MassStores => 26, 
            ParameterID::MassCargo => 27, 
            ParameterID::MassDeadweight => 28, 
            ParameterID::MassLightship => 29, 
            ParameterID::MassIcing => 30, 
            ParameterID::MassWetting => 31, 
        }
    }
}

/// Набор результатов расчетов для записи в БД
pub struct Parameters {
    data: RefCell<HashMap<ParameterID, f64>>,
} 
///
impl Parameters {
    ///
    pub fn new() -> Self {
        Self{ data: RefCell::new(HashMap::new()) }
    }
}
///
impl IParameters for Parameters {
    ///
    fn add(&self, id: ParameterID, value: f64) {
        self.data.borrow_mut().insert(id, value);
    }
    ///
    fn take_data(&self) -> Vec<(usize, f64)> {
        self.data.take().into_iter().map(|(k, v)| (k as usize, v)).collect()
    }
}
#[doc(hidden)]
pub trait IParameters {
    ///
    fn add(&self, id: ParameterID, value: f64);
    ///
    fn take_data(&self) -> Vec<(usize, f64)>;
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeParameters; 
#[doc(hidden)]
#[allow(dead_code)]
impl IParameters for FakeParameters {
    ///
    fn add(&self, _: ParameterID, _: f64) { }
    ///
    fn take_data(&self) -> Vec<(usize, f64)> {
        Vec::new()
    }
}

