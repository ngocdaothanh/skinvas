use napi::bindgen_prelude::*;
use napi_derive::napi;
use skia_safe::{Surface, AlphaType, ImageInfo};
use std::sync::Mutex;
use crate::context_2d::CanvasRenderingContext2D;

#[napi(js_name = "Canvas")]
pub struct HTMLCanvas {
  width: u32,
  height: u32,
  surface: Mutex<Surface>,
}

#[napi]
impl HTMLCanvas {
  #[napi(constructor)]
  pub fn new(width: u32, height: u32) -> Result<Self> {
    let info = ImageInfo::new_n32(
      (width as i32, height as i32),
      AlphaType::Premul,
      None,
    );

    let mut surface = skia_safe::surfaces::raster(&info, None, None).ok_or_else(|| {
      Error::new(Status::GenericFailure, "Failed to create Skia surface")
    })?;

    // Clear the canvas with white background
    let canvas = surface.canvas();
    canvas.clear(skia_safe::Color::WHITE);

    Ok(Self {
      width,
      height,
      surface: Mutex::new(surface),
    })
  }

  #[napi(getter)]
  pub fn width(&self) -> u32 {
    self.width
  }

  #[napi(setter)]
  pub fn set_width(&mut self, width: u32) -> Result<()> {
    self.resize(width, self.height)
  }

  #[napi(getter)]
  pub fn height(&self) -> u32 {
    self.height
  }

  #[napi(setter)]
  pub fn set_height(&mut self, height: u32) -> Result<()> {
    self.resize(self.width, height)
  }

  #[napi]
  pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
    let info = ImageInfo::new_n32(
      (width as i32, height as i32),
      AlphaType::Premul,
      None,
    );

    let mut new_surface = skia_safe::surfaces::raster(&info, None, None).ok_or_else(|| {
      Error::new(Status::GenericFailure, "Failed to create Skia surface")
    })?;

    // Clear the canvas with white background
    let canvas = new_surface.canvas();
    canvas.clear(skia_safe::Color::WHITE);

    self.width = width;
    self.height = height;

    let mut surface_guard = self.surface.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock surface mutex")
    })?;

    *surface_guard = new_surface;

    Ok(())
  }

  #[napi]
  pub fn get_context_2d(&self) -> Result<CanvasRenderingContext2D> {
    CanvasRenderingContext2D::new(self)
  }

  #[napi]
  pub fn to_buffer(&self, mime_type: Option<String>, quality: Option<f64>) -> Result<Buffer> {
    let format = match mime_type.as_deref() {
      Some("image/png") | None => skia_safe::EncodedImageFormat::PNG,
      Some("image/jpeg") => skia_safe::EncodedImageFormat::JPEG,
      Some("image/webp") => skia_safe::EncodedImageFormat::WEBP,
      _ => return Err(Error::new(Status::InvalidArg, "Unsupported MIME type")),
    };

    let quality = quality.unwrap_or(0.92);
    let quality_int = (quality * 100.0).clamp(0.0, 100.0) as i32;

    let mut surface_guard = self.surface.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock surface mutex")
    })?;

    let image = surface_guard.image_snapshot();
    // Use the encode method with the correct types
    let data = image.encode(None, format, quality_int as u32).ok_or_else(|| {
      Error::new(Status::GenericFailure, "Failed to encode image")
    })?;

    let bytes = data.as_bytes();
    let buffer = Buffer::from(bytes);

    Ok(buffer)
  }
}

// Internal function to get Skia canvas from HTMLCanvas
pub(crate) fn get_skia_canvas(canvas: &HTMLCanvas) -> Result<&skia_safe::Canvas> {
  // In skia-safe 0.84.0, Canvas is no longer Clone
  // We need to return a reference, but the lifetime is problematic
  // FIXME: This is a temporary workaround that leaks memory
  let mut surface_guard = canvas.surface.lock().map_err(|_| {
    Error::new(Status::GenericFailure, "Failed to lock surface mutex")
  })?;

  let canvas_ptr = surface_guard.canvas() as *const skia_safe::Canvas;
  unsafe {
    Ok(&*canvas_ptr)
  }
}
