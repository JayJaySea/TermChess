use crate::movement::{Square, Move};
use super::{Piece, PieceColor};
use crate::board::Board;

pub struct Pawn {
    color: PieceColor,
    moved: bool
}

impl Pawn {
    pub fn new(color: PieceColor) -> Pawn {
        Pawn {
            color, 
            moved: false
        }
    }
}

impl Piece for Pawn {
    fn can_move_to(&self, board: &Board, m: Move) -> (bool, bool) {
        let dest_occupied = board.get_piece(m.end()).is_some();

        let ((sx, sy), (ex, ey)) = m.to_coords();

        let (distance, forward) = match self.color {
            PieceColor::WHITE => {
                if ey > sy { 
                    (ey - sy, sy + 1)
                } else {
                    return (false, false);
                } 
            },
            PieceColor::BLACK => {
                if ey < sy {
                    (sy - ey, sy - 1)
                } else {
                    return (false, false);
                }
            }
        };


        (
            if sx == ex && !dest_occupied {
                match distance {
                    1 => true,
                    2 => !self.moved && board.get_piece(Square::new(ex, forward)).is_none(),
                    _ => false
                }
            } else if dest_occupied && distance == 1 {
                if sx > ex {
                    sx - ex == 1 
                } else if sx < ex {
                    ex - sx == 1 
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

    #[test]
    fn basic_white_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(PieceColor::WHITE))));
    
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

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(PieceColor::BLACK))));
    
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

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(3, 5), Some(Box::new(Pawn::new(PieceColor::BLACK))));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 5))), false);

        board.set(Square::new(3, 3), None);
        board.set(Square::new(3, 4), Some(Box::new(Pawn::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 5))), false);
    }

    #[test]
    fn blocked_basic_black_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(3, 1), Some(Box::new(Pawn::new(PieceColor::WHITE))));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 1))), false);

        board.set(Square::new(3, 3), None);
        board.set(Square::new(3, 2), Some(Box::new(Pawn::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 1))), false);
    }

    #[test]
    fn white_pawn_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(4, 5), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(2, 5), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(4, 4), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(2, 4), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(4, 3), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(2, 3), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(5, 4), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(1, 4), Some(Box::new(Pawn::new(PieceColor::BLACK))));

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

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(4, 1), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(2, 1), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(4, 2), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(2, 2), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(4, 3), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(2, 3), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(4, 3), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(2, 3), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(5, 2), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(1, 2), Some(Box::new(Pawn::new(PieceColor::WHITE))));

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

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(4, 4), Some(Box::new(Pawn::new(PieceColor::WHITE))));
        board.set(Square::new(2, 4), Some(Box::new(Pawn::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
    }

    #[test]
    fn black_pawn_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(4, 2), Some(Box::new(Pawn::new(PieceColor::BLACK))));
        board.set(Square::new(2, 2), Some(Box::new(Pawn::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
    }
}
