use napi::bindgen_prelude::*;
use napi_derive::napi;
use skia_safe::{Color4f, Shader, Point, TileMode, gradient_shader};
use std::sync::Mutex;
use crate::color_stop::ColorStop;

#[napi]
pub struct LinearGradient {
  x0: f64,
  y0: f64,
  x1: f64,
  y1: f64,
  color_stops: Mutex<Vec<ColorStop>>,
}

#[napi]
impl LinearGradient {
  #[napi(constructor)]
  pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
    Self {
      x0,
      y0,
      x1,
      y1,
      color_stops: Mutex::new(Vec::new()),
    }
  }

  #[napi]
  pub fn add_color_stop(&self, offset: f64, color: String) -> Result<()> {
    let mut color_stops = self.color_stops.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock color_stops mutex")
    })?;

    color_stops.push(ColorStop { offset, color });

    Ok(())
  }

  // Internal method to create a Skia shader
  pub(crate) fn create_shader(&self) -> Result<Shader> {
    let color_stops = self.color_stops.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock color_stops mutex")
    })?;

    if color_stops.is_empty() {
      return Err(Error::new(Status::InvalidArg, "No color stops defined"));
    }

    // Convert color stops to Skia format
    let mut colors = Vec::with_capacity(color_stops.len());
    let mut positions = Vec::with_capacity(color_stops.len());

    // This is a simplified implementation - in a real library we'd need to parse color strings
    for stop in color_stops.iter() {
      colors.push(Color4f::new(0.0, 0.0, 0.0, 1.0)); // Black as default
      positions.push(stop.offset as f32);
    }

    let point1 = Point::new(self.x0 as f32, self.y0 as f32);
    let point2 = Point::new(self.x1 as f32, self.y1 as f32);

    let shader = gradient_shader::linear(
      (point1, point2),
      colors.as_slice(),
      Some(positions.as_slice()),
      TileMode::Clamp,
      None,
      None,
    ).ok_or_else(|| {
      Error::new(Status::GenericFailure, "Failed to create linear gradient shader")
    })?;

    Ok(shader)
  }
}
