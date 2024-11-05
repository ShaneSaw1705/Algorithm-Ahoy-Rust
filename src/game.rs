use js_sys::Array;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct GameState {
    vec: Vec<Vec<u8>>,
    player_pos: (i32, i32),
    player_dir: (i32, i32),
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
            player_dir: (-1, 0),
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

    fn update_player_pos(&mut self, x: i32, y: i32) {
        // Gets the current position of the player before updating it
        let (old_x, old_y) = self.player_pos;

        //remove the player
        self.vec[old_x as usize][old_y as usize] = b'0';

        //update the player position
        self.player_pos = (x, y);

        //add the player to the new position
        self.vec[x as usize][y as usize] = b'X';
    }

    fn update_player_dir(&mut self, x: i32, y: i32) {
        self.player_dir = (x, y);
    }

    /// Move the player forward in the direction they are facing
    pub fn move_forward(&mut self) {
        let (x, y) = self.player_pos;
        let (dx, dy) = self.player_dir;
        let new_x = x + dx;
        let new_y = y + dy;
        if new_x >= 0
            && new_x < self.vec.len() as i32
            && new_y >= 0
            && new_y < self.vec.len() as i32
        {
            self.update_player_pos(new_x, new_y);
        }
    }

    pub fn turn(&mut self, direction: &str) {
        let (dx, dy) = self.player_dir;
        let new_dir = match direction {
            "right" => (dy, -dx),
            "left" => (-dy, dx),
            _ => (0, 0),
        };
        self.update_player_dir(new_dir.0, new_dir.1);
    }

    pub fn get_neighbouring_cells(&self) -> Array {
        // Initialize an empty array to store the neighbours in the type of a JsArray
        let neighbours = Array::new();
        let (x, y) = self.player_pos;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        // Iterate over the directions and check if the new position is within the board
        for (dx, dy) in directions {
            let new_x = x + dx;
            let new_y = y + dy;
            if new_x >= 0
                && new_x < self.vec.len() as i32
                && new_y >= 0
                && new_y < self.vec.len() as i32
            {
                // If the new position is within the board, add it to the neighbours array
                let neighbour = (new_x, new_y);
                neighbours.push(&to_value(&neighbour).unwrap());
            }
        }
        return neighbours;
    }
}
