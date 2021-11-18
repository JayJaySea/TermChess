use crate::Square;
use super::{Piece, PieceColor};
use crate::board::Board;

pub struct King {
    pos: Square,
    color: PieceColor
}

impl King {
    pub fn new(pos: Square, color: PieceColor) -> King {
        King {
            pos, color
        }
    }
}

impl Piece for King {
    fn can_move_to(&self, board: &Board, to: Square) -> bool {
        false 
    }

    fn get_character(&self) -> char {
        match self.color {
            PieceColor::BLACK => 'k',
            PieceColor::WHITE => 'K',
        }
    }

    fn color(&self) -> PieceColor {
        self.color
    }
}
