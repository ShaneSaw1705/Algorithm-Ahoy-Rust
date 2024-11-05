use wasm_bindgen::prelude::*;
mod code;
mod game;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn run_python_code(code: &str) -> Result<String, JsValue> {
    code::run_code(code)
}
