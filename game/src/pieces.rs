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

impl From<Color> for String {
    fn from(c: Color) -> Self {
        match c {
            Color::White => "w",
            Color::Black => "b",
        }
        .to_string()
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

impl From<PieceType> for String {
    fn from(s: PieceType) -> Self {
        match s {
            PieceType::Pawn => "p",
            PieceType::Bishop => "b",
            PieceType::King => "k",
            PieceType::Knight => "n",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
        }
        .to_string()
    }
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
