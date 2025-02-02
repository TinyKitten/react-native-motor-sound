#![deny(clippy::all)]

use rodio::{source::SineWave, OutputStream, Sink};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct MotorSound {
  sink: Arc<Mutex<Sink>>,
}

#[napi]
impl MotorSound {
  #![allow(clippy::new_without_default)]
  #[napi(constructor)]
  pub fn new() -> Self {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    Self {
      sink: Arc::new(Mutex::new(sink)),
    }
  }

  #[napi]
  pub fn update_speed(&self, speed: f64) {
    let base_freq = 50.0;
    let new_freq = base_freq * (speed / 100.0 + 1.0) as f32;
    let sink = self.sink.lock().unwrap();
    sink.clear();
    sink.append(SineWave::new(new_freq));
  }
}
