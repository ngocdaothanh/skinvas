#![deny(clippy::all)]

use napi_derive::napi;

mod canvas;
mod context_2d;
mod image_data;
mod path2d;
mod pattern;
mod gradient;

pub use canvas::*;
pub use context_2d::*;
pub use image_data::*;
pub use path2d::*;
pub use pattern::*;
pub use gradient::*;

#[napi]
pub fn version() -> String {
  format!("{}.{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"))
}
