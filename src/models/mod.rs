pub mod pyraminx;
pub mod rubiks_cube_2x2;
pub mod rubiks_cube;
pub mod megaminx;
pub mod skewb;

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

pub trait Puzzle {
    fn is_solved(&self) -> bool;
    fn print(&self);
    fn return_state(&self) -> u128;
}


pub enum PuzzleType {
    RubiksCube(rubiks_cube::RubiksCube),
    RubiksCube2x2(rubiks_cube_2x2::RubiksCube2x2),
    Skewb(skewb::Skewb),
    Pyraminx(pyraminx::Pyraminx),
    Megaminx(megaminx::Megaminx)
}

impl PuzzleType {
    pub fn is_solved(&self) -> bool {
        match self {
            PuzzleType::RubiksCube(cube) => cube.is_solved(),
            PuzzleType::RubiksCube2x2(cube) => cube.is_solved(),
            PuzzleType::Skewb(cube) => cube.is_solved(),
            PuzzleType::Pyraminx(pyraminx) => pyraminx.is_solved(),
            PuzzleType::Megaminx(megaminx) => megaminx.is_solved()
        }
    }

    pub fn print(&self) {
        match self {
            PuzzleType::RubiksCube(cube) => cube.print(),
            PuzzleType::RubiksCube2x2(cube) => cube.print(),
            PuzzleType::Skewb(cube) => cube.print(),
            PuzzleType::Pyraminx(pyraminx) => pyraminx.print(),
            PuzzleType::Megaminx(megaminx) => megaminx.print()
        }
    }

    pub fn return_state(&self) -> u128 {
        match self {
            PuzzleType::RubiksCube(cube) => cube.return_state(),
            PuzzleType::RubiksCube2x2(cube) => cube.return_state(),
            PuzzleType::Skewb(cube) => cube.return_state(),
            PuzzleType::Pyraminx(pyraminx) => pyraminx.return_state(),
            PuzzleType::Megaminx(megaminx) => megaminx.return_state()
        }
    }

    pub fn input_moves(&mut self, moves: &str) {
        match self {
            PuzzleType::RubiksCube(cube) => cube.input_moves(moves),
            PuzzleType::RubiksCube2x2(cube) => cube.input_moves(moves),
            PuzzleType::Skewb(cube) => cube.input_moves(moves),
            PuzzleType::Pyraminx(pyraminx) => pyraminx.input_moves(moves),
            PuzzleType::Megaminx(megaminx) => megaminx.input_moves(moves)
        }
    }
}