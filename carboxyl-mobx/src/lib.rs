use carboxyl::lift;
use carboxyl::Sink;
use carboxyl::Stream;
use wasm_bindgen::prelude::*;

// Trial interaction between carboxyl and mobx
// Do liftinf

static mut SINKS: Vec<Sink<i32>> = Vec::new();

#[wasm_bindgen]
pub fn new_sink() -> Sink<i32> {
  let sink = Sink::new();
  sink
}

pub fn main() {
  let sink = Sink::new();
  SINKS.push(sink);

  let signal = sink.stream().hold(0);
  println!("Hello, world!");
}
