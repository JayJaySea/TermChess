use super::{Piece, PieceColor};
use crate::movement::Move;
use crate::board::Board;

pub struct Bishop {
    color: PieceColor
}

impl Bishop {
    pub fn new(color: PieceColor) -> Bishop {
        Bishop {
            color
        }
    }
}

impl Piece for Bishop {
    fn can_move_to(&self, _b: &Board, m: Move) -> (bool, bool) {    
        let (dx, dy) = m.to_deltas();
        ( dx == dy, true )
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
    use crate::movement::Square;

    #[test]
    fn basic_bishop_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(PieceColor::WHITE))));

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

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        
        board.set(Square::new(2, 4), Some(Box::new(Bishop::new(PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Bishop::new(PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Bishop::new(PieceColor::BLACK))));
        board.set(Square::new(4, 4), Some(Box::new(Bishop::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        
        board.set(Square::new(0, 0), Some(Box::new(Bishop::new(PieceColor::BLACK))));
        board.set(Square::new(0, 6), Some(Box::new(Bishop::new(PieceColor::BLACK))));
        board.set(Square::new(6, 0), Some(Box::new(Bishop::new(PieceColor::BLACK))));
        board.set(Square::new(7, 7), Some(Box::new(Bishop::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), true);
    }

    #[test]
    fn bishop_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        
        board.set(Square::new(2, 4), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        board.set(Square::new(2, 2), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        board.set(Square::new(4, 2), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        board.set(Square::new(4, 4), Some(Box::new(Bishop::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        
        board.set(Square::new(0, 0), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        board.set(Square::new(0, 6), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        board.set(Square::new(6, 0), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        board.set(Square::new(7, 7), Some(Box::new(Bishop::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }

    #[test]
    fn bishop_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Bishop::new(PieceColor::WHITE))));
        
        board.set(Square::new(2, 4), Some(Box::new(Bishop::new(PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Bishop::new(PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Bishop::new(PieceColor::BLACK))));
        board.set(Square::new(4, 4), Some(Box::new(Bishop::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }
}
