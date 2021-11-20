use super::{Piece, PieceColor};
use crate::movement::Move;
use crate::board::Board;

pub struct King {
    color: PieceColor
}

impl King {
    pub fn new(color: PieceColor) -> King {
        King {
            color
        }
    }
}

impl Piece for King {
    fn can_move_to(&self, _board: &Board, m: Move) -> (bool, bool) {
        let ((sx, sy), (ex, ey)) = m.to_coords();

        let dx: u8 = (sx as i8 - ex as i8).abs() as u8;
        let dy: u8 = (sy as i8 - ey as i8).abs() as u8;
        
        ((dx != 0 || dy != 0) && (dx <= 1 && dy <= 1), false)
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
