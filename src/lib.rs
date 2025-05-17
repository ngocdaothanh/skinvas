#![deny(clippy::all)]

// Clean up imports to remove unnecessary ones
use napi_derive::napi;

mod canvas;
mod context_2d;
mod image_data;
mod path2d;
mod pattern;
// Replace gradient module with separate modules
mod color_stop;
mod linear_gradient;
mod radial_gradient;

pub use canvas::*;
pub use context_2d::*;
pub use image_data::*;
pub use path2d::*;
pub use pattern::*;
// Export from new modules instead of gradient
pub use color_stop::*;
pub use linear_gradient::*;
pub use radial_gradient::*;

#[napi]
pub fn version() -> String {
  format!("{}.{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"))
}
