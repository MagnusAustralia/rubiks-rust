use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::models::pyraminx::Pyraminx;
use crate::models::Puzzle;

const POSSIBLE_MOVES: [[[&str; 2]; 3]; 8] = [
    [["D", "D'"], ["D'", "D"], ["D2", "D2"]],
    [["R", "R'"], ["R'", "R"], ["R2", "R2"]],
    [["L", "L'"], ["L'", "L"], ["L2", "L2"]],
    [["F", "F'"], ["F'", "F"], ["F2", "F2"]],
    [["T", "T'"], ["T'", "T"], ["T2", "T2"]],
    [["E", "E'"], ["E'", "E"], ["E2", "E2"]],
    [["K", "K'"], ["K'", "K"], ["K2", "K2"]],
    [["B", "B'"], ["B'", "B"], ["B2", "B2"]],
];

lazy_static! {
    static ref KNOWN_STATES: std::sync::Mutex<HashMap<u128, u8>> = std::sync::Mutex::new(HashMap::new());
}

fn add_known_state(key: u128, value: u8) {
    let mut states = KNOWN_STATES.lock().unwrap();
    states.insert(key, value);
}

fn state_known(key: u128) -> bool {
    let states = KNOWN_STATES.lock().unwrap();
    states.contains_key(&key)
}

fn process_moves(initial_state: u128) -> Vec<u128> {
    let mut state = [[0u8; 9]; 4];
    let mut local_depth = Vec::new();
    let mut value = initial_state;
    for face in state.iter_mut().rev() {
        for piece in face.iter_mut().rev() {
            *piece = (value & 0b111) as u8;
            value >>= 3;
        }
    }
    let mut cube = Pyraminx::default();
    cube.state = state;

    for i in 0..8 {
        for j in 0..3 {
            cube.input_moves(POSSIBLE_MOVES[i][j][0]);
            let current_state = cube.return_state();
            if !state_known(current_state) {
                local_depth.push(current_state);
            }
            cube.input_moves(POSSIBLE_MOVES[i][j][1]);
        }
    }
    local_depth
}

fn bfs(mut depth: u8, mut current_depth: Vec<u128>, mut states_processed: u32) {
    let mut new_depth = Vec::new();

    while !current_depth.is_empty() {
        let state = current_depth.pop().unwrap();  // Safe to unwrap as it's ensured to not be empty
        let states = process_moves(state);

        for state in states {
            if !state_known(state) {
                new_depth.push(state);
                add_known_state(state, depth);
                states_processed += 1;
            }
        }

        if current_depth.is_empty() {
            println!("Depth: {}, States processed: {}", depth, states_processed);
            depth += 1;  // Move to the next depth
            current_depth = new_depth.clone();  // Copy new states to the next level
            new_depth.clear();  // Clear new_depth for the next round of processing
        }
    }
}

pub fn generate_db() {
    let mut current_depth = Vec::new();
    let depth = 1;
    let states_processed = 1;

    let cube = Pyraminx::default();
    let state = cube.return_state();
    add_known_state(state, 0);  // Initial state has depth 0
    println!("Depth: 0, States processed: {}", states_processed);
    current_depth.push(state);

    bfs(depth, current_depth, states_processed);
}
