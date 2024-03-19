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

pub use ship::*;
pub use frame::*;
pub use load_space::*;
pub use tank::*;
pub use result::*;
pub use serde_parser::*;
use data_array::*;
pub use load_constant::*;

pub type RadLongDataArray = DataArray<Pair>;
pub type RadCrossDataArray = DataArray<Pair>;
pub type MeanDraughtDataArray = DataArray<Pair>;
pub type CenterWaterlineArray = DataArray<Pair>;
pub type CenterDraughtShiftDataArray = DataArray<Quadruple>;

