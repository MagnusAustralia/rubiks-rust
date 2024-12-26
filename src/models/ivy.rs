use std::collections::HashMap;
use colored::{Colorize, ColoredString};

use crate::models::{Puzzle, Faces};

pub struct Ivy {
    pub state: [[u8;3];6]
}

impl Default for Ivy {
    fn default() -> Self {
        Self {
            state: [[0u8;3], [1u8;3], [2u8;3], [3u8;3], [4u8;3], [5u8;3]]
        }
    }
}

impl Ivy {
    fn rotate(&mut self, face: Faces, magnitude: u32) {
        let rotation = (magnitude % 8) as usize;

        let adjacent = match face {
            Faces::White => [(0, [1, 2]), (2, [1, 2]), (3, [1, 2])],
            Faces::Blue => [(0, [0, 1]), (4, [2, 1]), (1, [2, 1])],
            Faces::Red => [(1, [1, 0]), (5, [1, 2]), (2, [1, 0])],
            Faces::Green => [(4, [1, 0]), (3, [1, 0]), (5, [1, 0])],
            _ => return
        };

        let mut adjacent_cubies = [[0u8; 2];3];

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
                Some('R') => Some(Faces::Red),
                Some('L') => Some(Faces::Green),
                _ => None,
            };

            match face {
                Some(face_enum) => {
                    let magnitude: u32 = match x.chars().nth(1) {
                        Some('\'') => 2,
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

impl Puzzle for Ivy {
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
        self.state == [[0u8;3], [1u8;3], [2u8;3], [3u8;3], [4u8;3], [5u8;3]]
    }

    fn print(&self) {
        let mut print_state: Vec<Vec<ColoredString>> = vec![vec![" ".white(); 12]; 9];

        let print_map: HashMap<_, _> = vec![((0, 0), (2, 3)), ((0, 1), (1, 4)), ((0, 2), (0, 5)), ((1, 0), (5, 5)), ((1, 1), (4, 4)), ((1, 2), (3, 3)), ((2, 0), (5, 6)), ((2, 1), (4, 7)), ((2, 2), (3, 8)), ((3, 0), (5, 11)), ((3, 1), (4, 10)), ((3, 2), (3, 9)), ((4, 0), (5, 0)), ((4, 1), (4, 1)), ((4, 2), (3, 2)), ((5, 0), (8, 3)), ((5, 1), (7, 4)), ((5, 2), (6, 5))].into_iter().collect();

        for i in 0..6 {
            for j in 0..3 {
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