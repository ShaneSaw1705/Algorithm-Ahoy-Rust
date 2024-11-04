use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct GameState {
    vec: Vec<u8>,
    player_pos: (i32, i32),
}

#[wasm_bindgen]
impl GameState {
    pub fn new(size: i32) -> GameState {
        let mut board = vec![b' '; (size * size) as usize];
        board[(size * size / 2 + size / 2) as usize] = b'X';
        GameState {
            vec: board,
            player_pos: (size / 2, size / 2),
        }
    }

    pub fn get_board(&self) -> Vec<u8> {
        self.vec.clone()
    }

    pub fn get_player_pos(&self) -> JsValue {
        to_value(&self.player_pos).unwrap()
    }
}
