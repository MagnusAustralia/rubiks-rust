use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType}
};
use std::io::{self, stdout, Write};

mod models;
mod corner_database_generator;
mod scramble_generator;

fn select_puzzle() -> &'static str {
    let menu: &'static [&str] = &["2x2", "3x3", "Skewb", "Pyraminx", "Megaminx"];
    let mut selected = 0;

    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide).expect("Failed to initialize terminal");

    loop {
        execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))
            .expect("Failed to clear and reset cursor");

        for (i, item) in menu.iter().enumerate() {
            if i == selected {
                writeln!(stdout, "> {}", item).expect("Failed to write to stdout");
            } else {
                writeln!(stdout, "  {}", item).expect("Failed to write to stdout");
            }
        }

        stdout.flush().expect("Failed to flush stdout");

        if let Event::Key(key_event) = event::read().expect("Failed to read input") {
            match key_event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < menu.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    execute!(stdout, cursor::Show).expect("Failed to show cursor");
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    return menu[selected];
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let option = select_puzzle();
    println!("You selected: {}", option);

    let mut cube = match option {
        "2x2" => models::PuzzleType::RubiksCube2x2(models::rubiks_cube_2x2::RubiksCube2x2::default()),
        "3x3" => models::PuzzleType::RubiksCube(models::rubiks_cube::RubiksCube::default()),
        "Skewb" => models::PuzzleType::Skewb(models::skewb::Skewb::default()),
        "Pyraminx" => models::PuzzleType::Pyraminx(models::pyraminx::Pyraminx::default()),
        _ => models::PuzzleType::Megaminx(models::megaminx::Megaminx::default())
    };

    if option == "gcdb" {
        corner_database_generator::generate_db();
    }

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
        } else if moves == "rs" {
            println!("{:?}", cube.return_state());
        // } else if moves == "p" {
        //     cube.print();
        } else {
            cube.input_moves(&moves);
            cube.print();
            println!();
        }
    }
}