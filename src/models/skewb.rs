use std::collections::HashMap;
use colored::{Colorize, ColoredString};

use crate::models::{Puzzle, Faces};

pub struct Skewb {
    pub state: [[u8;5];6]
}

impl Default for Skewb {
    fn default() -> Self {
        Self {
            state: [[0u8;5], [1u8;5], [2u8;5], [3u8;5], [4u8;5], [5u8;5]]
        }
    }
}

impl Skewb {
    fn rotate(&mut self, face: Faces, magnitude: u32) {
        let rotation = (magnitude % 8) as usize;

        let adjacent = match face {
            Faces::White => [(0, [0, 4, 1, 2]), (2, [0, 4, 1, 2]), (3, [3, 4, 0, 1])],
            Faces::Blue => [(0, [0, 4, 3, 2]), (4, [2, 4, 1, 0]), (1, [1, 4, 3, 0])],
            Faces::Red => [(0, [3, 4, 2, 1]), (1, [2, 4, 1, 0]), (2, [1, 4, 0, 3])],
            Faces::Green => [(0, [1, 4, 0, 3]), (3, [2, 4, 1, 0]), (4, [1, 4, 0, 3])],
            _ => return
        };

        let mut adjacent_cubies = [[0u8; 4];3];

        for (i, x) in adjacent.iter().enumerate() {
            let mut group_of_cubies = [0u8; 4];
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

        let corners: [(usize, usize); 3] = match face {
            Faces::White => [(1, 1), (5, 2), (4, 0)],
            Faces::Blue => [(2, 0), (3, 1), (5, 0)],
            Faces::Red => [(4, 1), (5, 1), (3, 0)],
            Faces::Green => [(1, 0), (2, 1), (5, 3)],
            _ => return
        };

        let mut adjacent_edges = [0u8;3];

        for (i, x) in corners.iter().enumerate() {
            adjacent_edges[i] = self.state[x.0][x.1];
        }

        adjacent_edges.rotate_left(rotation);

        for (i, x) in corners.iter().enumerate() {
            self.state[x.0][x.1] = adjacent_edges[i];
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

impl Puzzle for Skewb {
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
        self.state == [[0u8;5], [1u8;5], [2u8;5], [3u8;5], [4u8;5], [5u8;5]]
    }

    fn print(&self) {
        let mut print_state: Vec<Vec<ColoredString>> = vec![vec![" ".white(); 12]; 9];

        let print_map: HashMap<_, _> = vec![((0, 0), (0, 3)), ((0, 1), (0, 5)), ((0, 2), (2, 5)), ((0, 3), (2, 3)), ((0, 4), (1, 4)), ((1, 0), (3, 3)), ((1, 1), (3, 5)), ((1, 2), (5, 5)), ((1, 3), (5, 3)), ((1, 4), (4, 4)), ((2, 0), (3, 6)), ((2, 1), (3, 8)), ((2, 2), (5, 8)), ((2, 3), (5, 6)), ((2, 4), (4, 7)), ((3, 0), (3, 9)), ((3, 1), (3, 11)), ((3, 2), (5, 11)), ((3, 3), (5, 9)), ((3, 4), (4, 10)), ((4, 0), (3, 0)), ((4, 1), (3, 2)), ((4, 2), (5, 2)), ((4, 3), (5, 0)), ((4, 4), (4, 1)), ((5, 0), (6, 3)), ((5, 1), (6, 5)), ((5, 2), (8, 5)), ((5, 3), (8, 3)), ((5, 4), (7, 4))].into_iter().collect();

        for i in 0..6 {
            for j in 0..5 {
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