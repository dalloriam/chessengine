use snafu::ensure;

use crate::board::*;
use crate::constants::*;
use crate::{Color, Piece, Square};

impl Board {
    pub(crate) fn validate_knight(&self, src: &Square, dst: &Square) -> Result<(), MoveError> {
        let coll_diff_abs = ((usize::from(src.col) as i16) - (usize::from(dst.col) as i16)).abs();
        let row_diff_abs = ((src.row as i16) - (dst.row as i16)).abs();

        ensure!(
            (coll_diff_abs == 1 && row_diff_abs == 2) || (coll_diff_abs == 2 && row_diff_abs == 1),
            InvalidMove
        );
        Ok(())
    }

    pub(crate) fn validate_queen(&self, src: &Square, dst: &Square) -> Result<(), MoveError> {
        // Make sure the queen moved either like a bishop or like a rook.
        ensure!(
            self.validate_bishop(src, dst).is_ok() || self.validate_rook(src, dst).is_ok(),
            InvalidMove
        );
        Ok(())
    }

    pub(crate) fn validate_bishop(&self, src: &Square, dst: &Square) -> Result<(), MoveError> {
        let col_diff = usize::from(dst.col) as i16 - usize::from(src.col) as i16;
        let coll_diff_abs = col_diff.abs();

        let row_diff = dst.row as i16 - src.row as i16;
        let row_diff_abs = row_diff.abs();

        let mut current_square = src.clone();

        // Validate we're actually moving in a straight diagonal.
        ensure!(coll_diff_abs == row_diff_abs, InvalidMove);

        loop {
            // Update the square.
            if col_diff > 0 {
                current_square = current_square.next_col().unwrap();
            } else {
                assert!(col_diff < 0);
                current_square = current_square.prev_col().unwrap();
            }

            if row_diff > 0 {
                current_square = current_square.next_row().unwrap();
            } else {
                assert!(row_diff < 0);
                current_square = current_square.prev_row().unwrap();
            }

            if &current_square == dst {
                break;
            }

            // Validate that no pieces are in the way.
            ensure!(
                self.board[usize::from(current_square.col)][current_square.row].is_none(),
                PathObstructed
            );
        }

        Ok(())
    }

    pub(crate) fn validate_king(&self, src: &Square, dst: &Square) -> Result<(), MoveError> {
        let coll_diff_abs = ((usize::from(src.col) as i16) - (usize::from(dst.col) as i16)).abs();
        let row_diff_abs = ((src.row as i16) - (dst.row as i16)).abs();

        // Validate that the king can move one space in any direction.
        ensure!(coll_diff_abs <= 1 && row_diff_abs <= 1, InvalidMove);

        // TODO: Implement castling.

        Ok(())
    }

    pub(crate) fn validate_rook(&self, src: &Square, dst: &Square) -> Result<(), MoveError> {
        let col_diff = usize::from(dst.col) as i16 - usize::from(src.col) as i16;
        let coll_diff_abs = col_diff.abs();

        let row_diff = dst.row as i16 - src.row as i16;
        let row_diff_abs = row_diff.abs();

        // Validate that movement is in a straight line.
        ensure!(coll_diff_abs == 0 || row_diff_abs == 0, InvalidMove);

        // Validate that there are no pieces on the way.
        let movement_magnitude = coll_diff_abs + row_diff_abs;

        let mut current_square = src.clone();
        for _i in 0..(movement_magnitude - 1) {
            // Update along the straight line.
            if row_diff > 0 {
                current_square = current_square.next_row().unwrap();
            } else if row_diff < 0 {
                current_square = current_square.prev_row().unwrap();
            } else if col_diff > 0 {
                current_square = current_square.next_col().unwrap();
            } else if col_diff < 0 {
                current_square = current_square.prev_col().unwrap();
            } else {
                panic!("invalid move")
            }

            // Check that the square is not occupied.
            ensure!(
                self.board[usize::from(current_square.col)][current_square.row].is_none(),
                PathObstructed
            );
        }

        Ok(())
    }

    pub(crate) fn validate_pawn(
        &self,
        src: &Square,
        dst: &Square,
        piece: &Piece,
    ) -> Result<(), MoveError> {
        let pawn_did_move = src.row
            != match piece.color {
                Color::Black => BLACK_PAWN_ROW,
                Color::White => WHITE_PAWN_ROW,
            };

        let row_diff = {
            let diff = dst.row as i16 - src.row as i16;

            // Make sure pawns can only move in one direction.
            if piece.color == Color::Black {
                ensure!(diff <= 0, InvalidMove);
            } else {
                ensure!(diff >= 0, InvalidMove);
            }
            diff.abs()
        };

        let coll_diff_abs = (usize::from(src.col) as i16 - usize::from(dst.col) as i16).abs();
        let target_piece_maybe = self.board[usize::from(dst.col)][dst.row].as_ref();

        if coll_diff_abs > 0 {
            // Handle captures.
            // TODO: Implement en passant.

            // Move must be a single diagonal.
            ensure!(coll_diff_abs == 1 && row_diff == 1, InvalidMove);

            // There has to be a piece of the opposite color on the target square.
            ensure!(
                target_piece_maybe.is_some() && target_piece_maybe.unwrap().color != piece.color,
                InvalidMove
            );
        } else {
            // Handle regular movement.
            if pawn_did_move {
                ensure!(row_diff == 1, InvalidMove);
            } else {
                ensure!((row_diff == 1 || row_diff == 2), InvalidMove);
            }

            // Validate there is no piece on the target square.
            ensure!(target_piece_maybe.is_none(), InvalidMove);
        }

        Ok(())
    }
}
