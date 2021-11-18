use super::pieces::*;
use super::Square;

pub struct Board {
    pieces: [Option<Box<dyn Piece>>; 64]
}

impl Board {
    pub fn new() -> Board {
        const INIT: Option<Box<dyn Piece>> = None;

        let mut board = Board {
            pieces: [INIT; 64]
        };

        board.pieces[Square::new(0, 0).to_index()] = Some(Box::new(Rook::new(Square::new(0, 0), PieceColor::WHITE)));
        board.pieces[Square::new(1, 0).to_index()] = Some(Box::new(Knight::new(Square::new(1, 0), PieceColor::WHITE)));
        board.pieces[Square::new(2, 0).to_index()] = Some(Box::new(Bishop::new(Square::new(2, 0), PieceColor::WHITE)));
        board.pieces[Square::new(3, 0).to_index()] = Some(Box::new(Queen::new(Square::new(3, 0), PieceColor::WHITE)));
        board.pieces[Square::new(4, 0).to_index()] = Some(Box::new(King::new(Square::new(4, 0), PieceColor::WHITE)));
        board.pieces[Square::new(5, 0).to_index()] = Some(Box::new(Bishop::new(Square::new(5, 0), PieceColor::WHITE)));
        board.pieces[Square::new(6, 0).to_index()] = Some(Box::new(Knight::new(Square::new(6, 0), PieceColor::WHITE)));
        board.pieces[Square::new(7, 0).to_index()] = Some(Box::new(Rook::new(Square::new(7, 0), PieceColor::WHITE)));
        
        board.pieces[Square::new(0, 7).to_index()] = Some(Box::new(Rook::new(Square::new(0, 7), PieceColor::BLACK)));
        board.pieces[Square::new(1, 7).to_index()] = Some(Box::new(Knight::new(Square::new(1, 7), PieceColor::BLACK)));
        board.pieces[Square::new(2, 7).to_index()] = Some(Box::new(Bishop::new(Square::new(2, 7), PieceColor::BLACK)));
        board.pieces[Square::new(3, 7).to_index()] = Some(Box::new(Queen::new(Square::new(3, 7), PieceColor::BLACK)));
        board.pieces[Square::new(4, 7).to_index()] = Some(Box::new(King::new(Square::new(4, 7), PieceColor::BLACK)));
        board.pieces[Square::new(5, 7).to_index()] = Some(Box::new(Bishop::new(Square::new(5, 7), PieceColor::BLACK)));
        board.pieces[Square::new(6, 7).to_index()] = Some(Box::new(Knight::new(Square::new(6, 7), PieceColor::BLACK)));
        board.pieces[Square::new(7, 7).to_index()] = Some(Box::new(Rook::new(Square::new(7, 7), PieceColor::BLACK)));

        for i in 0..8 {
            board.pieces[Square::new(i, 1).to_index()] = Some(Box::new(Pawn::new(Square::new(i, 1), PieceColor::WHITE)));
            board.pieces[Square::new(i, 6).to_index()] = Some(Box::new(Pawn::new(Square::new(i, 6), PieceColor::BLACK)));
        }

        board
    }

    pub fn get_piece(&self, square: Square) -> &Option<Box<dyn Piece>> {
        &self.pieces[square.to_index()]
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
}
