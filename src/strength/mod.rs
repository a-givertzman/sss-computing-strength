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

pub use computer::*;
pub use displacement::*;
pub use volume::*;
pub use frame::*;
pub use total_force::*;
pub use area::*;
pub use trim::*;



