//! Промежуточные структуры для serde_json для парсинга данных
//! для расчета остойчивости
pub mod bow_board;
pub mod delta_windage_moment;
pub mod draft_mark;
pub mod horizontal_area;
pub mod icing;
pub mod load_line;
pub mod multipler_s;
pub mod navigation_area;
pub mod navigation_area_data;
mod pantocaren;
pub mod point;
pub mod screw;
pub mod ship_type;
pub mod vertical_area;

pub use bow_board::*;
pub use delta_windage_moment::*;
pub use draft_mark::*;
pub use horizontal_area::*;
pub use vertical_area::*;
pub use icing::*;
pub use load_line::*;
pub use navigation_area::*;
pub use navigation_area_data::*;
pub use pantocaren::*;
pub use point::*;
pub use screw::*;
pub use load_line::*;
pub use bow_board::*;
use super::{DataArray, Pair};
//
/// Безразмерный множитель Х_1 Табл. 2.1.5.1-1
pub type MultiplerX1Array = DataArray<Pair>;
/// Безразмерный множитель Х_2 Табл. 2.1.5.1-2
pub type MultiplerX2Array = DataArray<Pair>;
/// Коэффициент k для судов, имеющих скуловые кили или
/// брусковый киль. Табл. 2.1.5.2
pub type CoefficientKArray = DataArray<Pair>;
/// Коэффициент k_theta учитывающий особенности качки судов смешанного типа
pub type CoefficientKThetaArray = DataArray<Pair>;
