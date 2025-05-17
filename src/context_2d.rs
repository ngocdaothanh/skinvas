use napi::bindgen_prelude::*;
use napi_derive::napi;
use skia_safe::{
  Paint, Path, Matrix, Point, Color4f, TextBlob, Font,
};
use std::sync::Mutex;
use crate::canvas::{HTMLCanvas, get_skia_canvas};

#[napi(object)]
pub struct TextMetrics {
  pub width: f64,
  pub actual_bounding_box_left: f64,
  pub actual_bounding_box_right: f64,
  pub font_bounding_box_ascent: f64,
  pub font_bounding_box_descent: f64,
  pub actual_bounding_box_ascent: f64,
  pub actual_bounding_box_descent: f64,
  pub em_height_ascent: f64,
  pub em_height_descent: f64,
  pub hanging_baseline: f64,
  pub alphabetic_baseline: f64,
  pub ideographic_baseline: f64,
}

#[napi(string_enum)]
pub enum CompositeOperation {
  SourceOver,
  SourceIn,
  SourceOut,
  SourceAtop,
  DestinationOver,
  DestinationIn,
  DestinationOut,
  DestinationAtop,
  Lighter,
  Copy,
  Xor,
  Multiply,
  Screen,
  Overlay,
  Darken,
  Lighten,
  ColorDodge,
  ColorBurn,
  HardLight,
  SoftLight,
  Difference,
  Exclusion,
}

#[napi(string_enum)]
pub enum LineCap {
  Butt,
  Round,
  Square,
}

#[napi(string_enum)]
pub enum LineJoin {
  Miter,
  Round,
  Bevel,
}

#[napi(string_enum)]
pub enum TextAlign {
  Start,
  End,
  Left,
  Right,
  Center,
}

#[napi(string_enum)]
pub enum TextBaseline {
  Top,
  Hanging,
  Middle,
  Alphabetic,
  Ideographic,
  Bottom,
}

#[napi]
pub struct CanvasRenderingContext2D {
  // Instead of Reference<HTMLCanvas>, store a raw pointer to the HTMLCanvas
  // This avoids the Reference issues with napi 2.16.17
  canvas_ptr: *mut HTMLCanvas,
  fill_style: Mutex<String>,
  stroke_style: Mutex<String>,
  line_width: Mutex<f64>,
  line_cap: Mutex<LineCap>,
  line_join: Mutex<LineJoin>,
  miter_limit: Mutex<f64>,
  font: Mutex<String>,
  text_align: Mutex<TextAlign>,
  text_baseline: Mutex<TextBaseline>,
  global_alpha: Mutex<f64>,
  global_composite_operation: Mutex<CompositeOperation>,
  shadow_blur: Mutex<f64>,
  shadow_color: Mutex<String>,
  shadow_offset_x: Mutex<f64>,
  shadow_offset_y: Mutex<f64>,
  transform_stack: Mutex<Vec<Matrix>>,
  current_path: Mutex<Path>,
}

#[napi]
impl CanvasRenderingContext2D {
  pub fn new(canvas: &HTMLCanvas) -> Result<Self> {
    // Don't use transmute, simply store the pointer directly
    let canvas_ptr = canvas as *const HTMLCanvas as *mut HTMLCanvas;

    Ok(Self {
      canvas_ptr,
      fill_style: Mutex::new(String::from("black")),
      stroke_style: Mutex::new(String::from("black")),
      line_width: Mutex::new(1.0),
      line_cap: Mutex::new(LineCap::Butt),
      line_join: Mutex::new(LineJoin::Miter),
      miter_limit: Mutex::new(10.0),
      font: Mutex::new(String::from("10px sans-serif")),
      text_align: Mutex::new(TextAlign::Start),
      text_baseline: Mutex::new(TextBaseline::Alphabetic),
      global_alpha: Mutex::new(1.0),
      global_composite_operation: Mutex::new(CompositeOperation::SourceOver),
      shadow_blur: Mutex::new(0.0),
      shadow_color: Mutex::new(String::from("rgba(0,0,0,0)")),
      shadow_offset_x: Mutex::new(0.0),
      shadow_offset_y: Mutex::new(0.0),
      transform_stack: Mutex::new(vec![Matrix::new_identity()]),
      current_path: Mutex::new(Path::new()),
    })
  }

  // Basic drawing methods

  #[napi]
  pub fn clear_rect(&self, _x: f64, _y: f64, _width: f64, _height: f64) -> Result<()> {
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;
    // Clear by drawing a transparent rectangle over the area
    canvas.clear(skia_safe::Color4f::new(0.0, 0.0, 0.0, 0.0));

    Ok(())
  }

  #[napi]
  pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;
    let rect = skia_safe::Rect::new(
      x as f32,
      y as f32,
      (x + width) as f32,
      (y + height) as f32
    );

    let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
    paint.set_style(skia_safe::PaintStyle::Fill);

    // TODO: Parse fill_style to determine color or pattern or gradient
    let _fill_style = self.fill_style.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock fill_style mutex")
    })?;

    // Simple implementation with default black color
    canvas.draw_rect(rect, &paint);

    Ok(())
  }

  #[napi]
  pub fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;
    let rect = skia_safe::Rect::new(
      x as f32,
      y as f32,
      (x + width) as f32,
      (y + height) as f32
    );

    let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
    paint.set_style(skia_safe::PaintStyle::Stroke);

    let line_width = *self.line_width.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock line_width mutex")
    })?;

    paint.set_stroke_width(line_width as f32);

    // TODO: Parse stroke_style to determine color or pattern or gradient

    canvas.draw_rect(rect, &paint);

    Ok(())
  }

  // Path methods

  #[napi]
  pub fn begin_path(&self) -> Result<()> {
    let mut current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    *current_path = Path::new();

    Ok(())
  }

  #[napi]
  pub fn move_to(&self, x: f64, y: f64) -> Result<()> {
    let mut current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    current_path.move_to(Point::new(x as f32, y as f32));

    Ok(())
  }

  #[napi]
  pub fn line_to(&self, x: f64, y: f64) -> Result<()> {
    let mut current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    current_path.line_to(Point::new(x as f32, y as f32));

    Ok(())
  }

  #[napi]
  pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) -> Result<()> {
    let mut current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    current_path.cubic_to(
      Point::new(cp1x as f32, cp1y as f32),
      Point::new(cp2x as f32, cp2y as f32),
      Point::new(x as f32, y as f32),
    );

    Ok(())
  }

  #[napi]
  pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) -> Result<()> {
    let mut current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    current_path.quad_to(
      Point::new(cpx as f32, cpy as f32),
      Point::new(x as f32, y as f32),
    );

    Ok(())
  }

  #[napi]
  pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, counter_clockwise: Option<bool>) -> Result<()> {
    let mut current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    let ccw = counter_clockwise.unwrap_or(false);

    // Skia uses a rectangle and start/sweep angles for arcs
    let rect = skia_safe::Rect::new(
      (x - radius) as f32,
      (y - radius) as f32,
      (x + radius) as f32,
      (y + radius) as f32,
    );

    // Convert from radians to degrees
    let start_deg = (start_angle * 180.0 / std::f64::consts::PI) as f32;
    let mut sweep_deg = ((end_angle - start_angle) * 180.0 / std::f64::consts::PI) as f32;

    if ccw && sweep_deg > 0.0 {
      sweep_deg -= 360.0;
    } else if !ccw && sweep_deg < 0.0 {
      sweep_deg += 360.0;
    }

    // Use Skia arc API instead
    let _start_pt = Point::new(
        (x + radius * f64::cos(start_angle)) as f32,
        (y + radius * f64::sin(start_angle)) as f32
    );

    current_path.add_arc(rect, start_deg, sweep_deg);

    Ok(())
  }

  #[napi]
  pub fn rect(&self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
    let mut current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    current_path.add_rect(
      skia_safe::Rect::new(
        x as f32,
        y as f32,
        (x + width) as f32,
        (y + height) as f32,
      ),
      None,
    );

    Ok(())
  }

  #[napi]
  pub fn close_path(&self) -> Result<()> {
    let mut current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    current_path.close();

    Ok(())
  }

  #[napi]
  pub fn fill(&self) -> Result<()> {
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;

    let current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
    paint.set_style(skia_safe::PaintStyle::Fill);

    // TODO: Parse fill_style to determine color or pattern or gradient

    canvas.draw_path(&current_path, &paint);

    Ok(())
  }

  #[napi]
  pub fn stroke(&self) -> Result<()> {
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;

    let current_path = self.current_path.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock current_path mutex")
    })?;

    let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
    paint.set_style(skia_safe::PaintStyle::Stroke);

    let line_width = *self.line_width.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock line_width mutex")
    })?;

    paint.set_stroke_width(line_width as f32);

    // TODO: Parse stroke_style to determine color or pattern or gradient

    canvas.draw_path(&current_path, &paint);

    Ok(())
  }

  // Properties

  #[napi(getter, setter)]
  pub fn fill_style(&self) -> Result<String> {
    let fill_style = self.fill_style.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock fill_style mutex")
    })?;

    Ok(fill_style.clone())
  }

  #[napi(setter)]
  pub fn set_fill_style(&self, value: String) -> Result<()> {
    let mut fill_style = self.fill_style.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock fill_style mutex")
    })?;

    *fill_style = value;

    Ok(())
  }

  #[napi(getter)]
  pub fn stroke_style(&self) -> Result<String> {
    let stroke_style = self.stroke_style.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock stroke_style mutex")
    })?;

    Ok(stroke_style.clone())
  }

  #[napi(setter)]
  pub fn set_stroke_style(&self, value: String) -> Result<()> {
    let mut stroke_style = self.stroke_style.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock stroke_style mutex")
    })?;

    *stroke_style = value;

    Ok(())
  }

  #[napi(getter)]
  pub fn line_width(&self) -> Result<f64> {
    let line_width = self.line_width.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock line_width mutex")
    })?;

    Ok(*line_width)
  }

  #[napi(setter)]
  pub fn set_line_width(&self, value: f64) -> Result<()> {
    let mut line_width = self.line_width.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock line_width mutex")
    })?;

    *line_width = value;

    Ok(())
  }

  // Text methods

  #[napi]
  pub fn fill_text(&self, text: String, x: f64, y: f64, _max_width: Option<f64>) -> Result<()> {
    // This is a simplified implementation
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;

    let font = Font::default();
    let text_blob = TextBlob::new(&text, &font).ok_or_else(|| {
      Error::new(Status::GenericFailure, "Failed to create text blob")
    })?;

    let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
    paint.set_style(skia_safe::PaintStyle::Fill);

    canvas.draw_text_blob(text_blob, (x as f32, y as f32), &paint);

    Ok(())
  }

  #[napi]
  pub fn stroke_text(&self, text: String, x: f64, y: f64, _max_width: Option<f64>) -> Result<()> {
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;

    let font = Font::default();
    let text_blob = TextBlob::new(&text, &font).ok_or_else(|| {
      Error::new(Status::GenericFailure, "Failed to create text blob")
    })?;

    let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
    paint.set_style(skia_safe::PaintStyle::Stroke);

    let line_width = *self.line_width.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock line_width mutex")
    })?;

    paint.set_stroke_width(line_width as f32);

    canvas.draw_text_blob(text_blob, (x as f32, y as f32), &paint);

    Ok(())
  }

  // Transform methods

  #[napi]
  pub fn save(&self) -> Result<()> {
    let mut transform_stack = self.transform_stack.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock transform_stack mutex")
    })?;

    let current_transform = transform_stack.last().cloned().unwrap_or_else(Matrix::new_identity);
    transform_stack.push(current_transform);

    Ok(())
  }

  #[napi]
  pub fn restore(&self) -> Result<()> {
    let mut transform_stack = self.transform_stack.lock().map_err(|_| {
      Error::new(Status::GenericFailure, "Failed to lock transform_stack mutex")
    })?;

    if transform_stack.len() > 1 {
      transform_stack.pop();
    }

    Ok(())
  }

  #[napi]
  pub fn translate(&self, x: f64, y: f64) -> Result<()> {
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;
    canvas.translate((x as f32, y as f32));
    Ok(())
  }

  #[napi]
  pub fn rotate(&self, angle: f64) -> Result<()> {
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;
    canvas.rotate(angle as f32, None);
    Ok(())
  }

  #[napi]
  pub fn scale(&self, x: f64, y: f64) -> Result<()> {
    let canvas_ref = unsafe { &*self.canvas_ptr };
    let canvas = get_skia_canvas(canvas_ref)?;
    canvas.scale((x as f32, y as f32));
    Ok(())
  }
}
