use crate::Color;

pub trait ToFen {
    fn to_fen(&self, color: Color) -> String;
}
