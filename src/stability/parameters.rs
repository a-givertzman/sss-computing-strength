//! Набор результатов расчетов для записи в БД

use std::{cell::RefCell, collections::HashMap};

///
#[derive(Hash, Eq, PartialEq)]
pub enum ParameterID {
    CenterMassZFix = 1,
    Displacement = 2,
    DraughtMean = 3,
    DraughtBow = 4,
    DraughtStern = 5,
    Trim = 6,
    Roll = 7,
    TonesPerCm = 8,
    MomentRollPerDeg = 9,
    MomentTrimPerCm = 10,
    CenterVolumeZ = 11,    
    CenterMassZ = 12,
    MetacentricTransRad = 13,
    MetacentricTransRadZ = 14,
    MetacentricTransHeight = 15,
    MetacentricTransStore = 16,
    MetacentricTransBallast = 17,
    MetacentricTransHeightFix = 18,       
    MetacentricLongRad = 19,
    MetacentricLongRadZ = 20, 
    MetacentricLongHeight = 21,
    MetacentricLongStore = 22,
    MetacentricLongBallast = 23,    
    MetacentricLongHeightFix = 24, 
    MassBallast = 25, 
    MassStores = 26, 
    MassCargo = 27, 
    MassDeadweight = 28, 
    MassLightship = 29, 
    MassIcing = 30, 
    MassWetting = 31, 
    CenterMassX = 32,
    WindPressure = 33,
    WindageArea = 34,  
    WindageAreaLever = 35,
    StaticWindageHeelingLever = 36,
    DynamicWindageHeelingLever = 37,
    StaticWindageHeelingAngle = 38,
    DynamicWindageHeelingAngle = 39,
    HeelingAngleOfSecondPointOfIntersectionWith = 40,
    RollAmplitude = 41,
    RollPeriod = 42,
    AreaA = 43,
    AreaB = 44,
    OpenDeckEdgeImmersionAngle = 45,
    AngleOfDownFlooding = 46,
    SunsetAngle = 47,
    HeelingMomentDueToTheTransverseShiftOfGrain = 48,
    HeelingAngleWithMaximumDifference = 49,
    VesselSpeed = 50,
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
    fn get(&self, id: ParameterID) -> Option<f64> {
        self.data.borrow().get(&id).copied()
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
    fn get(&self, id: ParameterID) -> Option<f64>;
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
    fn get(&self, _: ParameterID) -> Option<f64> {None}
    ///
    fn take_data(&self) -> Vec<(usize, f64)> {
        Vec::new()
    }
}

