use std::collections::HashMap;
use colored::{Colorize, ColoredString};

use crate::models::{Puzzle, Faces};

pub struct RubiksCube {
    pub state: [[u8; 8]; 6]
}

impl Default for RubiksCube {
    fn default() -> Self {
        Self {
            state: [[0u8; 8], [1u8; 8], [2u8; 8], [3u8; 8], [4u8; 8], [5u8; 8]]
        }
    }
}

impl Puzzle for RubiksCube {
    fn return_state(&self) -> u128 {
        let mut state: u128 = 0;
        for face in self.state.iter() {
            for &piece in face.iter() {
                assert!(piece <= 7, "Piece value exceeds 3 bits!");
                state = (state << 3) | (piece as u128);
            }
        }
        state
    }

    fn is_solved(&self) -> bool {
        self.state == [[0u8; 8], [1u8; 8], [2u8; 8], [3u8; 8], [4u8; 8], [5u8; 8]]
    }

    fn rotate(&mut self, face: Faces, magnitude: u32) {
        let tiles = &mut self.state[face.to_number()];
        let rotation = (magnitude % 8) as usize;
        tiles.rotate_right(rotation * 2);

        let adjacent = match face {
            Faces::White => [(0, [])],
            Faces::Blue => [],
            Faces::Orange => [],
            Faces::Green => [],
            Faces::Red => [],
            Faces::Yellow => []
        };

        let mut adjacent_cubies = [[0u8; 3];4];

        for (i, x) in adjacent.iter().enumerate() {
            let mut group_of_cubies = [0u8; 3];
            for (j, &k) in x.1.iter().enumerate() {
                group_of_cubies[j] = self.state[x.0][k]
            }
            adjacent_cubies[i] = group_of_cubies
        }

        adjacent_cubies.rotate_left(rotation);

        for (i, x) in adjacent.iter().enumerate() {
            for (j, &k) in x.1.iter().enumerate() {
                self.state[x.0][k] = adjacent_cubies[i][j]
            }
        }        
    }

    fn print(&self) {
        let mut print_state: Vec<Vec<ColoredString>> = vec![vec![" ".white(); 12]; 9];
        print_state[1][4] = "W".white();
        print_state[4][4] = "B".blue();
        print_state[4][7] = ColoredString::from(format!("\x1b[38;5;208m{}\x1b[0m", "O"));
        print_state[4][10] = "G".green();
        print_state[4][1] = "R".red();
        print_state[7][4] = "Y".yellow();

        let print_map: HashMap<_, _> = vec![((), ())].into_iter().collect();

        for i in 0..6 {
            for j in 0..8 {
                let (x, y) = print_map[&(i, j)];
                print_state[x][y] = match self.state[i][j] {0 => "W".white(), 1 => "B".blue(), 2 => ColoredString::from(format!("\x1b[38;5;208m{}\x1b[0m", "O")), 3 => "G".green(), 4 => "R".red(), 5 => "Y".yellow(), _ => " ".white()}
            }
        }

                
        for row in print_state {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}