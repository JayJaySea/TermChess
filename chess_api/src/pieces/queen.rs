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
        let dx: u8 = (self.pos.x as i8 - to.x as i8).abs() as u8;
        let dy: u8 = (self.pos.y as i8 - to.y as i8).abs() as u8;
    
        let (dx, dy): (i8, i8) = if dx == 0 || dy == 0 {
            if dx == 0 { if to.y > self.pos.y { (0, 1) } else { (0, -1) }} 
            else { if to.x > self.pos.x { (1, 0) } else { (-1, 0) }}
        } else if dx == dy {
            ( if self.pos.x < to.x { 1 } else { -1 }, if self.pos.y < to.y { 1 } else { -1 })
        } else {
            return false;
        };

        let mut x = self.pos.x as i8 + dx;
        let mut y = self.pos.y as i8 + dy;

        while x as u8 != to.x || y as u8 != to.y {
            if board.get_piece(Square::new(x as u8, y as u8)).is_some() {
                return false;
            }
            x += dx;
            y += dy;
        }

        true
    }

    fn get_character(&self) -> char {
        match self.color {
            PieceColor::BLACK => 'q',
            PieceColor::WHITE => 'Q',
        }
    }

    fn color(&self) -> PieceColor {
        self.color
    }
}
