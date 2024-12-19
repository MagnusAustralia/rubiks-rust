use std::collections::HashMap;
use colored::{Colorize, ColoredString};

use crate::models::{Puzzle, Faces};

pub struct RubiksCube2x2 {
    pub state: [[u8; 4]; 6]
}

impl Default for RubiksCube2x2 {
    fn default() -> Self {
        Self {
            state: [[0u8; 4],[1u8; 4],[2u8; 4],[3u8; 4],[4u8; 4],[5u8; 4]],
        }
    }
}

impl RubiksCube2x2 {
    fn rotate(&mut self, face: Faces, magnitude: u32) {
        let tiles = &mut self.state[face.to_number()];
        let rotation = (magnitude % 8) as usize;
        tiles.rotate_right(rotation);

        let adjacent = match face {
            Faces::White => [(3, [0, 1]), (4, [0, 1]), (1, [0, 1]), (2, [0, 1])],
            Faces::Blue => [(0, [2, 3]), (4, [1, 2]), (5, [0, 1]), (2, [3, 0])],
            Faces::Orange => [(0, [1, 2]), (1, [1, 2]), (5, [1, 2]), (3, [3, 0])],
            Faces::Green => [(0, [0, 1]), (2, [1, 2]), (5, [2, 3]), (4, [3, 0])],
            Faces::Red => [(0, [3, 0]), (3, [1, 2]), (5, [3, 0]), (1, [3, 0])],
            Faces::Yellow => [(1, [2, 3]), (4, [2, 3]), (3, [2, 3]), (2, [2, 3])]
        };

        let mut adjacent_cubies = [[0u8; 2];4];

        for (i, x) in adjacent.iter().enumerate() {
            let mut group_of_cubies = [0u8; 2];
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

    pub fn input_moves(&mut self, moves: &str) {
        let moves = moves.to_uppercase();
        let moves_list: Vec<&str> = moves.split_whitespace().collect();

        for x in moves_list {
            let face: Option<Faces> = match x.chars().next() {
                Some('U') => Some(Faces::White),
                Some('F') => Some(Faces::Blue),
                Some('R') => Some(Faces::Orange),
                Some('B') => Some(Faces::Green),
                Some('L') => Some(Faces::Red),
                Some('D') => Some(Faces::Yellow),
                _ => None,
            };

            match face {
                Some(face_enum) => {
                    let magnitude: u32 = match x.chars().nth(1) {
                        Some('\'') => 3,
                        Some('2') => 2,
                        _ => 1
                    };
        
                    self.rotate(face_enum, magnitude);
                },
                None => {break;}
            }
        }
    }
}

impl Puzzle for RubiksCube2x2 {
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
        self.state == [[0u8; 4], [1u8; 4], [2u8; 4], [3u8; 4], [4u8; 4], [5u8; 4]]
    }

    fn print(&self) {
        let mut print_state: Vec<Vec<ColoredString>> = vec![vec![" ".white(); 8]; 6];
        let print_map: HashMap<_, _> = vec![((0, 0), (0, 2)), ((0, 1), (0, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2)), ((1, 0), (2, 2)), ((1, 1), (2, 3)), ((1, 2), (3, 3)), ((1, 3), (3, 2)), ((2, 0), (2, 4)), ((2, 1), (2, 5)), ((2, 2), (3, 5)), ((2, 3), (3, 4)), ((3, 0), (2, 6)), ((3, 1), (2, 7)), ((3, 2), (3, 7)), ((3, 3), (3, 6)), ((4, 0), (2, 0)), ((4, 1), (2, 1)), ((4, 2), (3, 1)), ((4, 3), (3, 0)), ((5, 0), (4, 2)), ((5, 1), (4, 3)), ((5, 2), (5, 3)), ((5, 3), (5, 2))].into_iter().collect();

        for i in 0..6 {
            for j in 0..4 {
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