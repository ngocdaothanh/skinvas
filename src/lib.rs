#![deny(clippy::all)]

use napi_derive::napi;

pub mod canvas;
pub mod context_2d;
pub mod gradient;
pub mod color_stop;
pub mod pattern;
pub mod path2d;
pub mod image_data;
pub mod linear_gradient;
pub mod radial_gradient;

// Re-export the gradient types for easier access
pub use gradient::{LinearGradient, RadialGradient};

#[napi]
pub fn version() -> String {
  format!("{}.{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"))
}
