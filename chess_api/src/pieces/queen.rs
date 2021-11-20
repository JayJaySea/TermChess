use super::{Piece, PieceColor};
use crate::movement::Move;
use crate::board::Board;

pub struct Queen {
    color: PieceColor
}

impl Queen {
    pub fn new(color: PieceColor) -> Queen {
        Queen {
            color
        }
    }
}

impl Piece for Queen {
    fn can_move_to(&self, _b: &Board, m: Move) -> (bool, bool) {
        let ((min_x, max_x), (min_y, max_y)) = m.min_max_x_y();
        ( min_x == max_x || min_y == max_y || max_y - min_y == max_x - min_x, true )
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::movement::Square;

    #[test]
    fn queen_rook_basic_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Box::new(Queen::new(PieceColor::WHITE))));
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 2))), false);

        board.set(Square::new(1, 1), None);

        board.set(Square::new(5, 5), Some(Box::new(Queen::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(5, 5), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(5, 5), Square::new(5, 1))), true);
        
    }

    #[test]
    fn queen_bishop_basic_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Queen::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 6))), false);
    }

    #[test]
    fn queen_rook_taking_oppnent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(1, 5), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(5, 1), Some(Box::new(Queen::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), true);
    }

    #[test]
    fn queen_bishop_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Queen::new(PieceColor::WHITE))));
        
        board.set(Square::new(2, 4), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(4, 4), Some(Box::new(Queen::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Queen::new(PieceColor::WHITE))));
        
        board.set(Square::new(0, 0), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(0, 6), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(6, 0), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(7, 7), Some(Box::new(Queen::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), true);
    }
   
    #[test]
    fn queen_rook_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(1, 5), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(5, 1), Some(Box::new(Queen::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), false);
    }

    #[test]
    fn queen_bishop_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Queen::new(PieceColor::WHITE))));
        
        board.set(Square::new(2, 4), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(2, 2), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(4, 2), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(4, 4), Some(Box::new(Queen::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Queen::new(PieceColor::WHITE))));
        
        board.set(Square::new(0, 0), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(0, 6), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(6, 0), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(7, 7), Some(Box::new(Queen::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }

    #[test]
    fn queen_rook_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(1, 5), Some(Box::new(Queen::new(PieceColor::WHITE))));
        board.set(Square::new(5, 1), Some(Box::new(Queen::new(PieceColor::BLACK))));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(6, 1))), false);
    }

    #[test]
    fn queen_bishop_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Queen::new(PieceColor::WHITE))));
        
        board.set(Square::new(2, 4), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Queen::new(PieceColor::BLACK))));
        board.set(Square::new(4, 4), Some(Box::new(Queen::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }

}
