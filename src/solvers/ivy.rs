use priority_queue::PriorityQueue;
use main::PuzzleType;

fn h(cube:PuzzleType) -> u16 {
    0
}

const POSSIBLE_MOVES: [[&str;3];6] = [["U ", "U' ", "U2 "],["D ", "D' ", "D2 "],["R ", "R' ", "R2 "],["L ", "L' ", "L2 "],["F ", "F' ", "F2 "],["B ", "B' ", "B2 "]];

struct Node {
    cube: PuzzleType,
    current_move: String,
    depth: u16
}

impl Node {
    fn new(cube:PuzzleType, current_move:String, depth:u16) -> Self {
        Self {
            cube: cube,
            current_move: current_move,
            depth: depth
        }
    }
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

pub fn IDAStar(cube:PuzzleType) {
    let mut node_stack = Vec::new();
    let mut moves: [&str; 21];
    let mut solved = false;
    let mut next_bound: u16 = h(cube);
    let mut bound = 0;

    while !solved {
        if node_stack.len() == 0 {
            node_stack.push(Node::new(cube, "", 0));
            bound = next_bound.clone();
            next_bound = std::f64::INFINITY;
        }
        let mut current_node: Node = node_stack.pop();
        if current_node.depth > 0 {
            moves[current_node.depth - 1] = current_node.current_move;
        }
        moves[current_node.depth] = "";

        if current_node.depth == bound {
            if current_node.cube.is_solved() {
                solved = true;
                return moves.join(" ")
            }
        } else {
            let mut successors = PriorityQueue::new();
            for x in POSSIBLE_MOVES {
                for y in x {
                    if current_node.depth == 0 || !prune(y, current_node.current_move.as_str()) {
                        let mut new_cube = current_node.cube.clone();
                        new_cube.input_scramble(y);
                        let estimated_moves = current_node.depth + h(new_cube);
                        if estimated_moves <= bound {
                            successors.push([new_cube, y], estimated_moves)
                        } else if estimated_moves < next_bound {
                            next_bound = estimated_moves;
                        }
                    }
                }
            }
            for (item, _) in successors.iter() {
                node_stack.push(Node::new(item[0], item[1], current_node.depth + 1))
            }
        }
    }
}