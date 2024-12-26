use std::io::{self, stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

mod models;
mod db_generators;
mod scramble_generator;

fn select_option() -> &'static str {
    let menu: &'static [&str] = &["2x2", "3x3", "Skewb", "Pyraminx", "Megaminx", "Ivy", "gcdb", "gsdb", "gidb", "gpdb"];
    let mut selected = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    for (i, option) in menu.iter().enumerate() {
        if i == selected {
            write!(stdout, "> {}\n\r", option).unwrap();
        } else {
            write!(stdout, "  {}\n\r", option).unwrap();
        }
    }
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        match c.unwrap() {
            Key::Up => {
                if selected > 0 {
                    selected -= 1;
                } else {
                    selected = menu.len() - 1
                }
            }
            Key::Down => {
                if selected < menu.len() - 1 {
                    selected += 1;
                } else {
                    selected = 0
                }
            }
            Key::Char('\n') => {
                write!(stdout, "{}You selected: {}\n\r", clear::All, menu[selected]).unwrap();
                write!(stdout, "{}", cursor::Show).unwrap();
                return menu[selected];
            }
            _ => {}
        }

        for (i, option) in menu.iter().enumerate() {
            if i == selected {
                write!(stdout, "> {}\n\r", option).unwrap();
            } else {
                write!(stdout, "  {}\n\r", option).unwrap();
            }
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", cursor::Show).unwrap();

    menu[selected]
}

fn main() {
    let option = select_option();

    let mut cube = match option {
        "2x2" => models::PuzzleType::RubiksCube2x2(models::rubiks_cube_2x2::RubiksCube2x2::default()),
        "3x3" => models::PuzzleType::RubiksCube(models::rubiks_cube::RubiksCube::default()),
        "Skewb" => models::PuzzleType::Skewb(models::skewb::Skewb::default()),
        "Pyraminx" => models::PuzzleType::Pyraminx(models::pyraminx::Pyraminx::default()),
        "Ivy" => models::PuzzleType::Ivy(models::ivy::Ivy::default()),
        _ => models::PuzzleType::Megaminx(models::megaminx::Megaminx::default())
    };

    match option {
        "gcdb" => {db_generators::corner_database_generator::generate_db();return;},
        "gsdb" => {db_generators::skewb_database_generator::generate_db();return;},
        "gidb" => {db_generators::ivy_database_generator::generate_db();return;},
        "gpdb" => {db_generators::pyaminx_database_generator::generate_db();return;},
        _ => {}
    }

    let scramble = scramble_generator::main(50, &cube);
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