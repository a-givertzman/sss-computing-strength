//! Расчет прочности судна

pub(crate) mod bending_moment;
pub(crate) mod computer;
pub(crate) mod displacement;
pub(crate) mod volume;
pub(crate) mod frame;
pub(crate) mod shear_force;
pub(crate) mod total_force;
pub(crate) mod area;
pub(crate) mod trim;
pub(crate) mod results;
pub(crate) mod mass;
pub(crate) mod icing;
pub(crate) mod wetting;

pub use computer::*;
pub use displacement::*;
pub use volume::*;
pub use total_force::*;
pub use area::*;
pub use trim::*;
pub use results::*;
pub use mass::*;
pub use icing::*;
pub use wetting::*;

