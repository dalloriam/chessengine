use std::{convert::TryFrom, num::ParseIntError};

use snafu::{ensure, ResultExt, Snafu};

use crate::{
    board::CastleState, constants::BOARD_DIMENSION, pieces::PieceError,
    square::Error as SquareError, Board, Color, Column, Piece, PieceType, Square,
};

#[derive(Debug, Snafu)]
pub enum FenError {
    InvalidFEN,
    InvalidRowCount,
    InvalidColCount,
    TooManyRows,
    TooManyCols,
    InvalidPiece { source: PieceError },
    InvalidColorToPlay { source: PieceError },
    InvalidEnPassant { source: SquareError },
    InvalidHalfMoveClock { source: ParseIntError },
    InvalidFullMoveClock { source: ParseIntError },
}

type Result<T> = std::result::Result<T, FenError>;

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
    pub fn from_fen<T: AsRef<str>>(fen: T) -> Result<Board> {
        let mut board = Board::default();

        // Make sure FEN string is split in 6 chunks.
        let fen_ref = fen.as_ref().trim();
        let chunks: Vec<_> = fen_ref.split(' ').collect();
        ensure!(chunks.len() == 6, InvalidFEN);

        // Load board state.
        let board_state = chunks[0];
        let mut row_count = 0;
        for row in board_state.split('/') {
            let row_index = BOARD_DIMENSION - 1 - row_count;
            let mut col_index: usize = 0;
            for ch in row.chars() {
                if ch.is_digit(10) {
                    // Empty spaces
                    col_index += ch.to_string().parse::<usize>().unwrap();
                } else {
                    // Piece.
                    let mut piece = Piece::from_notation(ch).context(InvalidPiece)?;
                    ensure!(col_index < BOARD_DIMENSION, TooManyCols);
                    ensure!(row_index < BOARD_DIMENSION, TooManyRows);
                    let square = Square::new(Column::try_from(col_index).unwrap(), row_index);

                    match piece.piece_type {
                        PieceType::King => match piece.color {
                            Color::White => {
                                board.white_king_position = square.clone();
                            }
                            Color::Black => {
                                board.black_king_position = square.clone();
                            }
                        },

                        PieceType::Rook => match piece.color {
                            Color::White => {
                                let rook_squares =
                                    &[Square::new(Column::A, 0), Square::new(Column::H, 0)];
                                if !rook_squares.contains(&square) {
                                    piece.moved_once = true;
                                }
                            }
                            Color::Black => {
                                let rook_squares =
                                    &[Square::new(Column::A, 7), Square::new(Column::H, 7)];
                                if !rook_squares.contains(&square) {
                                    piece.moved_once = true;
                                }
                            }
                        },
                        _ => {}
                    }

                    board.place_piece(&square, piece);
                    col_index += 1;
                }
            }
            row_count += 1;
            ensure!(col_index == 8, InvalidColCount);
        }
        ensure!(row_count == 8, InvalidRowCount);

        // Load to_play.
        board.to_play = Color::try_from(chunks[1].to_string()).context(InvalidColorToPlay)?;

        // Load castle state.
        {
            let white_queenside = chunks[2].contains('Q');
            let white_kingside = chunks[2].contains('K');
            let black_kingside = chunks[2].contains('k');
            let black_queenside = chunks[2].contains('q');

            if !(white_queenside || white_kingside) {
                // Can't castle at all, mark the king as moved.
                let white_king_pos = board.white_king_position.clone();
                let white_king = board.at_mut(&white_king_pos).as_mut().unwrap();
                white_king.moved_once = true;
            }

            if !(black_kingside || black_queenside) {
                // Can't castle at all, mark the king as moved.
                let black_king_pos = board.black_king_position.clone();
                let black_king = board.at_mut(&black_king_pos).as_mut().unwrap();
                black_king.moved_once = true;
            }

            if !white_queenside {
                if let Some(p) = board.at_mut(&Square::new(Column::A, 0)) {
                    p.moved_once = true
                }
            }
            if !white_kingside {
                if let Some(p) = board.at_mut(&Square::new(Column::H, 0)) {
                    p.moved_once = true
                }
            }

            if !black_kingside {
                if let Some(p) = board.at_mut(&Square::new(Column::H, 7)) {
                    p.moved_once = true
                }
            }

            if !black_queenside {
                if let Some(p) = board.at_mut(&Square::new(Column::A, 7)) {
                    p.moved_once = true
                }
            }
        }

        // Load en-passant square.
        if chunks[3] != "-" {
            // We have an en-passant square.
            let sq = Square::from_notation(chunks[3]).context(InvalidEnPassant)?;
            board.en_passant_square = Some(sq)
        }

        // Load half-move clock.
        board.half_move_clock = chunks[4].parse::<usize>().context(InvalidHalfMoveClock)?;

        // Load full-move clock.
        board.full_move_clock = chunks[5].parse::<usize>().context(InvalidFullMoveClock)?;

        Ok(board)
    }

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
