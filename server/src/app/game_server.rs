use std::sync::Mutex;

use anyhow::{anyhow, Result};

use game::Board;

pub struct GameServer {
    board: Mutex<Board>,
}

impl GameServer {
    pub fn new() -> Self {
        let board = Mutex::from(game::board_with_setup());
        Self { board }
    }

    pub fn do_move(
        &self,
        start_square_notation: &str,
        end_square_notation: &str,
    ) -> Result<String> {
        println!("{} => {}", start_square_notation, end_square_notation);
        let mut guard = self.board.lock().map_err(|_e| anyhow!("Mutex poisoned"))?;
        let board_ref = &mut *guard;
        let new_position = board_ref.move_notation(start_square_notation, end_square_notation)?;
        *board_ref = new_position;
        Ok(board_ref.to_fen())
    }

    pub fn get_position(&self) -> Result<String> {
        let guard = self.board.lock().map_err(|_e| anyhow!("Mutex poisoned"))?;
        let board_ref = &*guard;
        Ok(board_ref.to_fen())
    }
}
