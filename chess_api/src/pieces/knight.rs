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
    fn can_move_to(&self, _b: &Board, to: Square) -> bool {
        let all_moves: Vec<(i8, i8)> = vec![
            ( 1,  2), ( 2,  1),
            (-1, -2), (-2, -1),
            ( 1, -2), ( 2, -1),
            (-1,  2), (-2,  1),
        ];

        for (dx, dy) in all_moves {
            if self.pos.x as i8 + dx == to.x as i8 && 
                self.pos.y as i8 + dy == to.y as i8 {
                return true;
            }
        }

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::Move;

    #[test]
    fn basic_knight_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Knight::new(Square::new(3, 3), PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 1))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(1, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 1))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(1, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 5))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
    }

    #[test]
    fn knight_movement_near_positive_board_edges() {
        let mut board = Board::new_clear();

        board.set(Square::new(6, 6), Some(Box::new(Knight::new(Square::new(6, 6), PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(6, 6), Square::new(4, 7))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(6, 6), Square::new(4, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(6, 6), Square::new(5, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(6, 6), Square::new(7, 4))), true);
    }

    #[test]
    fn knight_movement_near_negative_board_edges() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Box::new(Knight::new(Square::new(1, 1), PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(0, 3))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(2, 3))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(3, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(3, 2))), true);
    }
}
