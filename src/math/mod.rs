//! Коллекция примитивов для математических операций
pub mod vec;
pub mod position;
pub mod mass_moment;
pub mod surface_moment;
pub mod curve;
pub mod bound;
pub mod pos_shift;
pub mod inertia_shift;
pub mod bounds;

pub use vec::*;
pub use position::*;
pub use mass_moment::*;
pub use surface_moment::*;
pub use curve::*;
pub use bound::*;
pub use pos_shift::*;
pub use inertia_shift::*;
pub use bounds::*;
