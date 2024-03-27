pub mod navigation_area;
pub mod multipler_s;
///
use super::{DataArray, Pair};
/// Безразмерный множитель Х_1 Табл. 2.1.5.1-1
pub type MultiplerX1Array = DataArray<Pair>;
/// Безразмерный множитель Х_2 Табл. 2.1.5.1-2
pub type MultiplerX2Array = DataArray<Pair>;
/// Коэффициент k для судов, имеющих скуловые кили или 
/// брусковый киль. Табл. 2.1.5.2
pub type CoefficientKArray = DataArray<Pair>;
