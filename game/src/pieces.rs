use std::convert::TryFrom;

use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PieceError {
    InvalidNotation,
}

type Result<T> = std::result::Result<T, PieceError>;

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
impl TryFrom<String> for Color {
    type Error = PieceError;
    fn try_from(c: String) -> Result<Color> {
        match c.to_lowercase().as_ref() {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(PieceError::InvalidNotation),
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

impl TryFrom<String> for PieceType {
    type Error = PieceError;

    fn try_from(value: String) -> Result<Self> {
        match value.to_lowercase().as_ref() {
            "p" => Ok(PieceType::Pawn),
            "b" => Ok(PieceType::Bishop),
            "k" => Ok(PieceType::King),
            "n" => Ok(PieceType::Knight),
            "r" => Ok(PieceType::Rook),
            "q" => Ok(PieceType::Queen),
            _ => Err(PieceError::InvalidNotation),
        }
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

    pub fn from_notation(notation: char) -> Result<Self> {
        let notation_str = notation.to_string();
        let piece_type = PieceType::try_from(notation_str.clone())?;
        let piece_color = {
            if notation_str == notation_str.to_lowercase() {
                Color::Black
            } else {
                Color::White
            }
        };

        Ok(Piece::new(piece_type, piece_color))
    }
}
