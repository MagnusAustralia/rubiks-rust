use std::collections::HashMap;
use colored::{Colorize, ColoredString};

use crate::models::Puzzle;

pub enum Faces {
    White,
    Blue,
    Yellow,
    Purple,
    Green,
    Red,
    Lime,
    Orange,
    Sky,
    Beige,
    Pink,
    Gray
}

impl Faces {
    pub fn to_number(&self) -> usize {
        match self {
            Faces::White => 0,
            Faces::Blue => 1,
            Faces::Yellow => 2,
            Faces::Purple => 3,
            Faces::Green => 4,
            Faces::Red => 5,
            Faces::Lime => 6,
            Faces::Orange => 7,
            Faces::Sky => 8,
            Faces::Beige => 9,
            Faces::Pink => 10,
            Faces::Gray => 11
        }
    }
}

pub struct Megaminx {
    pub state: [[u8; 10]; 12]
}

impl Default for Megaminx {
    fn default() -> Self {
        Self {
            state: [[0u8;10], [1u8; 10], [2u8; 10], [3u8; 10], [4u8; 10], [5u8; 10], [6u8; 10], [7u8; 10], [8u8; 10], [9u8; 10], [10u8; 10], [11u8; 10]]
        }
    }
}

impl Megaminx {
    fn rotate(&mut self, face: Faces, magnitude: u32) {
        let tiles = &mut self.state[face.to_number()];
        let rotation = (magnitude % 8) as usize;
        tiles.rotate_right(rotation * 2);

        let adjacent = match face {
            Faces::White => [(1, [2, 3, 4]), (2, [0, 1, 2]), (3, [8, 9, 0]), (4, [6, 7, 8]), (5, [4, 5, 6])],
            Faces::Blue => [(10, [0, 1, 2]), (6, [0, 1, 2]), (2, [8, 9, 0]), (0, [8, 9, 0]), (5, [6, 7, 8])],
            Faces::Yellow => [(6, [2, 3, 4]), (7, [2, 3, 4]), (3, [6, 7, 8]), (0, [6, 7, 8]), (1, [4, 5, 6])],
            Faces::Purple => [(7, [4, 5, 6]), (8, [4, 5, 6]), (4, [4, 5, 6]), (0, [4, 5, 6]), (2, [2, 3, 4])],
            Faces::Green => [(8, [6, 7, 8]), (9, [6, 7, 8]), (5, [2, 3, 4]), (0, [2, 3, 4]), (3, [0, 1, 2])],
            Faces::Red => [(9, [8, 9, 0]), (10, [8, 9, 0]), (1, [0, 1, 2]), (0, [0, 1, 2]), (4, [8, 9, 0])],
            Faces::Lime => [(11, [2, 3, 4]), (7, [0, 1, 2]), (2, [6, 7, 8]), (1, [6, 7, 8]), (10, [2, 3, 4])],
            Faces::Orange => [(11, [4, 5, 6]), (8, [2, 3, 4]), (3, [4, 5, 6]), (2, [4, 5, 6]), (6, [4, 5, 6])],
            Faces::Sky => [(11, [6, 7, 8]), (9, [4, 5, 6]), (4, [2, 3, 4]), (3, [2, 3, 4]), (7,[6, 7, 8])],
            Faces::Beige => [(11, [8, 9, 0]), (10, [6, 7, 8]), (5, [0, 1, 2]), (4, [0, 1, 2]), (8, [8, 9, 0])],
            Faces::Pink => [(11, [0, 1, 2]), (6, [8, 9, 0]), (1, [8, 9, 0]), (5, [8, 9, 0]), (9, [0, 1, 2])],
            Faces::Gray => [(10, [4, 5, 6]), (9, [2, 3, 4]), (8, [0, 1, 2]), (7, [8, 9, 0]), (6, [6, 7, 8])]
        };

        let mut adjacent_cubies = [[0u8; 3];5];

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

    pub fn input_moves(&mut self, moves: &str) {
        let moves = moves.to_uppercase();
        let moves_list: Vec<&str> = moves.split_whitespace().collect();

        for x in moves_list {
            let face: Option<Faces> = match x.chars().next() {
                Some('U') => Some(Faces::White),
                Some('F') => Some(Faces::Blue),
                Some('R') => Some(Faces::Yellow),
                Some('B') => Some(Faces::Purple),
                Some('V') => Some(Faces::Green),
                Some('L') => Some(Faces::Red),
                Some('P') => Some(Faces::Pink),
                Some('G') => Some(Faces::Lime),
                Some('O') => Some(Faces::Orange),
                Some('S') => Some(Faces::Sky),
                Some('J') => Some(Faces::Beige),
                Some('D') => Some(Faces::Gray),
                _ => None,
            };

            match face {
                Some(face_enum) => {
                    let magnitude: u32 = match x.chars().nth(1) {
                        Some('\'') => 4,
                        Some('2') => 2,
                        Some('3') => 3,
                        Some('4') => 4,
                        _ => 1
                    };
        
                    self.rotate(face_enum, magnitude);
                },
                None => {break;}
            }
        }
    }
}

impl Puzzle for Megaminx {
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
        self.state == [[0u8; 10], [1u8; 10], [2u8; 10], [3u8; 10], [4u8; 10], [5u8; 10], [6u8; 10], [7u8; 10], [8u8; 10], [9u8; 10], [10u8; 10], [11u8; 10]]
    }

    fn print(&self) {
        let mut print_state: Vec<Vec<ColoredString>> = vec![vec![" ".white(); 56]; 13];
        print_state[7][12] = "W".white();
        print_state[10][17] = "B".blue();
        print_state[5][20] = "Y".yellow();
        print_state[2][12] = ColoredString::from(format!("\x1b[38;5;91m{}\x1b[0m", "P"));
        print_state[5][4] = "G".green();
        print_state[10][7] = "R".red();
        print_state[5][34] = ColoredString::from(format!("\x1b[38;5;10m{}\x1b[0m", "L"));
        print_state[2][42] = ColoredString::from(format!("\x1b[38;5;208m{}\x1b[0m", "O"));
        print_state[5][50] = ColoredString::from(format!("\x1b[38;5;81m{}\x1b[0m", "S"));
        print_state[10][37] = ColoredString::from(format!("\x1b[38;5;200m{}\x1b[0m", "P"));
        print_state[10][47] = ColoredString::from(format!("\x1b[38;5;230m{}\x1b[0m", "B"));
        print_state[7][42] = ColoredString::from(format!("\x1b[38;5;15m{}\x1b[0m", "G"));

        let print_map: HashMap<_, _> = vec![
            ((0, 0), (9, 12)), ((0, 1), (8, 10)), ((0, 2), (7, 8)), ((0, 3), (6, 9)), ((0, 4), (5, 10)), ((0, 5), (5, 12)), ((0, 6), (5, 14)), ((0, 7), (6, 15)), ((0, 8), (7, 16)), ((0, 9), (8, 14)), 
            ((1, 0), (12, 15)), ((1, 1), (11, 14)), ((1, 2), (10, 13)), ((1, 3), (9, 15)), ((1, 4), (8, 17)), ((1, 5), (9, 19)), ((1, 6), (10, 21)), ((1, 7), (11, 20)), ((1, 8), (12, 19)), ((1, 9), (12, 17)), 
            ((2, 0), (7, 18)), ((2, 1), (6, 17)), ((2, 2), (5, 16)), ((2, 3), (4, 18)), ((2, 4), (3, 20)), ((2, 5), (4, 22)), ((2, 6), (5, 24)), ((2, 7), (6, 23)), ((2, 8), (7, 22)), ((2, 9), (7, 20)), 
            ((3, 0), (4, 10)), ((3, 1), (3, 9)), ((3, 2), (2, 8)), ((3, 3), (1, 10)), ((3, 4), (0, 12)), ((3, 5), (1, 14)), ((3, 6), (2, 16)), ((3, 7), (3, 15)), ((3, 8), (4, 14)), ((3, 9), (4, 12)), 
            ((4, 0), (7, 2)), ((4, 1), (6, 1)), ((4, 2), (5, 0)), ((4, 3), (4, 2)), ((4, 4), (3, 4)), ((4, 5), (4, 6)), ((4, 6), (5, 8)), ((4, 7), (6, 7)), ((4, 8), (7, 6)), ((4, 9), (7, 4)), 
            ((5, 0), (12, 5)), ((5, 1), (11, 4)), ((5, 2), (10, 3)), ((5, 3), (9, 5)), ((5, 4), (8, 7)), ((5, 5), (9, 9)), ((5, 6), (10, 11)), ((5, 7), (11, 10)), ((5, 8), (12, 9)), ((5, 9), (12, 7)), 
            ((6, 0), (7, 32)), ((6, 1), (6, 31)), ((6, 2), (5, 30)), ((6, 3), (4, 32)), ((6, 4), (3, 34)), ((6, 5), (4, 36)), ((6, 6), (5, 38)), ((6, 7), (6, 37)), ((6, 8), (7, 36)), ((6, 9), (7, 34)), 
            ((7, 0), (4, 40)), ((7, 1), (3, 39)), ((7, 2), (2, 38)), ((7, 3), (1, 40)), ((7, 4), (0, 42)), ((7, 5), (1, 44)), ((7, 6), (2, 46)), ((7, 7), (3, 45)), ((7, 8), (4, 44)), ((7, 9), (4, 42)), 
            ((8, 0), (7, 48)), ((8, 1), (6, 47)), ((8, 2), (5, 46)), ((8, 3), (4, 48)), ((8, 4), (3, 50)), ((8, 5), (4, 52)), ((8, 6), (5, 54)), ((8, 7), (6, 53)), ((8, 8), (7, 52)), ((8, 9), (7, 50)), 
            ((9, 0), (12, 45)), ((9, 1), (11, 44)), ((9, 2), (10, 43)), ((9, 3), (9, 45)), ((9, 4), (8, 47)), ((9, 5), (9, 49)), ((9, 6), (10, 51)), ((9, 7), (11, 50)), ((9, 8), (12, 49)), ((9, 9), (12, 47)), 
            ((10, 0), (12, 35)), ((10, 1), (11, 34)), ((10, 2), (10, 33)), ((10, 3), (9, 35)), ((10, 4), (8, 37)), ((10, 5), (9, 39)), ((10, 6), (10, 41)), ((10, 7), (11, 40)), ((10, 8), (12, 39)), ((10, 9), (12, 37)), 
            ((11, 0), (9, 42)), ((11, 1), (8, 40)), ((11, 2), (7, 38)), ((11, 3), (6, 39)), ((11, 4), (5, 40)), ((11, 5), (5, 42)), ((11, 6), (5, 44)), ((11, 7), (6, 45)), ((11, 8), (7, 46)), ((11, 9), (8, 44)), 
            ].into_iter().collect();

        for i in 0..12 {
            for j in 0..10 {
                let (x, y) = print_map[&(i, j)];
                print_state[x][y] = match self.state[i][j] {
                    0 => "W".white(),
                    1 => "B".blue(),
                    2 => "Y".yellow(),
                    3 => ColoredString::from(format!("\x1b[38;5;91m{}\x1b[0m", "P")),
                    4 => "G".green(),
                    5 => "R".red(),
                    6 => ColoredString::from(format!("\x1b[38;5;10m{}\x1b[0m", "L")),
                    7 => ColoredString::from(format!("\x1b[38;5;208m{}\x1b[0m", "O")),
                    8 => ColoredString::from(format!("\x1b[38;5;81m{}\x1b[0m", "S")),
                    9 => ColoredString::from(format!("\x1b[38;5;230m{}\x1b[0m", "B")),
                    10 => ColoredString::from(format!("\x1b[38;5;200m{}\x1b[0m", "P")),
                    11 => ColoredString::from(format!("\x1b[38;5;15m{}\x1b[0m", "G")),
                    _ => " ".white()
                }
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