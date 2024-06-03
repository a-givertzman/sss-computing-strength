//! Промежуточные структуры для serde_json для парсинга данных
//! для расчета прочности
pub mod horizontal_area;
pub mod frame_area;
pub mod frame_index;
pub mod computed_frame;

pub use frame_area::*;
pub use frame_index::*;
pub use computed_frame::*;
pub use horizontal_area::*;