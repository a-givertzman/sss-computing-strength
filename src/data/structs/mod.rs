//! Структуры для ввода/вывода данных
pub mod result;
pub mod ship;
pub mod data_array;
pub mod loads;
pub mod stability;
pub mod strength;
pub mod serde_parser;

pub use data_array::*;
pub use ship::*;
pub use result::*;
pub use stability::*;
pub use strength::*;


pub use stability::navigation_area_data::NavigationAreaArray as NavigationAreaArray;
pub use stability::multipler_s::MultiplerSArray as MultiplerSArray;
pub use stability::MultiplerX1Array as MultiplerX1Array;
pub use stability::MultiplerX2Array as MultiplerX2Array;
pub use stability::CoefficientKArray as CoefficientKArray;
pub use stability::CoefficientKThetaArray as CoefficientKThetaArray;

pub type RadLongDataArray = DataArray<Pair>;
pub type RadCrossDataArray = DataArray<Pair>;
pub type MetacentricHeightSubdivisionArray = DataArray<Pair>;
pub type MeanDraughtDataArray = DataArray<Pair>;
pub type CenterWaterlineArray = DataArray<Pair>;
pub type CenterDraughtShiftDataArray = DataArray<Quadruple>;
pub type FloodingAngleDataArray = DataArray<Pair>;
pub type EntryAngleDataArray = DataArray<Pair>;
pub type WaterlineLengthArray = DataArray<Pair>;
pub type WaterlineBreadthArray = DataArray<Pair>;
pub type VolumeShiftArray = DataArray<Pair>;
pub type DeltaWindageAreaDataArray = DataArray<Pair>;


