extern crate synth;

use wasm_bindgen::prelude::*;

use synth::synth::generate as wrapped_generate;

#[wasm_bindgen]
pub fn generate() -> Vec<f32> {
   wrapped_generate()
}
