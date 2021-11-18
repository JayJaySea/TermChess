use crate::Square;
use super::{Piece, PieceColor};
use crate::board::Board;

pub struct Bishop {
    pos: Square,
    color: PieceColor
}

impl Bishop {
    pub fn new(pos: Square, color: PieceColor) -> Bishop {
        Bishop {
            pos, color
        }
    }
}

impl Piece for Bishop {
    fn can_move_to(&self, board: &Board, to: Square) -> bool {
        false 
    }

    fn get_character(&self) -> char {
        match self.color {
            PieceColor::BLACK => 'b',
            PieceColor::WHITE => 'B',
        }
    }
}
