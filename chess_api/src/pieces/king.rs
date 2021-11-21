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
    fn can_move_to(&self, _b: &Board, m: Move) -> (bool, bool) {
        let (dx, dy) = m.to_deltas();
        (dx <= 1 && dy <= 1, false)
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
