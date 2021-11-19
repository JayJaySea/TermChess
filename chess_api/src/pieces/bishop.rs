use crate::Square;
use super::{Piece, PieceColor};
use crate::board::Board;
use crate::min_max_rev;

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
        let (min_x, max_x, rev_x) = min_max_rev(self.pos.x, to.x);
        let (min_y, max_y, rev_y) = min_max_rev(self.pos.y, to.y);
      
        if min_x != max_x && max_x - min_x == max_y - min_y {
            if rev_x == rev_y {
                ((min_x + 1)..max_x)
                    .zip((min_y + 1)..max_y)
                    .map(|(x, y)| Square::new(x, y))
                    .all(|pos| board.get_piece(pos).is_none())
            } else {
                ((min_x + 1)..max_x).rev()
                    .zip((min_y + 1)..max_y)
                    .map(|(x, y)| Square::new(x, y))
                    .all(|pos| board.get_piece(pos).is_none())
            }
        } else { false }
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
