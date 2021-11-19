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
    fn can_move_to(&self, _board: &Board, to: Square) -> bool {
        let dx: u8 = (self.pos.x as i8 - to.x as i8).abs() as u8;
        let dy: u8 = (self.pos.y as i8 - to.y as i8).abs() as u8;
        
        (dx != 0 || dy != 0) && (dx <= 1 && dy <= 1)
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
