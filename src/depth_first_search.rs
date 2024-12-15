// Link to Corner Perms
// Link to Edge Perms

use std::cmp;
use std::collections::HashMap;
use main::PuzzleType;

fn movesTillCornersComplete(cube:PuzzleType) -> u32 {
    // open the corner perms and return int(m[PermutationIndexer(cube.corners, cube.cornerOrientations)])
    0
}

fn movesTillEdgesComplete(cube:PuzzleType) -> u32 {
    0
}

fn h(cube:main::PuzzleType) {
    cmp::max(movesTillCornersComplete(cube), movesTillEdgesComplete(cube))
}

const POSSIBLE_MOVES: [[&str;3];6] = [["U ", "U' ", "U2 "],["D ", "D' ", "D2 "],["R ", "R' ", "R2 "],["L ", "L' ", "L2 "],["F ", "F' ", "F2 "],["B ", "B' ", "B2 "]];

struct Node {
    cube: PuzzleType,
    current_move: String,
    depth: u16
}

fn get_opposite_layer(layer: char) -> Option<char> {
    match layer {
        'U' => Some('D'),
        'D' => Some('U'),
        'R' => Some('L'),
        'L' => Some('R'),
        'F' => Some('B'),
        'B' => Some('F'),
        _ => None,
    }
}

fn prune(new_move: &str, last_move: &str) -> bool {
    let new_move_char = new_move.chars().next();
    [Some(last_move.chars().next()), Some(get_opposite_layer(new_move_char.unwrap()))].contains(&Some(new_move_char))
}