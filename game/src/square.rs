use std::convert::TryFrom;
use std::fmt;
use std::num::ParseIntError;

use snafu::{ensure, ResultExt, Snafu};

use crate::constants::BOARD_DIMENSION;

#[derive(Debug, Snafu)]
pub enum Error {
    ParseError,
    InvalidColumn,
    InvalidRow { source: ParseIntError },
    RowTooBig,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Column {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Column::A => "A",
            Column::B => "B",
            Column::C => "C",
            Column::D => "D",
            Column::E => "E",
            Column::F => "F",
            Column::G => "G",
            Column::H => "H",
        };

        write!(f, "{}", ch)
    }
}

impl TryFrom<String> for Column {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "A" => Ok(Column::A),
            "B" => Ok(Column::B),
            "C" => Ok(Column::C),
            "D" => Ok(Column::D),
            "E" => Ok(Column::E),
            "F" => Ok(Column::F),
            "G" => Ok(Column::G),
            "H" => Ok(Column::H),
            _ => Err(Error::InvalidColumn),
        }
    }
}

impl TryFrom<usize> for Column {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Column::A),
            1 => Ok(Column::B),
            2 => Ok(Column::C),
            3 => Ok(Column::D),
            4 => Ok(Column::E),
            5 => Ok(Column::F),
            6 => Ok(Column::G),
            7 => Ok(Column::H),
            _ => Err(Error::ParseError),
        }
    }
}

impl From<Column> for usize {
    fn from(col: Column) -> usize {
        match col {
            Column::A => 0,
            Column::B => 1,
            Column::C => 2,
            Column::D => 3,
            Column::E => 4,
            Column::F => 5,
            Column::G => 6,
            Column::H => 7,
        }
    }
}

pub type Row = usize;

pub struct Square {
    pub col: Column,
    pub row: Row,
}

impl Square {
    pub fn new(col: Column, row: Row) -> Square {
        assert!(row < BOARD_DIMENSION);
        Square { col, row }
    }

    pub fn from_notation<T: AsRef<str>>(notation: T) -> Result<Square, Error> {
        ensure!(notation.as_ref().len() == 2, ParseError);
        let col = {
            let ch = notation.as_ref().chars().nth(0).unwrap();
            Column::try_from(ch.to_string().to_uppercase())?
        };
        let row = {
            let ch = notation.as_ref().chars().nth(1).unwrap();
            ch.to_string().parse::<Row>().context(InvalidRow)?
        };

        ensure!(row > 0 && (row - 1) < BOARD_DIMENSION, RowTooBig);
        Ok(Square::new(col, row - 1))
    }

    pub fn next_col(mut self) -> Option<Square> {
        let idx: usize = self.col.into();
        match Column::try_from(idx + 1) {
            Ok(col) => {
                self.col = col;
                Some(self)
            }
            Err(_) => None,
        }
    }

    pub fn prev_col(mut self) -> Option<Square> {
        let idx: usize = self.col.into();
        if idx == 0 {
            return None;
        }

        match Column::try_from(idx - 1) {
            Ok(col) => {
                self.col = col;
                Some(self)
            }
            Err(_) => None,
        }
    }

    pub fn next_row(mut self) -> Option<Square> {
        if self.row < (BOARD_DIMENSION - 1) {
            self.row += 1;
            Some(self)
        } else {
            None
        }
    }

    pub fn prev_row(mut self) -> Option<Square> {
        if self.row > 0 {
            self.row -= 1;
            Some(self)
        } else {
            None
        }
    }
}
