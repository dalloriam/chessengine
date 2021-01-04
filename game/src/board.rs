use snafu::{ensure, ResultExt, Snafu};

use crate::{constants::BOARD_DIMENSION, Column};
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

    #[snafu(display("Invalid notation."))]
    InvalidSquare { source: NotationError },

    #[snafu(display("Invalid castling condition."))]
    CannotCastle,

    #[snafu(display("Cannot castle through check."))]
    CannotCastleThroughCheck,
}

type Result<T> = std::result::Result<T, MoveError>;

#[derive(Clone, Debug)]
pub struct Board {
    pub board: [[Option<Piece>; BOARD_DIMENSION]; BOARD_DIMENSION],

    black_king_position: Square,
    white_king_position: Square,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: Default::default(),
            black_king_position: Square::new(Column::E, 7),
            white_king_position: Square::new(Column::E, 0),
        }
    }
}

impl Board {
    pub fn at(&self, square: &Square) -> Option<&Piece> {
        self.board[usize::from(square.col)][square.row].as_ref()
    }

    pub(crate) fn at_mut(&mut self, square: &Square) -> &mut Option<Piece> {
        &mut self.board[usize::from(square.col)][square.row]
    }

    /// Clear all pieces from the board.
    pub fn clear(&mut self) {
        self.board = Default::default();
    }

    /// Place a piece on an empty square of the board.
    pub fn place_piece(&mut self, square: &Square, piece: Piece) {
        let piece_maybe = self.at_mut(square);
        assert!(piece_maybe.is_none());
        piece_maybe.replace(piece);
    }

    pub(crate) fn validate_square_threatened(&self, square: &Square, by_color: Color) -> bool {
        // Check for threats by knights.
        for col_move in &[1, 2, -1, -2] {
            for row_move in &[1, 2, -1, -2] {
                if let Some(attacker) = square.clone().relative(*col_move, *row_move) {
                    if let Some(atk_piece) = self.at(&attacker) {
                        if atk_piece.color == by_color {
                            return true;
                        }
                    }
                }
            }
        }

        // Check for lateral & diagonal threats.
        for col_delta in &[0, 1, -1] {
            for row_delta in &[0, 1, -1] {
                if *row_delta == 0 && *col_delta == 0 {
                    // Prevent an infinite loop.
                    continue;
                }

                let mut lat_square_pos = square.clone();
                while let Some(s) = lat_square_pos.relative(*col_delta, *row_delta) {
                    if let Some(atk_piece) = self.at(&s) {
                        if atk_piece.color != by_color {
                            // Line of sight is blocked by friendly piece.
                            break;
                        } else {
                            return true;
                        }
                    }
                    lat_square_pos = s;
                }
            }
        }

        return false;
    }

    fn validate_check(&self, color: Color) -> bool {
        match color {
            Color::Black => {
                self.validate_square_threatened(&self.black_king_position, Color::White)
            }
            Color::White => {
                self.validate_square_threatened(&self.white_king_position, Color::Black)
            }
        }
    }

    /// Validates that a move is legal.
    pub fn validate_move(&self, src: &Square, dst: &Square) -> Result<()> {
        // Validate that we have a piece to move.
        let piece_maybe = self.at(src);
        ensure!(piece_maybe.is_some(), NoPieceToMove);

        let piece = piece_maybe.unwrap();

        // Validate that the destination square is available.
        if let Some(dst_piece) = self.at(dst) {
            // If a capture, make sure we only capture the other side.
            ensure!(piece.color != dst_piece.color, DestinationObstructed);
        }

        // Piece-specific logic.
        match &piece.piece_type {
            PieceType::Knight => self.validate_knight(src, dst)?,
            PieceType::Pawn => self.validate_pawn(src, dst, piece)?,
            PieceType::Rook => self.validate_rook(src, dst)?,
            PieceType::King => self.validate_king(src, dst)?,
            PieceType::Bishop => self.validate_bishop(src, dst)?,
            PieceType::Queen => self.validate_queen(src, dst)?,
        }

        Ok(())
    }

    /// Move a piece from one square to another, respecting the rules of the game.
    ///
    /// Returns the next position, or an error if the move was invalid.
    pub fn move_piece(&self, src: &Square, dst: &Square) -> Result<Board> {
        self.validate_move(src, dst)?;

        let mut new_position = self.clone();

        let piece = new_position.at_mut(src).take().unwrap(); // Unwrap safe because validate move throws.
        let _captured_maybe = new_position.at_mut(dst).replace(piece.clone());
        // TODO: Do something with the captured piece.

        if piece.piece_type == PieceType::King {
            // Update the king position so we can make sure it's not in check.
            match &piece.color {
                Color::Black => {
                    new_position.black_king_position = dst.clone();
                }
                Color::White => new_position.white_king_position = dst.clone(),
            }
        }

        ensure!(!new_position.validate_check(piece.color), PutSelfInCheck);

        // Mark this piece as moved (used for tracking castling.)
        // TODO: Implement castling.
        new_position.at_mut(dst).as_mut().unwrap().moved_once = true;

        let coll_diff = usize::from(dst.col) as i32 - usize::from(src.col) as i32;
        if piece.piece_type == PieceType::King && coll_diff.abs() == 2 {
            // If we reached here we just castled.
            // We need to move the corresponding rook as well.
            let (rook_src, rook_dst) = {
                if coll_diff > 0 {
                    // Kingside castling.
                    (
                        Square::new(Column::H, src.row),
                        Square::new(Column::F, src.row),
                    )
                } else {
                    // Queenside castling
                    (
                        Square::new(Column::A, src.row),
                        Square::new(Column::D, src.row),
                    )
                }
            };

            // Move the rook.
            let rook = new_position.at_mut(&rook_src).take().unwrap();
            new_position.at_mut(&rook_dst).replace(rook);
        }

        Ok(new_position)
    }

    pub fn move_notation(&self, src: &str, dst: &str) -> Result<Board> {
        let src_square = Square::from_notation(src).context(InvalidSquare)?;
        let dst_square = Square::from_notation(dst).context(InvalidSquare)?;
        self.move_piece(&src_square, &dst_square)
    }
}
