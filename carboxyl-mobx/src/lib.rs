use carboxyl::lift;
use carboxyl::Sink;
use carboxyl::Stream;
use wasm_bindgen::prelude::*;

// Trial interaction between carboxyl and mobx
// Do liftinf


// Fist pass
// Allow creating and deleting named "observables"


// What's the minimum that's useful?
// - Need to do some computations in Rust

// Minimum to demonstrate the principal of reactivity across Rust and JS?
// - Create a flat object of {[key: string]: string | number} from JS
// - Allow updating with JSON
// - Allow serialising to JSON
// - Watch variables from JS


// **Even smaller**
// - Statically declare models in RUst
// - Observe properties from JS

#[derive(Serialize, Deserialize)]
struct MyModel {
  name: String,
  r#type: String,
  url: String,
  maximumZoom: u32,
  dates: Vec<String>gsdfsfwwdfsewfdsfdwfdsfdefewfsw
}


// Flow:
// 1. Create

static mut SINKS: Vec<Sink<i32>> = Vec::new();

#[wasm_bindgen]
pub fn create_object()

#[wasm_bindgen]
pub fn new_sink() -> Sink<i32> {
  let sink = Sink::new();
  sink
}

pub fn create_string_observable() -> i32

pub fn main() {
  let sink = Sink::new();
  SINKS.push(sink);

  let signal = sink.stream().hold(0);
  println!("Hello, world!");
}
