mod render;

use std::io::{self, Write};

use anyhow::{anyhow, Result};

use colored::*;

use game::{Board, Color};

pub fn prompt(msg: &str) -> io::Result<String> {
    print!("{}", msg.blue());
    let mut user_input = String::new();

    io::stdout().flush()?;
    io::stdin().read_line(&mut user_input)?;

    Ok(String::from(user_input.trim()))
}

fn single_turn(board: &mut Board) -> Result<Board> {
    let r = prompt("Enter your move: ").unwrap();
    let v: Vec<_> = r.split(" ").collect();
    if v.len() != 2 {
        return Err(anyhow!("Invalid move"));
    }

    let new_position = board.move_notation(v[0], v[1])?;
    Ok(new_position)
}

fn main() {
    let mut board = game::board_with_setup();

    loop {
        render::render_board(&board, Color::Black, true);
        loop {
            match single_turn(&mut board) {
                Ok(b) => {
                    // Turn successful, update the board.
                    board = b;
                    break;
                }
                Err(e) => {
                    println!("{}", e.to_string().red());
                }
            }
        }
    }
}
