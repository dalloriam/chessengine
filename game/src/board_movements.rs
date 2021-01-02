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
            // TODO: Prise en passant.

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
