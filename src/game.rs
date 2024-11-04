use js_sys::Array;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct GameState {
    vec: Vec<Vec<u8>>,
    player_pos: (i32, i32),
}

#[wasm_bindgen]
impl GameState {
    pub fn new(size: i32) -> GameState {
        let mut board = vec![vec![b'0'; size as usize]; size as usize];
        let mid = (size / 2) as usize;
        board[mid][mid] = b'X';
        GameState {
            vec: board,
            player_pos: (mid as i32, mid as i32),
        }
    }

    pub fn get_board(&self) -> Array {
        let array = Array::new();
        for row in &self.vec {
            let js_row = Array::new();
            for &cell in row {
                let character = cell as char;
                js_row.push(&JsValue::from(character.to_string()));
            }
            array.push(&js_row);
        }
        array
    }

    pub fn get_player_pos(&self) -> JsValue {
        to_value(&self.player_pos).unwrap()
    }
}
