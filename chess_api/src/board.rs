use super::pieces::*;
use super::movement::*;

/*
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
*/


pub struct Board {
    pieces: [Option<Box<dyn Piece>>; 64]
}

impl Board {
    // Constructors
    /// # Creates new clear board
    ///
    /// all of squares on resulting board are None
    ///
    /// ```
    /// use chess_api::board::Board;
    ///
    /// let board = Board::new_clear();
    ///
    /// for (_, piece) in board.squares() {
    ///     assert!(piece.is_none());
    /// }
    /// ```
    pub fn new_clear() -> Board {
        const INIT: Option<Box<dyn Piece>> = None;

        Board {
            pieces: [INIT; 64]
        }
    
    }

    /// # Creates new board with standard starting position
    ///
    /// ```
    /// use chess_api::pieces::PieceColor;
    /// use chess_api::board::Board;
    ///
    /// let board = Board::new();
    ///
    /// assert_eq!(board.squares().count(), 64);
    /// assert_eq!(board.pieces(None).count(), 32);
    /// assert_eq!(board.pieces(Some(PieceColor::WHITE)).count(), 16);
    /// assert_eq!(board.pieces(Some(PieceColor::BLACK)).count(), 16);
    /// ```
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

    // basic board state getter functions
    /// # Returns piece at given square
    pub fn get_piece(&self, square: Square) -> Option<&Box<dyn Piece>> {
        self.pieces[square.to_index()].as_ref()
    }

    /// # Returns piece at given index
    pub fn get_piece_at_index(&self, index: usize) -> Option<&Box<dyn Piece>> {
        assert!(index < 64);
        self.pieces[index].as_ref()
    }

    /// # Returns piece at given square after simulating move
    pub fn get_piece_after_move(&self, square: Square, m: Move) -> Option<&Box<dyn Piece>> {
        if square == m.start() {
            None
        } else if square == m.end() {
            self.get_piece(m.start())
        } else {
            self.get_piece(m.end())
        }
    }

    // advanced board state getters
    /// # Returns true if given square is attacked by given player
    pub fn is_square_attacked(_color: PieceColor) -> bool {
        todo!("Implement is_square_attacked"); 
    }

    /// # Returns true if given square is attacked by given player after simulating move
    pub fn is_square_attacked_after_move(_color: PieceColor, _m: Move) -> bool {
        todo!("Implement is_square_attacked_after_move"); 
    }

    // move possibility checks
    /// # Returns if specified move is possible
    ///
    /// takes flag that should only be manipulated by board's methods
    fn is_move_possible_skippable_checks(&self, m: Move, check_block: bool) -> bool {
        let (src, dst) = m.to_squares();

        let source_piece = self.get_piece(src);
        let destination_piece = self.get_piece(dst);

        if let Some(source_piece) = source_piece {
            if let Some(destination_piece) = destination_piece {
                if source_piece.color() == destination_piece.color() {
                    return false;
                }
            }

            let (can_move, validate_block) = source_piece.can_move_to(self, m);

            if validate_block && can_move && check_block {
                LineMovement::from(m).all(|pos| self.get_piece(pos).is_none())
            } else { can_move }
        } else {
            false
        }
    }

    /// # Public wrapper around private board's function
    ///
    /// ```
    /// use chess_api::movement::{Move, Square};
    /// use chess_api::board::Board;
    ///
    /// let board = Board::new();
    ///
    /// assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 3))), true);
    /// assert_eq!(board.is_move_possible(Move::new(Square::new(1, 0), Square::new(1, 3))), false);
    /// ```
    pub fn is_move_possible(&self, m: Move) -> bool {
        self.is_move_possible_skippable_checks(m, true) 
    }

    // basic board state changers
    /// # Sets piece at square
    ///
    /// should only be used for setting up custom positions
    /// not for moving pieces during game
    ///
    pub fn set(&mut self, square: Square, piece: Option<Box<dyn Piece>>) {
        self.pieces[square.to_index()] = piece; 
    }

    // advanced board state changers
    /// # Performs move after checking if it is possible
    ///
    /// ```
    /// use chess_api::movement::{Move, Square};
    /// use chess_api::board::Board;
    ///
    /// let mut board = Board::new();
    ///
    /// board.perform_move(Move::new(Square::new(1, 1), Square::new(1, 3)));
    ///
    /// assert_eq!(board.get_piece(Square::new(1, 3)).unwrap().get_character(), 'P');
    /// assert!(board.get_piece(Square::new(1, 1)).is_none());
    /// ```
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

    // iterators
    /// # Returns iterator over every square on the board
    pub fn squares(&self) -> impl Iterator<Item = (Square, Option<&Box<dyn Piece>>)>{
        (0..64).map(|index| Square::from_index(index)).map(|square| (square, self.get_piece(square)))
    }

    /// # Returns iterator over every piece on the board
    ///
    /// ```
    /// use chess_api::board::Board;
    /// 
    /// let board = Board::new();
    ///
    /// for (square, piece) in board.pieces(None) {
    ///     println!("{:?} => {}", square, piece.get_character());
    /// }
    /// ```
    pub fn pieces(&self, color: Option<PieceColor>) -> impl Iterator<Item = (Square, &Box<dyn Piece>)> {
        self.squares().filter_map(|square| match square.1 {
            Some(piece) => Some((square.0, piece)),
            None => None
        }).filter(move |piece| match color {
            Some(color) => piece.1.color() == color,
            None => true
        })
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

    #[test]
    fn piece_iterator() {
        let board = Board::new();

        assert_eq!(board.squares().count(), 64);
        assert_eq!(board.pieces(None).count(), 32);
        assert_eq!(board.pieces(Some(PieceColor::WHITE)).count(), 16);
        assert_eq!(board.pieces(Some(PieceColor::BLACK)).count(), 16);
    }
}
