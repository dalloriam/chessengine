use std::convert::TryFrom;

use colored::*;

use game::{constants::BOARD_DIMENSION, Board, Color as GameColor, Column, Piece, PieceType};

fn render_piece(p: &Piece) -> String {
    match p.color {
        GameColor::White => match p.piece_type {
            PieceType::Pawn => "♙",
            PieceType::Rook => "♖",
            PieceType::Bishop => "♗",
            PieceType::Queen => "♕",
            PieceType::King => "♔",
            PieceType::Knight => "♘",
        },
        GameColor::Black => match p.piece_type {
            PieceType::Pawn => "♟︎",
            PieceType::Rook => "♜",
            PieceType::Bishop => "♝",
            PieceType::Queen => "♛",
            PieceType::King => "♚",
            PieceType::Knight => "♞",
        },
    }
    .to_string()
}

pub fn render_board(board: &Board, perspective: GameColor) {
    let mut board_repr = String::default();

    let row_range: Vec<_> = match perspective {
        GameColor::Black => (0..BOARD_DIMENSION).collect(),
        GameColor::White => (0..BOARD_DIMENSION).rev().collect(),
    };

    let mut coll_range = row_range.clone();
    coll_range.reverse();

    for row_idx in row_range.iter() {
        let mut row_str = String::default();
        for col_idx in coll_range.iter() {
            match &board.board[*col_idx][*row_idx] {
                Some(p) => row_str += &format!("{} ", render_piece(p)),
                None => {
                    row_str += ". ";
                }
            }
        }

        board_repr += &format!("{} | {}\n", row_idx + 1, row_str.black().on_white());
    }

    // Append columns.
    {
        let mut row_str = String::from("    ");
        let mut row_str_2 = String::from("   -");
        for col_idx in coll_range.iter() {
            let col = Column::try_from(*col_idx).unwrap(); // safe because BOARD_DIMENSION is always correct.
            row_str += &format!("{} ", col);
            row_str_2 += "--"
        }
        board_repr += &row_str_2;
        board_repr += "\n";
        board_repr += &row_str;
    }

    println!("{}", board_repr);
}
