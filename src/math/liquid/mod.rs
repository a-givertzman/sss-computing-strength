//! Кривая момента инерции
pub mod inertia_shift;
pub mod inertia_moment;
pub mod delta_m_h;
pub mod surface_moment;

pub use surface_moment::*;
pub use delta_m_h::*;
#[allow(unused)]
pub use inertia_shift::InertiaShift as InertiaShift;
#[allow(unused)]
pub use inertia_moment::InertiaMoment as InertiaMoment; 