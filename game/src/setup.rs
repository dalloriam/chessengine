use crate::constants::*;
use crate::{Board, Color, Column, Piece, PieceType, Square};

const PIECES_ROW: &[PieceType] = &[
    PieceType::Rook,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Queen,
    PieceType::King,
    PieceType::Bishop,
    PieceType::Knight,
    PieceType::Rook,
];

pub fn board_with_setup() -> Board {
    let mut board = Board::default();
    setup_board(&mut board);
    board
}

fn setup_pawn_row(board: &mut Board, color: Color) {
    let row = match color {
        Color::White => WHITE_PAWN_ROW,
        Color::Black => BLACK_PAWN_ROW,
    };

    let mut square = Square::new(Column::A, row);
    loop {
        board.place_piece(&square, Piece::new(PieceType::Pawn, color));

        if let Some(new_sq) = square.next_col() {
            square = new_sq;
        } else {
            break;
        }
    }
}

fn setup_pieces_row(board: &mut Board, color: Color) {
    let row = match color {
        Color::White => WHITE_PIECES_ROW,
        Color::Black => BLACK_PIECES_ROW,
    };

    let mut square = Square::new(Column::A, row);
    let mut pieces_iter = PIECES_ROW.iter();

    loop {
        let piece_type = pieces_iter.next().unwrap();
        board.place_piece(&square, Piece::new(*piece_type, color));

        if let Some(new_sq) = square.next_col() {
            square = new_sq;
        } else {
            break;
        }
    }
}

pub fn setup_board(board: &mut Board) {
    board.clear();
    for color in &[Color::White, Color::Black] {
        setup_pawn_row(board, *color);
        setup_pieces_row(board, *color);
    }
}
