use crate::Square;
use super::{Piece, PieceColor};
use crate::board::Board;

pub struct Knight {
    pos: Square,
    color: PieceColor
}

impl Knight {
    pub fn new(pos: Square, color: PieceColor) -> Knight {
        Knight {
            pos, color
        }
    }
}

impl Piece for Knight {
    fn can_move_to(&self, board: &Board, to: Square) -> bool {
        false 
    }

    fn get_character(&self) -> char {
        match self.color {
            PieceColor::BLACK => 'n',
            PieceColor::WHITE => 'N',
        }
    }

    fn color(&self) -> PieceColor {
        self.color
    }
}
