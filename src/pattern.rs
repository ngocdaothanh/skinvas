use napi::bindgen_prelude::*;
use napi_derive::napi;
use skia_safe::{Canvas, Bitmap as SkBitmap, Shader, TileMode, Matrix, Paint};
use crate::canvas::HTMLCanvas;
use crate::image_data::ImageData;
use std::sync::Mutex;

#[napi(string_enum)]
pub enum RepeatPattern {
  Repeat,
  RepeatX,
  RepeatY,
  NoRepeat,
}

#[napi]
pub struct CanvasPattern {
  bitmap: Mutex<SkBitmap>,
  repeat: Mutex<RepeatPattern>,
}

#[napi]
impl CanvasPattern {
  #[napi(constructor)]
  pub fn new(image_data: &ImageData, repeat_pattern: Option<String>) -> Result<Self> {
    let bitmap = image_data.to_skia_bitmap()?;

    let repeat = match repeat_pattern.as_deref() {
      Some("repeat") | None => RepeatPattern::Repeat,
      Some("repeat-x") => RepeatPattern::RepeatX,
      Some("repeat-y") => RepeatPattern::RepeatY,
      Some("no-repeat") => RepeatPattern::NoRepeat,
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid repeat pattern: {:?}", repeat_pattern),
        ));
      }
    };

    Ok(Self {
      bitmap: Mutex::new(bitmap),
      repeat: Mutex::new(repeat),
    })
  }

  // Internal method to create a Skia shader
  pub(crate) fn create_shader(&self) -> Result<Shader> {
    let bitmap = self.bitmap.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock bitmap mutex")
    })?;

    let repeat = self.repeat.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock repeat mutex")
    })?;

    let (tile_x, tile_y) = match *repeat {
      RepeatPattern::Repeat => (TileMode::Repeat, TileMode::Repeat),
      RepeatPattern::RepeatX => (TileMode::Repeat, TileMode::Clamp),
      RepeatPattern::RepeatY => (TileMode::Clamp, TileMode::Repeat),
      RepeatPattern::NoRepeat => (TileMode::Clamp, TileMode::Clamp),
    };

    let image = bitmap.as_image();
    let shader = image.to_shader(
      (tile_x, tile_y),
      None,
      None,
    ).ok_or_else(|| {
      Error::new(Status::GenericFailure, "Failed to create shader from image")
    })?;

    Ok(shader)
  }
}
