use napi::bindgen_prelude::*;
use napi_derive::napi;
use skia_safe::{Color4f, Shader, Point, TileMode, GradientShader, Matrix};
use std::sync::Mutex;

#[napi(object)]
pub struct ColorStop {
  pub offset: f64,
  pub color: String,
}

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

    let points = [
      Point::new(self.x0 as f32, self.y0 as f32),
      Point::new(self.x1 as f32, self.y1 as f32),
    ];

    let shader = GradientShader::linear(
      &points,
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

#[napi]
pub struct RadialGradient {
  x0: f64,
  y0: f64,
  r0: f64,
  x1: f64,
  y1: f64,
  r1: f64,
  color_stops: Mutex<Vec<ColorStop>>,
}

#[napi]
impl RadialGradient {
  #[napi(constructor)]
  pub fn new(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> Self {
    Self {
      x0,
      y0,
      r0,
      x1,
      y1,
      r1,
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

    // For simplicity, we'll just use a two-point conical gradient
    let center = Point::new(self.x0 as f32, self.y0 as f32);
    let shader = GradientShader::two_point_conical(
      center,
      self.r0 as f32,
      Point::new(self.x1 as f32, self.y1 as f32),
      self.r1 as f32,
      colors.as_slice(),
      Some(positions.as_slice()),
      TileMode::Clamp,
      None,
      None,
    ).ok_or_else(|| {
      Error::new(Status::GenericFailure, "Failed to create radial gradient shader")
    })?;

    Ok(shader)
  }
}
