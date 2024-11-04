use wasm_bindgen::prelude::*;
mod game;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn new_game(size: i32) -> game::GameState {
    game::GameState::new(size)
}
