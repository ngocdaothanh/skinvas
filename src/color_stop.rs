use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct ColorStop {
  pub offset: f64,
  pub color: String,
}
