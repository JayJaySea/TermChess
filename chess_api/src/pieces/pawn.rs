use crate::Square;
use super::{Piece, PieceColor};
use crate::board::Board;

pub struct Pawn {
    pos: Square,
    color: PieceColor
}

impl Pawn {
    pub fn new(pos: Square, color: PieceColor) -> Pawn {
        Pawn {
            pos, color
        }
    }
}

impl Piece for Pawn {
    fn can_move_to(&self, board: &Board, to: Square) -> bool {
        false 
    }

    fn get_character(&self) -> char {
        match self.color {
            PieceColor::BLACK => 'p',
            PieceColor::WHITE => 'P',
        }
    }
}
