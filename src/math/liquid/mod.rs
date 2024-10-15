//! Момент инерции жидкости
pub mod inertia_moment;
pub mod delta_m_h;
pub mod surface_moment;

pub use surface_moment::*;
pub use delta_m_h::*;
#[allow(unused)]
pub use inertia_moment::InertiaMoment as InertiaMoment; 