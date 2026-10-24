// Module root required by the supplied crate paths (crate::geometry, crate::dodleston, …).
// Source modules are transcribed as supplied.

pub mod capacity;
pub mod dodleston;
pub mod dodleston_integration;
pub mod fdtd;
pub mod geometry;
pub mod thin_wire;

pub use dodleston::DodlestonDetector;
pub use dodleston_integration::DodlestonEMModel;
