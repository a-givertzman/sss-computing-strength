pub mod navigation_area;
pub mod multipler_s;
pub mod multipler_x1;
pub mod multipler_x2;
pub mod coefficient_k;

use super::DataArray;

use self::{coefficient_k::CoefficientKData, multipler_x1::MultiplerX1Data, multipler_x2::MultiplerX2Data};

pub type MultiplerX1Array = DataArray<MultiplerX1Data>;
pub type MultiplerX2Array = DataArray<MultiplerX2Data>;
pub type CoefficientKArray = DataArray<CoefficientKData>;