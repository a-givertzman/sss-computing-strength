//! Коллекция примитивов для математических операций
pub mod vec;
pub mod position;
pub mod moment;
pub mod surface_moment;
pub mod curve;
pub mod bound;
pub mod pos_shift;
pub mod inertia;
pub mod bounds;
pub mod delta_m_h;
pub mod curve2d;
pub mod area;

#[allow(unused)]
pub use vec::integral_sum::IntegralSum as IntegralSum;
#[allow(unused)]
pub use vec::integral_cotes::IntegralCotes as IntegralCotes;
#[allow(unused)]
pub use vec::integral::Integral as Integral;
pub use vec::*;
pub use position::*;
pub use moment::*;
pub use surface_moment::*;
pub use curve::*;
pub use curve2d::*;
pub use bound::*;
pub use pos_shift::*;
pub use inertia::*;
pub use bounds::*;
pub use delta_m_h::*;
pub use area::*;