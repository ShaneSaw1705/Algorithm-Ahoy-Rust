use js_sys::Array;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct GameState {
    vec: Vec<Vec<u8>>,
    player_pos: (i32, i32),
    player_dir: (i32, i32),
    levels: Vec<Level>,
}

#[wasm_bindgen]
pub struct Level {
    id: i32,
    board: Vec<Vec<u8>>,
    starting_pos: (i32, i32),
    dialoge: String,
}

#[wasm_bindgen]
impl GameState {
    pub fn new(size: i32) -> GameState {
        let mut board = vec![vec![b'0'; size as usize]; size as usize];
        let mid = (size / 2) as usize;
        board[mid][mid] = b'X';

        // I dont have time for a better soloution so here is every level hardcoded : )
        let levels: Vec<Level> = vec![
            Level {
                id: 1,
                board: vec![
                    vec![b'C', b'0', b'0', b'0', b'C'],
                    vec![b'0', b'0', b'0', b'0', b'0'],
                    vec![b'0', b'0', b'0', b'0', b'0'],
                    vec![b'0', b'0', b'0', b'0', b'0'],
                    vec![b'C', b'0', b'0', b'0', b'C'],
                ],
                starting_pos: (2, 2),
                dialoge: "Welcome to the first level collect all four coins in each corner".to_string(),
            },
            Level {
                id: 2,
                board: vec![
                    vec![b'#', b'#', b'#', b'0', b'0'],
                    vec![b'#', b'C', b'#', b'0', b'0'],
                    vec![b'0', b'0', b'0', b'0', b'0'],
                    vec![b'0', b'0', b'0', b'0', b'0'],
                    vec![b'0', b'0', b'0', b'0', b'0'],
                ],
                starting_pos: (2, 2),
                dialoge: "Welcome to the second level you need to navigate in to the room to grab the coin\n Remeber that you can get the neighbouring cell in any direction using
                    get_neighbour('north')".to_string(),
            },
            Level {
                id: 3,
                board: vec![
                    vec![b'0', b'0', b'0', b'C', b'C'],
                    vec![b'0', b'0', b'0', b'#', b'#'],
                    vec![b'0', b'0', b'0', b'#', b'C'],
                    vec![b'0', b'0', b'0', b'0', b'0'],
                    vec![b'0', b'0', b'0', b'0', b'0'],
                ],
                starting_pos: (2, 2),
                dialoge: "Welcome to the second level you need to navigate in to the room to grab the coin\n Remeber that you can get the neighbouring cell in any direction using
                    get_neighbour('north')".to_string(),
            },
        ];
        // adds a coin so it doesnt say you won the game straight away :)
        board[mid - 3][mid] = b'C';
        GameState {
            vec: board,
            player_pos: (mid as i32, mid as i32),
            player_dir: (-1, 0),
            levels,
        }
    }

    /// Returns the current coin count as its the easiest way I could think of "winning" the game
    pub fn get_coin_count(&self) -> i32 {
        let mut count = 0;
        for row in &self.vec {
            for &cell in row {
                if cell == b'C' {
                    count += 1;
                }
            }
        }
        count
    }

    /// Function to load a level(board)
    pub fn load_level(&mut self, level_num: i32) -> String {
        if let Some(level) = self.levels.iter().find(|&level| level.id == level_num) {
            self.vec = level.board.clone();
            self.update_player_pos(level.starting_pos.0, level.starting_pos.1);
        }
        return self.get_dialoge(level_num);
    }

    fn get_dialoge(&self, level_num: i32) -> String {
        if let Some(level) = self.levels.iter().find(|&level| level.id == level_num) {
            return level.dialoge.clone();
        }
        return "No dialoge found".to_string();
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
        let (old_x, old_y) = self.player_pos;

        if old_x >= 0
            && old_x < self.vec.len() as i32
            && old_y >= 0
            && old_y < self.vec[0].len() as i32
        {
            self.vec[old_x as usize][old_y as usize] = b'0';
        }

        if x >= 0 && x < self.vec.len() as i32 && y >= 0 && y < self.vec[0].len() as i32 {
            self.player_pos = (x, y);
            self.vec[x as usize][y as usize] = b'X';
        }
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
            // Check if the new position is not a wall
            if self.vec[new_x as usize][new_y as usize] != b'#' {
                self.update_player_pos(new_x, new_y);
            }
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

    pub fn get_neighbour(&self, direction: &str) -> JsValue {
        let (x, y) = self.player_pos;
        let (dx, dy) = match direction {
            "north" => (-1, 0),
            "south" => (1, 0),
            "east" => (0, 1),
            "west" => (0, -1),
            _ => (0, 0),
        };
        let new_x = x + dx;
        let new_y = y + dy;

        if new_x >= 0
            && new_x < self.vec.len() as i32
            && new_y >= 0
            && new_y < self.vec[0].len() as i32
        {
            let neighbour = self.vec[new_x as usize][new_y as usize];

            let result = match neighbour {
                b'#' => "wall",
                b'0' => "space",
                b'C' => "coin",
                _ => "unknown",
            };

            return JsValue::from_str(result);
        }

        JsValue::NULL
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
