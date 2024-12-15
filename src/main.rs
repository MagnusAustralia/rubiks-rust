use std::collections::HashMap;
use std::io::{self, Write};
use colored::{Colorize, ColoredString};

pub mod scramble_generator;

pub struct RubiksCube {
    state: [[u8; 8]; 6]
}

pub struct RubiksCube2x2 {
    state: [[u8; 4]; 6]
}

pub enum Faces {
    White,
    Blue,
    Orange,
    Green,
    Red,
    Yellow
}

impl Faces {
    pub fn to_number(&self) -> usize {
        match self {
            Faces::White => 0,
            Faces::Blue => 1,
            Faces::Orange => 2,
            Faces::Green => 3,
            Faces::Red => 4,
            Faces::Yellow => 5,
        }
    }
}

trait Puzzel {
    fn new() -> Self;
    fn is_solved(&self) -> bool;
    fn rotate(&mut self, face:Faces, magnitude: u32);
    fn print(&self);
    fn input_moves(&mut self, moves: &str) {
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

impl Puzzel for RubiksCube {
    fn new() -> Self {
        Self {
            state: [[0u8; 8], [1u8; 8], [2u8; 8], [3u8; 8], [4u8; 8], [5u8; 8]],
        }
    }

    fn is_solved(&self) -> bool {
        self.state == [[0u8; 8], [1u8; 8], [2u8; 8], [3u8; 8], [4u8; 8], [5u8; 8]]
    }

    fn rotate(&mut self, face: Faces, magnitude: u32) {
        let tiles = &mut self.state[face.to_number()];
        let rotation = (magnitude % 8) as usize;
        tiles.rotate_right(rotation * 2);

        let adjacent = match face {
            Faces::White => [(3, [0, 1, 2]), (4, [0, 1, 2]), (1, [0, 1, 2]), (2, [0, 1, 2])],
            Faces::Blue => [(0, [4, 5, 6]), (4, [2, 3, 4]), (5, [0, 1, 2]), (2, [6, 7, 0])],
            Faces::Orange => [(0, [2, 3, 4]), (1, [2, 3, 4]), (5, [2, 3, 4]), (3, [6, 7, 0])],
            Faces::Green => [(0, [0, 1, 2]), (2, [2, 3, 4]), (5, [4, 5, 6]), (4, [6, 7, 0])],
            Faces::Red => [(0, [6, 7, 0]), (3, [2, 3, 4]), (5, [6, 7, 0]), (1, [6, 7, 0])],
            Faces::Yellow => [(1, [4, 5, 6]), (4, [4, 5, 6]), (3, [4, 5, 6]), (2, [4, 5, 6])]
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

        let print_map: HashMap<_, _> = vec![((0, 0), (0, 3)),((0, 1), (0, 4)),((0, 2), (0, 5)),((0, 3), (1, 5)),((0, 4), (2, 5)),((0, 5), (2, 4)),((0, 6), (2, 3)),((0, 7), (1, 3)),((1, 0), (3, 3)),((1, 1), (3, 4)),((1, 2), (3, 5)),((1, 3), (4, 5)),((1, 4), (5, 5)),((1, 5), (5, 4)),((1, 6), (5, 3)),((1, 7), (4, 3)),((2, 0), (3, 6)),((2, 1), (3, 7)),((2, 2), (3, 8)),((2, 3), (4, 8)),((2, 4), (5,8)),((2, 5), (5,7)),((2, 6), (5,6)),((2, 7), (4, 6)),((3, 0), (3,9)),((3, 1), (3,10)),((3, 2), (3,11)),((3, 3), (4,11)),((3, 4), (5,11)),((3, 5), (5,10)),((3, 6), (5,9)),((3, 7), (4, 9)),((4, 0), (3,0)),((4, 1), (3,1)),((4, 2), (3,2)),((4, 3), (4,2)),((4, 4), (5,2)),((4, 5), (5,1)),((4, 6), (5,0)),((4, 7), (4, 0)),((5, 0), (6,3)),((5, 1), (6,4)),((5, 2), (6,5)),((5, 3), (7, 5)),((5, 4), (8,5)),((5, 5), (8,4)),((5, 6), (8,3)),((5, 7), (7,3))].into_iter().collect();

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

impl Puzzel for RubiksCube2x2 {
    fn new() -> Self {
        Self {
            state: [[0u8; 4], [1u8; 4], [2u8; 4], [3u8; 4], [4u8; 4], [5u8; 4]],
        }
    }

    fn is_solved(&self) -> bool {
        self.state == [[0u8; 4], [1u8; 4], [2u8; 4], [3u8; 4], [4u8; 4], [5u8; 4]]
    }

    fn rotate(&mut self, face: Faces, magnitude: u32) {
        let tiles = &mut self.state[face.to_number()];
        let rotation = (magnitude % 8) as usize;
        tiles.rotate_right(rotation * 2);

        let adjacent = match face {
            Faces::White => [(3, [0, 1]), (4, [0, 1]), (1, [0, 1]), (2, [0, 1])],
            Faces::Blue => [(0, [2, 3]), (4, [2, 3]), (5, [0, 1]), (2, [3, 0])],
            Faces::Orange => [(0, [1, 2]), (1, [1, 2]), (5, [1, 2]), (3, [3, 0])],
            Faces::Green => [(0, [0, 1]), (2, [1, 2]), (5, [3, 4]), (4, [3, 0])],
            Faces::Red => [(0, [4, 0]), (3, [1, 2]), (5, [3, 0]), (1, [3, 0])],
            Faces::Yellow => [(1, [3, 4]), (4, [3, 4]), (3, [3, 4]), (2, [3, 4])]
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
        let mut print_state: Vec<Vec<ColoredString>> = vec![vec![" ".white(); 8]; 6];
        let print_map: HashMap<_, _> = vec![((0, 0), (0, 2)), ((0, 1), (0, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2)), ((1, 0), (2, 2)), ((1, 1), (2, 3)), ((1, 2), (3, 3)), ((1, 3), (3, 2)), ((2, 0), (2, 4)), ((2, 1), (2, 5)), ((2, 2), (3, 5)), ((2, 3), (3, 4)), ((3, 0), (2, 6)), ((3, 1), (2, 7)), ((3, 2), (3, 7)), ((3, 3), (3, 6)), ((4, 0), (2, 0)), ((4, 1), (2, 1)), ((4, 2), (3, 1)), ((4, 3), (3, 0)), ((5, 0), (4, 2)), ((5, 1), (4, 3)), ((5, 2), (5, 2)), ((5, 3), (5, 3))].into_iter().collect();

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
fn main() {
    let mut cube: RubiksCube2x2 = Puzzel::new();
    let scramble = scramble_generator::main(50);
    println!("{scramble}");
    cube.input_moves(scramble.as_str());
    cube.print();

    loop {
        let mut moves = String::new();
        
        print!("Enter moves: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut moves).expect("Failed to read input");
        moves = moves.trim().to_string();

        if moves == "c" {
            println!("{}", cube.is_solved())
        } else {
            cube.input_moves(&moves);
            cube.print();
        }
    }
}
