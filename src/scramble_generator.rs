use rand::Rng;
use crate::models::PuzzleType;

fn get_array_size_for_puzzle(puzzle: &PuzzleType) -> Vec<Vec<&'static str>> {
    match puzzle {
        PuzzleType::RubiksCube(_) | PuzzleType::RubiksCube2x2(_) => vec![
            vec!["U ", "U' ", "U2 "],
            vec!["D ", "D' ", "D2 "],
            vec!["R ", "R' ", "R2 "],
            vec!["L ", "L' ", "L2 "],
            vec!["F ", "F' ", "F2 "],
            vec!["B ", "B' ", "B2 "],
        ],
        PuzzleType::Skewb(_) | PuzzleType::Ivy(_) => vec![
            vec!["U ", "U' ", "U2 "],
            vec!["R ", "R' ", "R2 "],
            vec!["L ", "L' ", "L2 "],
            vec!["F ", "F' ", "F2 "],
        ],
        PuzzleType::Pyraminx(_) => vec![
            vec!["D ", "D' ", "D2 "],
            vec!["R ", "R' ", "R2 "],
            vec!["L ", "L' ", "L2 "],
            vec!["F ", "F' ", "F2 "],
        ],
        PuzzleType::Megaminx(_) => vec![
            vec!["U ", "U' ", "U2 ", "U3 "],
            vec!["F ", "F' ", "F2 ", "F3 "],
            vec!["R ", "R' ", "R2 ", "R3 "],
            vec!["B ", "B' ", "B2 ", "B3 "],
            vec!["V ", "V' ", "V2 ", "V3 "],
            vec!["L ", "L' ", "L2 ", "L3 "],
            vec!["P ", "P' ", "P2 ", "P3 "],
            vec!["G ", "G' ", "G2 ", "G3 "],
            vec!["O ", "O' ", "O2 ", "O3 "],
            vec!["S ", "S' ", "S2 ", "S3 "],
            vec!["J ", "J' ", "J2 ", "J3 "],
            vec!["D ", "D' ", "D2 ", "D3 "],
        ],
    }
}

// fn get_moves_combos(puzzle: &PuzzleType) {
//     match puzzle {
//         PuzzleType::RubiksCube(_) | PuzzleType::RubiksCube2x2(_) => [[1, 2], [3, 4], [5, 6]],
//         PuzzleType::Skewb(_) | PuzzleType::Ivy(_) => [

//         ],
//         PuzzleType::Pyraminx(_) => vec![
//             vec!["D ", "D' ", "D2 "],
//             vec!["R ", "R' ", "R2 "],
//             vec!["L ", "L' ", "L2 "],
//             vec!["F ", "F' ", "F2 "],
//         ],
//         PuzzleType::Megaminx(_) => vec![
//             vec!["U ", "U' ", "U2 ", "U3 "],
//             vec!["F ", "F' ", "F2 ", "F3 "],
//             vec!["R ", "R' ", "R2 ", "R3 "],
//             vec!["B ", "B' ", "B2 ", "B3 "],
//             vec!["V ", "V' ", "V2 ", "V3 "],
//             vec!["L ", "L' ", "L2 ", "L3 "],
//             vec!["P ", "P' ", "P2 ", "P3 "],
//             vec!["G ", "G' ", "G2 ", "G3 "],
//             vec!["O ", "O' ", "O2 ", "O3 "],
//             vec!["S ", "S' ", "S2 ", "S3 "],
//             vec!["J ", "J' ", "J2 ", "J3 "],
//             vec!["D ", "D' ", "D2 ", "D3 "],
//         ],
//     }
// }

pub fn main(n: u16, puzzle:&PuzzleType) -> String {
    let available_moves = get_array_size_for_puzzle(puzzle);

    let moves = [[1, 2], [3, 4], [5, 6]];

    let mut scramble: String = String::new();
	let mut previous_move = 2;

    for _ in 0..n {
        let mut options = Vec::new();
        for group in moves {
            for num in group {
                if group != moves[previous_move] {
                    options.push(num)
                }
            }
        }

        let x = options[rand::thread_rng().gen_range(0..available_moves[0].len())];

        for (j, current_move) in moves.iter().enumerate() {
            if current_move.contains(&x) {
                previous_move = j
            }
        }

        scramble += available_moves[x - 1][rand::thread_rng().gen_range(0..available_moves[0].len())];
    }

    scramble[0..scramble.len() - 1].to_string()
}