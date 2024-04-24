//! Коллекция примитивов для математических операций
pub mod vec;
pub mod position;
pub mod moment;
pub mod curve;
pub mod bound;
pub mod pos_shift;
pub mod liquid;
pub mod bounds;
pub mod curve2d;

#[allow(unused)]
pub use vec::integral_sum::IntegralSum as IntegralSum;
#[allow(unused)]
pub use vec::integral_cotes::IntegralCotes as IntegralCotes;
#[allow(unused)]
pub use vec::integral::Integral as Integral;
pub use vec::*;
pub use position::*;
pub use moment::*;
pub use curve::*;
pub use curve2d::*;
pub use bound::*;
pub use pos_shift::*;
pub use liquid::*;
pub use bounds::*;