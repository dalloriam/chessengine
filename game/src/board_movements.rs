use snafu::ensure;

use crate::board::*;
use crate::constants::*;
use crate::{Color, Piece, Square};

impl Board {
    pub(crate) fn validate_line_clear(
        &self,
        src: &Square,
        dst: &Square,
        col_delta: i32,
        row_delta: i32,
    ) -> Result<(), MoveError> {
        let mut s = src.clone();
        loop {
            s = s.relative(col_delta, row_delta).unwrap();
            if &s == dst {
                break;
            }
            ensure!(self.at(&s).is_none(), PathObstructed)
        }

        Ok(())
    }

    pub(crate) fn validate_line_threat(
        &self,
        src: &Square,
        dst: &Square,
        col_delta: i32,
        row_delta: i32,
        by_color: Color,
    ) -> Result<(), MoveError> {
        let mut s = src.clone();
        ensure!(
            !self.validate_square_threatened(src, by_color),
            CannotCastle
        );

        loop {
            s = s.relative(col_delta, row_delta).unwrap();
            if &s == dst {
                break;
            }
            ensure!(
                !self.validate_square_threatened(&s, by_color),
                CannotCastleThroughCheck
            );
        }

        Ok(())
    }

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
        let col_diff = usize::from(dst.col) as i32 - usize::from(src.col) as i32;
        let col_diff_abs = col_diff.abs();

        let row_diff = dst.row as i32 - src.row as i32;
        let row_diff_abs = row_diff.abs();

        // Validate we're actually moving in a straight diagonal.
        ensure!(col_diff_abs == row_diff_abs, InvalidMove);

        // Validate the line is clear.
        let direction_col = col_diff / col_diff_abs;
        let direction_row = row_diff / row_diff_abs;
        self.validate_line_clear(&src, dst, direction_col, direction_row)?;

        Ok(())
    }

    pub(crate) fn validate_king(&self, src: &Square, dst: &Square) -> Result<(), MoveError> {
        let coll_diff = usize::from(dst.col) as i32 - usize::from(src.col) as i32;
        let coll_diff_abs = coll_diff.abs();
        let row_diff_abs = ((src.row as i16) - (dst.row as i16)).abs();

        if coll_diff_abs == 2 {
            // Validate castling.
            ensure!(row_diff_abs == 0, InvalidMove); // Don't allow vertical movements when castling.

            // Get the square of the right rook.
            let castle_state = self.get_castle_state(self.at(src).unwrap().color);
            let attempted_castle = {
                if coll_diff > 0 {
                    CastleState::Kingside
                } else {
                    CastleState::Queenside
                }
            };
            ensure!(
                castle_state == CastleState::Both || castle_state == attempted_castle,
                CannotCastle
            );
        } else {
            // Validate that the king can move one space in any direction.
            ensure!(coll_diff_abs <= 1 && row_diff_abs <= 1, InvalidMove);
        }

        Ok(())
    }

    pub(crate) fn validate_rook(&self, src: &Square, dst: &Square) -> Result<(), MoveError> {
        let col_diff = usize::from(dst.col) as i32 - usize::from(src.col) as i32;
        let coll_diff_abs = col_diff.abs();

        let row_diff = dst.row as i32 - src.row as i32;
        let row_diff_abs = row_diff.abs();

        // Validate that movement is in a straight line.
        ensure!(coll_diff_abs == 0 || row_diff_abs == 0, InvalidMove);

        // Validate the line is clear.
        let direction_col = {
            if coll_diff_abs == 0 {
                0
            } else {
                col_diff / coll_diff_abs
            }
        };
        let direction_row = {
            if row_diff_abs == 0 {
                0
            } else {
                row_diff / row_diff_abs
            }
        };
        self.validate_line_clear(&src, &dst, direction_col, direction_row)?;

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
        let target_piece_maybe = self.at(dst);

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
