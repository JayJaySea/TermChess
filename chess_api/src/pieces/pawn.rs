use crate::Square;
use super::{Piece, PieceColor};
use crate::board::Board;

pub struct Pawn {
    pos: Square,
    color: PieceColor,
    moved: bool
}

impl Pawn {
    pub fn new(pos: Square, color: PieceColor) -> Pawn {
        Pawn {
            pos, color, 
            moved: false
        }
    }
}

impl Piece for Pawn {
    fn can_move_to(&self, board: &Board, to: Square) -> (bool, bool) {
        let dest_occupied = board.get_piece(to).is_some();

        let (distance, forward) = match self.color {
            PieceColor::WHITE => {
                if to.y > self.pos.y { 
                    (to.y - self.pos.y, self.pos.y + 1)
                } else {
                    return (false, false);
                } 
            },
            PieceColor::BLACK => {
                if to.y < self.pos.y {
                    (self.pos.y - to.y, self.pos.y - 1)
                } else {
                    return (false, false);
                }
            }
        };


        (
            if self.pos.x == to.x && !dest_occupied {
                match distance {
                    1 => true,
                    2 => !self.moved && board.get_piece(Square::new(to.x, forward)).is_none(),
                    _ => false
                }
            } else if dest_occupied && distance == 1 {
                if self.pos.x > to.x {
                    self.pos.x - to.x == 1 
                } else if self.pos.x < to.x {
                    to.x - self.pos.x == 1 
                } else { false }
            } else { false }, // todo en passant 
            false
        )
    }

    fn get_character(&self) -> char {
        match self.color {
            PieceColor::BLACK => 'p',
            PieceColor::WHITE => 'P',
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
    fn basic_white_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(Square::new(3, 3), PieceColor::WHITE))));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 5))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), false);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 4))), false);
    }

    #[test]
    fn basic_black_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(Square::new(3, 3), PieceColor::BLACK))));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 1))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), false);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 4))), false);
    }

    #[test]
    fn blocked_basic_white_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(Square::new(3, 3), PieceColor::WHITE))));
        board.set(Square::new(3, 5), Some(Box::new(Pawn::new(Square::new(3, 5), PieceColor::BLACK))));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 5))), false);

        board.set(Square::new(3, 3), None);
        board.set(Square::new(3, 4), Some(Box::new(Pawn::new(Square::new(3, 4), PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 5))), false);
    }

    #[test]
    fn blocked_basic_black_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(Square::new(3, 3), PieceColor::BLACK))));
        board.set(Square::new(3, 1), Some(Box::new(Pawn::new(Square::new(3, 1), PieceColor::WHITE))));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 1))), false);

        board.set(Square::new(3, 3), None);
        board.set(Square::new(3, 2), Some(Box::new(Pawn::new(Square::new(3, 2), PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 1))), false);
    }

    #[test]
    fn white_pawn_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(Square::new(3, 3), PieceColor::WHITE))));
        board.set(Square::new(4, 5), Some(Box::new(Pawn::new(Square::new(4, 5), PieceColor::BLACK))));
        board.set(Square::new(2, 5), Some(Box::new(Pawn::new(Square::new(2, 5), PieceColor::BLACK))));
        board.set(Square::new(4, 4), Some(Box::new(Pawn::new(Square::new(4, 4), PieceColor::BLACK))));
        board.set(Square::new(2, 4), Some(Box::new(Pawn::new(Square::new(2, 4), PieceColor::BLACK))));
        board.set(Square::new(4, 3), Some(Box::new(Pawn::new(Square::new(4, 3), PieceColor::BLACK))));
        board.set(Square::new(2, 3), Some(Box::new(Pawn::new(Square::new(2, 3), PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Pawn::new(Square::new(4, 2), PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Pawn::new(Square::new(2, 2), PieceColor::BLACK))));
        board.set(Square::new(5, 4), Some(Box::new(Pawn::new(Square::new(5, 4), PieceColor::BLACK))));
        board.set(Square::new(1, 4), Some(Box::new(Pawn::new(Square::new(1, 4), PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(1, 4))), false);
    }

    #[test]
    fn black_pawn_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(Square::new(3, 3), PieceColor::BLACK))));
        board.set(Square::new(4, 1), Some(Box::new(Pawn::new(Square::new(4, 1), PieceColor::WHITE))));
        board.set(Square::new(2, 1), Some(Box::new(Pawn::new(Square::new(2, 1), PieceColor::WHITE))));
        board.set(Square::new(4, 2), Some(Box::new(Pawn::new(Square::new(4, 2), PieceColor::WHITE))));
        board.set(Square::new(2, 2), Some(Box::new(Pawn::new(Square::new(2, 2), PieceColor::WHITE))));
        board.set(Square::new(4, 3), Some(Box::new(Pawn::new(Square::new(4, 3), PieceColor::WHITE))));
        board.set(Square::new(2, 3), Some(Box::new(Pawn::new(Square::new(2, 3), PieceColor::WHITE))));
        board.set(Square::new(4, 3), Some(Box::new(Pawn::new(Square::new(4, 4), PieceColor::WHITE))));
        board.set(Square::new(2, 3), Some(Box::new(Pawn::new(Square::new(2, 4), PieceColor::WHITE))));
        board.set(Square::new(5, 2), Some(Box::new(Pawn::new(Square::new(5, 2), PieceColor::WHITE))));
        board.set(Square::new(1, 2), Some(Box::new(Pawn::new(Square::new(1, 2), PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 1))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 1))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(1, 2))), false);
    }

    #[test]
    fn white_pawn_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(Square::new(3, 3), PieceColor::WHITE))));
        board.set(Square::new(4, 4), Some(Box::new(Pawn::new(Square::new(4, 4), PieceColor::WHITE))));
        board.set(Square::new(2, 4), Some(Box::new(Pawn::new(Square::new(2, 4), PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
    }

    #[test]
    fn black_pawn_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(Square::new(3, 3), PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Pawn::new(Square::new(4, 2), PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Pawn::new(Square::new(2, 2), PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
    }
}
