use napi::bindgen_prelude::*;

#[derive(Clone)]
pub struct ColorStop {
    pub offset: f64,
    pub color: String,
}
