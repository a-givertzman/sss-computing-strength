//! Промежуточные структуры для serde_json для парсинга данных
//! для расчета прочности
pub mod horizontal_area;
pub mod frame;
pub mod computed_frame;

pub use frame::*;
pub use computed_frame::*;
pub use horizontal_area::*;