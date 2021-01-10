use std::convert::TryFrom;

use crate::{board::CastleState, Column, Square};
use crate::{Board, Color, Piece};

impl CastleState {
    fn to_fen(&self, color: Color) -> String {
        let s = match self {
            CastleState::Both => "kq",
            CastleState::Kingside => "k",
            CastleState::Queenside => "q",
            CastleState::None => "",
        }
        .to_string();

        if color == Color::White {
            s.to_uppercase()
        } else {
            s
        }
    }
}

impl Piece {
    fn to_fen(&self) -> String {
        let c: String = self.piece_type.into();

        if self.color == Color::White {
            c.to_uppercase()
        } else {
            c
        }
    }
}

impl Board {
    pub fn to_fen(&self) -> String {
        let mut fen_board_rows: [String; 8] = Default::default();

        for row in (0..8).rev() {
            let mut last_piece_distance = 0;
            let mut fen_row = String::default();

            for col in 0..8 {
                let sq = Square::new(Column::try_from(col).unwrap(), row);
                if let Some(piece) = self.at(&sq) {
                    if last_piece_distance > 0 {
                        fen_row += &format!("{}", last_piece_distance);
                    }

                    fen_row += &piece.to_fen();
                    last_piece_distance = 0;
                } else {
                    last_piece_distance += 1;
                }
            }

            if last_piece_distance > 0 {
                fen_row += &format!("{}", last_piece_distance);
            }
            fen_board_rows[7 - row] = fen_row;
        }

        let en_passant_square_notation = match self.en_passant_square.as_ref() {
            Some(s) => String::from(s.clone()),
            None => String::from("-"),
        };

        format!(
            "{} {} {}{} {} {} {}",
            fen_board_rows.join("/"),
            String::from(self.to_play),
            self.get_castle_state(Color::White).to_fen(Color::White),
            self.get_castle_state(Color::Black).to_fen(Color::Black),
            en_passant_square_notation,
            self.half_move_clock,
            self.full_move_clock
        )
    }
}
