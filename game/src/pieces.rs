#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub moved_once: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
            moved_once: false,
        }
    }
}
