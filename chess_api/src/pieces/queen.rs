use crate::Square;
use super::{Piece, PieceColor};
use crate::board::Board;

pub struct Queen {
    pos: Square,
    color: PieceColor
}

impl Queen {
    pub fn new(pos: Square, color: PieceColor) -> Queen {
        Queen {
            pos, color
        }
    }
}

impl Piece for Queen {
    fn can_move_to(&self, board: &Board, to: Square) -> bool {
        false 
    }

    fn get_character(&self) -> char {
        match self.color {
            PieceColor::BLACK => 'q',
            PieceColor::WHITE => 'Q',
        }
    }
}
