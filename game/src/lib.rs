mod board;
mod board_movements;
pub mod constants;
mod pieces;
mod setup;
mod square;

pub use board::Board;
pub use pieces::{Color, Piece, PieceType};
pub use square::{Column, Row, Square};

use square::Error as NotationError;

pub use board::MoveError;

pub use setup::board_with_setup;
