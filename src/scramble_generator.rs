use rand::Rng;

pub fn main(n: u16) -> String {
    let available_moves = [
        ["U ", "U' ", "U2 "],
        ["D ", "D' ", "D2 "],
        ["R ", "R' ", "R2 "],
        ["L ", "L' ", "L2 "],
        ["F ", "F' ", "F2 "],
        ["B ", "B' ", "B2 "],
    ];
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

        let x = options[rand::thread_rng().gen_range(0..3)];

        for (j, current_move) in moves.iter().enumerate() {
            if current_move.contains(&x) {
                previous_move = j
            }
        }

        scramble += available_moves[x - 1][rand::thread_rng().gen_range(0..2)];
    }

    scramble[0..scramble.len() - 1].to_string()
}