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
        let dx: u8 = (self.pos.x as i8 - to.x as i8).abs() as u8;
        let dy: u8 = (self.pos.y as i8 - to.y as i8).abs() as u8;

        if dx == 0 { return false; }
        if dx != dy { return false; }

        let dx: i8 = if self.pos.x < to.x { 1 } else { -1 };
        let dy: i8 = if self.pos.y < to.y { 1 } else { -1 };

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
            PieceColor::BLACK => 'b',
            PieceColor::WHITE => 'B',
        }
    }

    fn color(&self) -> PieceColor {
        self.color
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Move;

    #[test]
    fn basic_bishop_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(Square::new(3, 3), PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 6))), false);
    }

    #[test]
    fn bishop_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(Square::new(3, 3), PieceColor::WHITE))));
        
        board.set(Square::new(2, 4), Some(Box::new(Bishop::new(Square::new(2, 4), PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Bishop::new(Square::new(2, 2), PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Bishop::new(Square::new(4, 2), PieceColor::BLACK))));
        board.set(Square::new(4, 4), Some(Box::new(Bishop::new(Square::new(4, 4), PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(Square::new(3, 3), PieceColor::WHITE))));
        
        board.set(Square::new(0, 0), Some(Box::new(Bishop::new(Square::new(0, 0), PieceColor::BLACK))));
        board.set(Square::new(0, 6), Some(Box::new(Bishop::new(Square::new(0, 6), PieceColor::BLACK))));
        board.set(Square::new(6, 0), Some(Box::new(Bishop::new(Square::new(6, 0), PieceColor::BLACK))));
        board.set(Square::new(7, 7), Some(Box::new(Bishop::new(Square::new(7, 7), PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), true);
    }

    #[test]
    fn bishop_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(Square::new(3, 3), PieceColor::WHITE))));
        
        board.set(Square::new(2, 4), Some(Box::new(Bishop::new(Square::new(2, 4), PieceColor::WHITE))));
        board.set(Square::new(2, 2), Some(Box::new(Bishop::new(Square::new(2, 2), PieceColor::WHITE))));
        board.set(Square::new(4, 2), Some(Box::new(Bishop::new(Square::new(4, 2), PieceColor::WHITE))));
        board.set(Square::new(4, 4), Some(Box::new(Bishop::new(Square::new(4, 4), PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(Square::new(3, 3), PieceColor::WHITE))));
        
        board.set(Square::new(0, 0), Some(Box::new(Bishop::new(Square::new(0, 0), PieceColor::WHITE))));
        board.set(Square::new(0, 6), Some(Box::new(Bishop::new(Square::new(0, 6), PieceColor::WHITE))));
        board.set(Square::new(6, 0), Some(Box::new(Bishop::new(Square::new(6, 0), PieceColor::WHITE))));
        board.set(Square::new(7, 7), Some(Box::new(Bishop::new(Square::new(7, 7), PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }

    #[test]
    fn bishop_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(Square::new(3, 3), PieceColor::WHITE))));
        
        board.set(Square::new(2, 4), Some(Box::new(Bishop::new(Square::new(2, 4), PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Bishop::new(Square::new(2, 2), PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Bishop::new(Square::new(4, 2), PieceColor::BLACK))));
        board.set(Square::new(4, 4), Some(Box::new(Bishop::new(Square::new(4, 4), PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }
}
