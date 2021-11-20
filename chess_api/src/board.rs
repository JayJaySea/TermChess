use super::pieces::*;
use super::movement::*;

pub enum PostMoveStatus {
    NORMAL,
    CHECKMATE,
    STELEMATE,
    CHECK
}

pub enum MoveFailReason {
    ILLEGAL_MOVE,
    CHECK
}

pub struct Board {
    pieces: [Option<Box<dyn Piece>>; 64]
}

impl Board {
    pub fn new_clear() -> Board {
        const INIT: Option<Box<dyn Piece>> = None;

        Board {
            pieces: [INIT; 64]
        }
    
    }

    pub fn new() -> Board {
        let mut board = Board::new_clear();

        board.pieces[Square::new(0, 0).to_index()] = Some(Box::new(Rook::new(PieceColor::WHITE)));
        board.pieces[Square::new(1, 0).to_index()] = Some(Box::new(Knight::new(PieceColor::WHITE)));
        board.pieces[Square::new(2, 0).to_index()] = Some(Box::new(Bishop::new(PieceColor::WHITE)));
        board.pieces[Square::new(3, 0).to_index()] = Some(Box::new(Queen::new(PieceColor::WHITE)));
        board.pieces[Square::new(4, 0).to_index()] = Some(Box::new(King::new(PieceColor::WHITE)));
        board.pieces[Square::new(5, 0).to_index()] = Some(Box::new(Bishop::new(PieceColor::WHITE)));
        board.pieces[Square::new(6, 0).to_index()] = Some(Box::new(Knight::new(PieceColor::WHITE)));
        board.pieces[Square::new(7, 0).to_index()] = Some(Box::new(Rook::new(PieceColor::WHITE)));
        
        board.pieces[Square::new(0, 7).to_index()] = Some(Box::new(Rook::new(PieceColor::BLACK)));
        board.pieces[Square::new(1, 7).to_index()] = Some(Box::new(Knight::new(PieceColor::BLACK)));
        board.pieces[Square::new(2, 7).to_index()] = Some(Box::new(Bishop::new(PieceColor::BLACK)));
        board.pieces[Square::new(3, 7).to_index()] = Some(Box::new(Queen::new(PieceColor::BLACK)));
        board.pieces[Square::new(4, 7).to_index()] = Some(Box::new(King::new(PieceColor::BLACK)));
        board.pieces[Square::new(5, 7).to_index()] = Some(Box::new(Bishop::new(PieceColor::BLACK)));
        board.pieces[Square::new(6, 7).to_index()] = Some(Box::new(Knight::new(PieceColor::BLACK)));
        board.pieces[Square::new(7, 7).to_index()] = Some(Box::new(Rook::new(PieceColor::BLACK)));

        for i in 0..8 {
            board.pieces[Square::new(i, 1).to_index()] = Some(Box::new(Pawn::new(PieceColor::WHITE)));
            board.pieces[Square::new(i, 6).to_index()] = Some(Box::new(Pawn::new(PieceColor::BLACK)));
        }

        board
    }

    pub fn get_piece(&self, square: Square) -> &Option<Box<dyn Piece>> {
        &self.pieces[square.to_index()]
    }

    pub fn is_move_possible(&self, m: Move) -> bool {
        let (src, dst) = m.to_squares();

        let source_piece = self.get_piece(src);
        let destination_piece = self.get_piece(dst);

        if let Some(source_piece) = source_piece {
            if let Some(destination_piece) = destination_piece {
                if source_piece.color() == destination_piece.color() {
                    return false;
                }
            }

            let (can_move, validate_empty) = source_piece.can_move_to(self, m);

            if validate_empty && can_move {
                LineMovement::from(m).all(|pos| self.get_piece(pos).is_none())
            } else { can_move }
        } else {
            false
        }
    }

    pub fn perform_move(&mut self, m: Move) -> bool { // todo more sophisticated movement error indicator
        if self.is_move_possible(m) {
            let src = m.start().to_index();
            let dst = m.end().to_index();

            self.pieces[dst] = self.pieces[src].take();

            true
        } else {
            false
        }
    }

    /// # Sets piece at square
    ///
    /// should only be used for setting up custom positions
    /// not for moving pieces during game
    ///
    pub fn set(&mut self, square: Square, piece: Option<Box<dyn Piece>>) {
        self.pieces[square.to_index()] = piece; 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_board() {
        let b = Board::new();

        assert_eq!(b.get_piece(Square::new(0, 0)).as_ref().unwrap().get_character(), 'R');
        assert_eq!(b.get_piece(Square::new(7, 7)).as_ref().unwrap().get_character(), 'r');

        assert_eq!(b.get_piece(Square::new(1, 0)).as_ref().unwrap().get_character(), 'N');
        assert_eq!(b.get_piece(Square::new(6, 7)).as_ref().unwrap().get_character(), 'n');

        assert_eq!(b.get_piece(Square::new(2, 0)).as_ref().unwrap().get_character(), 'B');
        assert_eq!(b.get_piece(Square::new(5, 7)).as_ref().unwrap().get_character(), 'b');

        assert_eq!(b.get_piece(Square::new(3, 0)).as_ref().unwrap().get_character(), 'Q');
        assert_eq!(b.get_piece(Square::new(3, 7)).as_ref().unwrap().get_character(), 'q');

        assert_eq!(b.get_piece(Square::new(4, 0)).as_ref().unwrap().get_character(), 'K');
        assert_eq!(b.get_piece(Square::new(4, 7)).as_ref().unwrap().get_character(), 'k');

        assert!(b.get_piece(Square::new(4, 4)).is_none());
    }

    #[test]
    fn basic_board_movement() {
        let mut b = Board::new();

        // 1.c4 e5 2.Nc3 Nc6 3.g3 g6
        // c4 e5
        assert_eq!(b.perform_move(Move::new(Square::new(2, 1), Square::new(2, 3))), true);
        assert_eq!(b.get_piece(Square::new(2, 3)).as_ref().unwrap().get_character(), 'P');
        //assert_eq!(b.get_piece(Square::new(2, 1)), None);
   
        assert_eq!(b.perform_move(Move::new(Square::new(4, 6), Square::new(4, 4))), true);
        assert_eq!(b.get_piece(Square::new(4, 4)).as_ref().unwrap().get_character(), 'p');
        //assert_eq!(b.get_piece(Square::new(4, 6)), None);
        
        // Nc3 Nc6
        assert_eq!(b.perform_move(Move::new(Square::new(1, 0), Square::new(2, 2))), true);
        assert_eq!(b.get_piece(Square::new(2, 2)).as_ref().unwrap().get_character(), 'N');
        //assert_eq!(b.get_piece(Square::new(1, 0)), None);
   
        assert_eq!(b.perform_move(Move::new(Square::new(1, 7), Square::new(2, 5))), true);
        assert_eq!(b.get_piece(Square::new(2, 5)).as_ref().unwrap().get_character(), 'n');
        //assert_eq!(b.get_piece(Square::new(1, 7)), None);

        // g3 g6
        assert_eq!(b.perform_move(Move::new(Square::new(6, 1), Square::new(6, 2))), true);
        assert_eq!(b.get_piece(Square::new(6, 2)).as_ref().unwrap().get_character(), 'P');
        //assert_eq!(b.get_piece(Square::new(6, 1)), None);
   
        assert_eq!(b.perform_move(Move::new(Square::new(6, 6), Square::new(6, 5))), true);
        assert_eq!(b.get_piece(Square::new(2, 5)).as_ref().unwrap().get_character(), 'n');
        //assert_eq!(b.get_piece(Square::new(1, 7)), None);
    }
}
