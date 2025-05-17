use napi::bindgen_prelude::*;
use napi_derive::napi;
use skia_safe::{Color, Color4f, Data, Bitmap as SkBitmap, AlphaType, ColorType, ImageInfo};

#[napi(js_name = "ImageData")]
pub struct ImageData {
  width: u32,
  height: u32,
  data: Buffer,
}

#[napi]
impl ImageData {
  #[napi(constructor)]
  pub fn new(width: u32, height: u32) -> Result<Self> {
    let data_size = width as usize * height as usize * 4;
    let data = Buffer::new_with_size(data_size);

    Ok(Self {
      width,
      height,
      data,
    })
  }

  #[napi(factory)]
  pub fn from_buffer(data: Buffer, width: u32, height: Option<u32>) -> Result<Self> {
    let height = match height {
      Some(h) => h,
      None => {
        if data.len() % (width as usize * 4) != 0 {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Buffer size {} is not a multiple of (width * 4)", data.len()),
          ));
        }
        (data.len() / (width as usize * 4)) as u32
      }
    };

    if width as usize * height as usize * 4 != data.len() {
      return Err(Error::new(
        Status::InvalidArg,
        format!(
          "Buffer size {} does not match width {} x height {} x 4",
          data.len(),
          width,
          height
        ),
      ));
    }

    Ok(Self {
      width,
      height,
      data,
    })
  }

  #[napi(getter)]
  pub fn width(&self) -> u32 {
    self.width
  }

  #[napi(getter)]
  pub fn height(&self) -> u32 {
    self.height
  }

  #[napi(getter)]
  pub fn data(&self) -> Buffer {
    self.data.clone()
  }

  // Internal method to convert to Skia bitmap
  pub(crate) fn to_skia_bitmap(&self) -> Result<SkBitmap> {
    let mut bitmap = SkBitmap::new();

    let info = ImageInfo::new_n32(
      (self.width as i32, self.height as i32),
      AlphaType::Premul,
      None,
    );

    bitmap.set_info(&info, None);
    let pixels = unsafe { bitmap.get_pixels_mut().unwrap() };

    // Copy data from Buffer to bitmap
    let data_slice = self.data.as_ref();
    pixels.copy_from_slice(data_slice);

    Ok(bitmap)
  }

  // Internal method to create from Skia bitmap
  pub(crate) fn from_skia_bitmap(bitmap: &SkBitmap) -> Result<Self> {
    let width = bitmap.width() as u32;
    let height = bitmap.height() as u32;

    let info = bitmap.info();
    let bytes_per_pixel = info.bytes_per_pixel();
    let row_bytes = bitmap.row_bytes();

    let data_size = width as usize * height as usize * 4;
    let mut data = Buffer::new_with_size(data_size);

    let pixels = unsafe { bitmap.get_pixels().unwrap() };

    // Copy data from bitmap to Buffer
    let data_slice = unsafe {
      std::slice::from_raw_parts_mut(
        data.as_mut_ptr(),
        data.len()
      )
    };

    for y in 0..height as usize {
      let src_offset = y * row_bytes;
      let dst_offset = y * width as usize * 4;

      for x in 0..width as usize {
        let src_pixel_offset = src_offset + x * bytes_per_pixel;
        let dst_pixel_offset = dst_offset + x * 4;

        // RGBA to RGBA
        data_slice[dst_pixel_offset + 0] = pixels[src_pixel_offset + 0];
        data_slice[dst_pixel_offset + 1] = pixels[src_pixel_offset + 1];
        data_slice[dst_pixel_offset + 2] = pixels[src_pixel_offset + 2];
        data_slice[dst_pixel_offset + 3] = pixels[src_pixel_offset + 3];
      }
    }

    Ok(Self {
      width,
      height,
      data,
    })
  }
}
