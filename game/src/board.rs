use snafu::{ensure, ResultExt, Snafu};

use crate::constants::BOARD_DIMENSION;
use crate::{Color, NotationError, Piece, PieceType, Square};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum MoveError {
    #[snafu(display("There is no piece there!"))]
    NoPieceToMove,

    #[snafu(display("This piece doesn't move like that!"))]
    InvalidMove,

    #[snafu(display("You cannot take your own pieces."))]
    DestinationObstructed,

    #[snafu(display("The path is obstructed."))]
    PathObstructed,

    #[snafu(display("You can't put yourself in check."))]
    PutSelfInCheck,

    #[snafu(display("Invalid notation"))]
    InvalidSquare { source: NotationError },
}

type Result<T> = std::result::Result<T, MoveError>;

#[derive(Clone, Debug)]
pub struct Board {
    pub board: [[Option<Piece>; BOARD_DIMENSION]; BOARD_DIMENSION],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: Default::default(),
        }
    }
}

impl Board {
    /// Clear all pieces from the board.
    pub fn clear(&mut self) {
        self.board = Default::default();
    }

    /// Place a piece on an empty square of the board.
    pub fn place_piece(&mut self, square: &Square, piece: Piece) {
        assert!(self.board[usize::from(square.col)][square.row].is_none());
        self.board[usize::from(square.col)][square.row] = Some(piece);
    }

    fn validate_check(&self) -> Option<Color> {
        None
    }

    /// Validates that a move is legal.
    pub fn validate_move(&self, src: &Square, dst: &Square) -> Result<()> {
        // Validate that we have a piece to move.
        let piece_maybe = self.board[usize::from(src.col)][src.row].as_ref();
        ensure!(piece_maybe.is_some(), NoPieceToMove);

        let piece = piece_maybe.unwrap();

        // Validate that the destination square is available.
        if let Some(dst_piece) = self.board[usize::from(dst.col)][dst.row].as_ref() {
            // If a capture, make sure we only capture the other side.
            ensure!(piece.color != dst_piece.color, DestinationObstructed);
        }

        // Piece-specific logic.
        match &piece.piece_type {
            PieceType::Knight => self.validate_knight(src, dst)?,
            PieceType::Pawn => self.validate_pawn(src, dst, piece)?,
            _ => {
                unimplemented!()
            }
        }

        Ok(())
    }

    /// Move a piece from one square to another, respecting the rules of the game.
    ///
    /// Returns the next position, or an error if the move was invalid.
    pub fn move_piece(&self, src: &Square, dst: &Square) -> Result<Board> {
        self.validate_move(src, dst)?;

        let mut new_position = self.clone();

        let piece = new_position.board[usize::from(src.col)][src.row]
            .take()
            .unwrap(); // Unwrap safe because validate move throws.
        let piece_color = piece.color;
        new_position.board[usize::from(dst.col)][dst.row] = Some(piece);

        if let Some(checked_color) = self.validate_check() {
            ensure!(checked_color != piece_color, PutSelfInCheck);
        }

        Ok(new_position)
    }

    pub fn move_notation(&self, src: &str, dst: &str) -> Result<Board> {
        let src_square = Square::from_notation(src).context(InvalidSquare)?;
        let dst_square = Square::from_notation(dst).context(InvalidSquare)?;
        self.move_piece(&src_square, &dst_square)
    }
}
