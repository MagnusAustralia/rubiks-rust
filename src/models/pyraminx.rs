use std::collections::HashMap;
use colored::{Colorize, ColoredString};

use crate::models::Puzzle;

pub enum Faces {
    Blue,
    Red,
    Green,
    Yellow,
}

enum Tips {
    Top,
    Right,
    Left,
    Back
}

impl Faces {
    pub fn to_number(&self) -> usize {
        match self {
            Faces::Blue => 0,
            Faces::Red => 1,
            Faces::Green => 2,
            Faces::Yellow => 3,
        }
    }
}

pub struct Pyraminx {
    pub state: [[u8; 9]; 4]
}

impl Default for Pyraminx {
    fn default() -> Self {
        Self {
            state: [[0u8; 9], [1u8; 9], [2u8; 9], [3u8; 9]]
        }
    }
}

impl Pyraminx {
    fn rotate(&mut self, face: Faces, magnitude: u32) {
        let number = face.to_number();
        let corners = [0, 4, 7];
        let edges = [1, 2, 3, 5, 6, 8];
        let rotation = (magnitude % 8) as usize;
        let mut rotation_corners = [0u8;3];
        let mut rotation_edges = [0u8;6];
        
        for (i, &piece) in corners.iter().enumerate() {
            rotation_corners[i] = self.state[number][piece]
        }
        
        rotation_corners.rotate_right(rotation);
        
        for (i, &piece) in corners.iter().enumerate() {
            self.state[number][piece] = rotation_corners[i]
        }

        for (i, &piece) in edges.iter().enumerate() {
            rotation_edges[i] = self.state[number][piece]
        }
        
        rotation_edges.rotate_right(rotation * 2);
        
        for (i, &piece) in edges.iter().enumerate() {
            self.state[number][piece] = rotation_edges[i]
        }

        let adjacent = match face {
            Faces::Blue => [(2, [4, 3, 5, 6, 7]), (3, [0, 1, 2, 3, 4]), (1, [0, 1, 2, 3, 4])],
            Faces::Red => [(0, [4, 3, 5, 6, 7]), (3, [4, 3, 5, 6, 7]), (2, [0, 1, 2, 3, 4])],
            Faces::Green => [(1, [7, 6, 5, 3, 4]), (3, [1, 0, 8, 6, 7]), (0, [4, 3, 2, 1, 0])],
            Faces::Yellow => [(0, [7, 6, 8, 0, 1]), (2, [7, 6, 8, 0, 1]), (1, [7, 6, 8, 0, 1])]
        };

        let mut adjacent_cubies = [[0u8; 5];3];

        for (i, x) in adjacent.iter().enumerate() {
            let mut group_of_cubies = [0u8; 5];
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

    fn rotate_tip(&mut self, tip: Tips, magnitude: u32) {
        let rotation = (magnitude % 8) as usize;

        let tips = match tip {
            Tips::Top => [(0, 4), (1, 4), (2, 4)],
            Tips::Right => [(0, 7), (3, 4), (1, 0)],
            Tips::Left => [(0, 0), (2, 7), (3, 0)],
            Tips::Back => [(1, 7), (3, 7), (2, 0)]
        };

        let mut adjacent_edges = [0u8;3];

        for (i, x) in tips.iter().enumerate() {
            adjacent_edges[i] = self.state[x.0][x.1];
        }

        adjacent_edges.rotate_left(rotation);

        for (i, x) in tips.iter().enumerate() {
            self.state[x.0][x.1] = adjacent_edges[i];
        }
    }

    pub fn input_moves(&mut self, moves: &str) {
        let moves = moves.to_uppercase();
        let moves_list: Vec<&str> = moves.split_whitespace().collect();

        for x in moves_list {
            let face: Option<Faces> = match x.chars().next() {
                Some('F') => Some(Faces::Blue),
                Some('R') => Some(Faces::Red),
                Some('L') => Some(Faces::Green),
                Some('D') => Some(Faces::Yellow),
                _ => None,
            };

            let magnitude: u32 = match x.chars().nth(1) {
                Some('\'') => 2,
                Some('2') => 2,
                _ => 1
            };

            match face {
                Some(face_enum) => self.rotate(face_enum, magnitude),
                None => {
                    let tip: Option<Tips> = match x.chars().next() {
                        Some('T') => Some(Tips::Top),
                        Some('E') => Some(Tips::Right),
                        Some('K') => Some(Tips::Left),
                        Some('B') => Some(Tips::Back),
                        _ => None,
                    };

                    match tip {
                        Some(tip_enum) => self.rotate_tip(tip_enum, magnitude),
                        None => {break;}
                    }
                }
            }
        }
    }
}

impl Puzzle for Pyraminx {
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
        self.state == [[0u8; 9], [1u8; 9], [2u8; 9], [3u8; 9]]
    }

    fn print(&self) {
        let mut print_state: Vec<Vec<ColoredString>> = vec![vec![" ".white(); 32]; 11];
        let print_map: HashMap<_, _> = vec![
            ((0, 0),(4, 11)), ((0, 1),(3, 14)), ((0, 2),(2, 13)), ((0, 3),(2, 15)), ((0, 4),(0, 15)), ((0, 5),(2, 17)), ((0, 6),(3, 16)), ((0, 7),(4, 19)), ((0, 8),(4, 15)), 
            ((1, 0),(4, 23)), ((1, 1),(2, 24)), ((1, 2),(2, 21)), ((1, 3),(1, 23)), ((1, 4),(0, 19)), ((1, 5),(0, 24)), ((1, 6),(1, 26)), ((1, 7),(0, 30)), ((1, 8),(2, 28)), 
            ((2, 0),(0, 0)), ((2, 1),(1, 5)), ((2, 2),(0, 6)), ((2, 3),(1, 8)), ((2, 4),(0, 11)), ((2, 5),(2, 9)), ((2, 6),(2, 7)), ((2, 7),(4, 7)), ((2, 8),(2, 4)), 
            ((3, 0),(6, 11)), ((3, 1),(7, 14)), ((3, 2),(6, 15)), ((3, 3),(7, 16)), ((3, 4),(6, 19)), ((3, 5),(8, 17)), ((3, 6),(8, 15)), ((3, 7),(10, 15)), ((3, 8),(8, 13))].into_iter().collect();

        for i in 0..4 {
            for j in 0..9 {
                let (x, y) = print_map[&(i, j)];
                print_state[x][y] = match self.state[i][j] {0 => "B".blue(), 1 => "R".red(), 2 => "G".green(), 3=> "Y".yellow(), _ => " ".white()}
            }
        }

        for row in print_state {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}