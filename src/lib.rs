use wasm_bindgen::prelude::*;
mod game;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
