//! Промежуточные структуры для serde_json для парсинга данных
//! для расчета остойчивости
pub mod navigation_area_data;
pub mod multipler_s;
pub mod horizontal_area;
mod pantocaren;
pub mod delta_windage_moment;
pub mod vertical_area;
pub mod ship_type;
pub mod navigation_area;
pub mod icing;

pub use horizontal_area::*;
pub use pantocaren::*;
pub use delta_windage_moment::*;
pub use vertical_area::*;
pub use ship_type::*;
pub use navigation_area::*;
pub use icing::*;

///
use super::{DataArray, Pair};
/// Безразмерный множитель Х_1 Табл. 2.1.5.1-1
pub type MultiplerX1Array = DataArray<Pair>;
/// Безразмерный множитель Х_2 Табл. 2.1.5.1-2
pub type MultiplerX2Array = DataArray<Pair>;
/// Коэффициент k для судов, имеющих скуловые кили или 
/// брусковый киль. Табл. 2.1.5.2
pub type CoefficientKArray = DataArray<Pair>;
/// Коэффициент k_theta учитывающий особенности качки судов смешанного типа
pub type CoefficientKThetaArray = DataArray<Pair>;
