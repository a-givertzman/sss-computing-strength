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
pub mod delta_m_h;
pub mod curve2d;

pub use vec::*;
pub use position::*;
pub use mass_moment::*;
pub use surface_moment::*;
pub use curve::*;
pub use curve2d::*;
pub use bound::*;
pub use pos_shift::*;
pub use inertia_shift::*;
pub use bounds::*;
pub use delta_m_h::*;