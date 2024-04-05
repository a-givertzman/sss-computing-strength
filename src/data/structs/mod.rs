//! Структуры для ввода/вывода данных
pub mod frame;
pub mod load_space;
pub mod tank;
pub mod result;
pub mod serde_parser;
pub mod ship;
mod data_array;
pub mod load_constant;
mod pantocaren;
mod stability;
pub mod computed_frame;

use data_array::*;
pub use ship::*;
pub use frame::*;
pub use load_space::*;
pub use tank::*;
pub use result::*;
pub use serde_parser::*;
pub use load_constant::*;
pub use pantocaren::*;
pub use computed_frame::*;
pub use stability::navigation_area::NavigationAreaArray as NavigationAreaArray;
pub use stability::multipler_s::MultiplerSArray as MultiplerSArray;
pub use stability::MultiplerX1Array as MultiplerX1Array;
pub use stability::MultiplerX2Array as MultiplerX2Array;
pub use stability::CoefficientKArray as CoefficientKArray;

pub type RadLongDataArray = DataArray<Pair>;
pub type RadCrossDataArray = DataArray<Pair>;
pub type MeanDraughtDataArray = DataArray<Pair>;
pub type CenterWaterlineArray = DataArray<Pair>;
pub type CenterDraughtShiftDataArray = DataArray<Quadruple>;
pub type FloodingAngleDataArray = DataArray<Pair>;
pub type EntryAngleDataArray = DataArray<Pair>;
pub type WaterlineLengthArray = DataArray<Pair>;
pub type WaterlineBreadthArray = DataArray<Pair>;
pub type VolumeShiftArray = DataArray<Pair>;

