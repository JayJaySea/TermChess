use crate::Square;
use super::{Piece, PieceColor};
use crate::board::Board;

pub struct Rook {
    pos: Square,
    color: PieceColor
}

impl Rook {
    pub fn new(pos: Square, color: PieceColor) -> Rook {
        Rook {
            pos, color
        }
    }
}

impl Piece for Rook {
    fn can_move_to(&self, board: &Board, to: Square) -> bool {
        false 
    }

    fn get_character(&self) -> char {
        match self.color {
            PieceColor::BLACK => 'r',
            PieceColor::WHITE => 'R',
        }
    }
}
