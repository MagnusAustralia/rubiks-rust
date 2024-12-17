use std::collections::HashMap;
use std::io::{self, Write};
use colored::{Colorize, ColoredString};

pub mod scramble_generator;
// pub mod depth_first_search;

pub struct RubiksCube {
    state: [[u8; 8]; 6]
}

pub struct RubiksCube2x2 {
    state: [[u8; 4]; 6]
}

pub struct Pyraminx {
    state: [[u8; 9]; 4]
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

trait Puzzle {
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

impl Puzzle for RubiksCube {
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

impl Puzzle for RubiksCube2x2 {
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

impl Puzzle for Pyraminx {
    fn new() -> Self {
        Self {
            state: [[1u8; 9], [4u8; 9], [3u8; 9], [5u8; 9]],
        }
    }

    fn is_solved(&self) -> bool {
        self.state == [[1u8; 9], [4u8; 9], [3u8; 9], [5u8; 9]]
    }

    fn rotate(&mut self, face: Faces, magnitude: u32) {
        let number = match face {
            Faces::Blue => 0,
            Faces::Red => 1,
            Faces::Green => 2,
            Faces::Yellow => 3,
            _ => 4
        };
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
            Faces::Yellow => [(0, [7, 6, 8, 0, 1]), (2, [7, 6, 8, 0, 1]), (1, [7, 6, 8, 0, 1])],
            _ => [(10, [7, 6, 8, 0, 1]), (2, [7, 6, 8, 0, 1]), (1, [7, 6, 8, 0, 1])]
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

    fn input_moves(&mut self, moves: &str) {
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
                print_state[x][y] = match self.state[i][j] {1 => "B".blue(), 3 => "G".green(), 4 => "R".red(), 5 => "Y".yellow(), _ => " ".white()}
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

pub enum PuzzleType {
    RubiksCube(RubiksCube),
    RubiksCube2x2(RubiksCube2x2),
    Pyraminx(Pyraminx),
}

impl PuzzleType {
    fn is_solved(&self) -> bool {
        match self {
            PuzzleType::RubiksCube(cube) => cube.is_solved(),
            PuzzleType::RubiksCube2x2(cube) => cube.is_solved(),
            PuzzleType::Pyraminx(pyraminx) => pyraminx.is_solved()
        }
    }

    fn print(&self) {
        match self {
            PuzzleType::RubiksCube(cube) => cube.print(),
            PuzzleType::RubiksCube2x2(cube) => cube.print(),
            PuzzleType::Pyraminx(pyraminx) => pyraminx.print()
        }
    }

    fn input_moves(&mut self, moves: &str) {
        match self {
            PuzzleType::RubiksCube(cube) => cube.input_moves(moves),
            PuzzleType::RubiksCube2x2(cube) => cube.input_moves(moves),
            PuzzleType::Pyraminx(pyraminx) => pyraminx.input_moves(moves)
        }
    }
}

pub fn main() {
    let mut choice = String::new();
    print!("Enter 2 for a 2x2 and 3 for a 3x3 or anything else for a Pyraminx: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");
    let option: &str = choice.trim();

    let mut cube = match option {
        "2" => PuzzleType::RubiksCube2x2(RubiksCube2x2::new()),
        "3" => PuzzleType::RubiksCube(RubiksCube::new()),
        _ => PuzzleType::Pyraminx(Pyraminx::new())
    };

    let scramble = scramble_generator::main(50);
    println!("{scramble}");
    // cube.input_moves(scramble.as_str());
    cube.print();

    loop {
        let mut moves = String::new();
        
        print!("Enter moves: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut moves).expect("Failed to read input");
        moves = moves.trim().to_string();

        if moves == "c" {
            println!("{}", cube.is_solved())
        } else if moves == "p" {
            cube.print();
        } else {
            cube.input_moves(&moves);
            cube.print();
        }
    }
}