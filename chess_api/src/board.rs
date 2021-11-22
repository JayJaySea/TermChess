use super::piece::*;
use super::movement::*;


pub struct Board {
    pieces: [Option<Piece>; 64]
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
        const INIT: Option<Piece> = None;

        Board {
            pieces: [INIT; 64]
        }
    
    }

    /// # Creates new board with standard starting position
    ///
    /// ```
    /// use chess_api::piece::PieceColor;
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

        board.pieces[Square::new(0, 0).to_index()] = Some(Piece::new(PieceType::Rook,   PieceColor::WHITE));
        board.pieces[Square::new(1, 0).to_index()] = Some(Piece::new(PieceType::Knight, PieceColor::WHITE));
        board.pieces[Square::new(2, 0).to_index()] = Some(Piece::new(PieceType::Bishop, PieceColor::WHITE));
        board.pieces[Square::new(3, 0).to_index()] = Some(Piece::new(PieceType::Queen,  PieceColor::WHITE));
        board.pieces[Square::new(4, 0).to_index()] = Some(Piece::new(PieceType::King,   PieceColor::WHITE));
        board.pieces[Square::new(5, 0).to_index()] = Some(Piece::new(PieceType::Bishop, PieceColor::WHITE));
        board.pieces[Square::new(6, 0).to_index()] = Some(Piece::new(PieceType::Knight, PieceColor::WHITE));
        board.pieces[Square::new(7, 0).to_index()] = Some(Piece::new(PieceType::Rook,   PieceColor::WHITE));

        board.pieces[Square::new(0, 7).to_index()] = Some(Piece::new(PieceType::Rook,   PieceColor::BLACK));
        board.pieces[Square::new(1, 7).to_index()] = Some(Piece::new(PieceType::Knight, PieceColor::BLACK));
        board.pieces[Square::new(2, 7).to_index()] = Some(Piece::new(PieceType::Bishop, PieceColor::BLACK));
        board.pieces[Square::new(3, 7).to_index()] = Some(Piece::new(PieceType::Queen,  PieceColor::BLACK));
        board.pieces[Square::new(4, 7).to_index()] = Some(Piece::new(PieceType::King,   PieceColor::BLACK));
        board.pieces[Square::new(5, 7).to_index()] = Some(Piece::new(PieceType::Bishop, PieceColor::BLACK));
        board.pieces[Square::new(6, 7).to_index()] = Some(Piece::new(PieceType::Knight, PieceColor::BLACK));
        board.pieces[Square::new(7, 7).to_index()] = Some(Piece::new(PieceType::Rook,   PieceColor::BLACK));

        for i in 0..8 {
            board.pieces[Square::new(i, 1).to_index()] = Some(Piece::new(PieceType::Pawn, PieceColor::WHITE));
            board.pieces[Square::new(i, 6).to_index()] = Some(Piece::new(PieceType::Pawn, PieceColor::BLACK));
        }

        board
    }

    // basic board state getter functions
    /// # Returns piece at given index
    fn get_piece_at_index(&self, index: usize) -> Option<&Piece> {
        assert!(index < 64);
        self.pieces[index].as_ref()
    }

    /// # Returns piece at given square
    pub fn get_piece(&self, square: Square) -> Option<&Piece> {
        self.get_piece_at_index(square.to_index())
    }

    // move simulating board state getter functions
    /// # Returns piece at given index after simulating move
    fn get_piece_at_index_after_move(&self, index: usize, sm: Option<Move>) -> Option<&Piece> {
        match sm {
            Some(sm) => {
                if index == sm.start().to_index() {
                    None
                } else if index == sm.end().to_index() {
                    self.get_piece_at_index(sm.start().to_index())    
                } else {
                    self.get_piece_at_index(index)
                }
            },
            None => self.get_piece_at_index(index)
        }
    }

    /// # Returns piece at given square after simulating move
    fn get_piece_after_move(&self, square: Square, sm: Option<Move>) -> Option<&Piece> {
        self.get_piece_at_index_after_move(square.to_index(), sm)
    }




    // move possibility checks
    fn is_move_possible_after_move(&self, m: Move, sm: Option<Move>) -> bool {
        let (src, dst) = m.to_squares();

        let sorce_piece = self.get_piece_after_move(src, sm);
        let destination_piece = self.get_piece_after_move(dst, sm);

        if let Some(source_piece) = sorce_piece {
            if let Some(destination_piece) = destination_piece {
                if source_piece.color() == destination_piece.color() {
                    return false;
                }
            }

            let (can_move, validate_block) = source_piece.can_move_to(self, m);

            if validate_block && can_move {
                LineMovement::from(m).all(|pos| self.get_piece_after_move(pos, sm).is_none())
            } else { can_move }
        } else {
            false
        }
    }

    /// # 
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
        self.is_move_possible_after_move(m, None) 
    }




    // basic board state changers
    /// # Sets piece at square
    ///
    /// should only be used for setting up custom positions
    /// not for moving pieces during game
    ///
    pub fn set(&mut self, square: Square, piece: Option<Piece>) {
        self.pieces[square.to_index()] = piece; 
    }

    // advanced board state changers
    /// # Performs move after checking if it is possible
    ///
    /// ```
    /// use chess_api::movement::{Move, Square};
    /// use chess_api::piece::{PieceType, PieceColor};
    /// use chess_api::board::Board;
    ///
    /// let mut board = Board::new();
    ///
    /// board.perform_move(Move::new(Square::new(1, 1), Square::new(1, 3)));
    ///
    /// assert_eq!(board.get_piece(Square::new(1, 3)).unwrap().piece_type(), PieceType::Pawn);
    /// assert_eq!(board.get_piece(Square::new(1, 3)).unwrap().color(), PieceColor::WHITE);
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
    fn squares_after_move(&self, sm: Option<Move>) -> impl Iterator<Item = (Square, Option<&Piece>)> {
        (0..64).map(|index| Square::from_index(index)).map(move |square| (square, self.get_piece_after_move(square, sm)))
    }

    /// # Returns iterator over every square on the board
    pub fn squares(&self) -> impl Iterator<Item = (Square, Option<&Piece>)> {
        self.squares_after_move(None)
    }

    /// # Returns iterator over every piece on the board
    ///
    /// ```
    /// use chess_api::board::Board;
    /// 
    /// let board = Board::new();
    ///
    /// for (square, piece) in board.pieces(None) {
    ///     println!("{:?} => {:?}", square, piece);
    /// }
    /// ```
    fn pieces_after_move(&self, color: Option<PieceColor>, sm: Option<Move>) -> impl Iterator<Item = (Square, &Piece)> {
        self.squares_after_move(sm).filter_map(|square| match square.1 {
            Some(piece) => Some((square.0, piece)),
            None => None
        }).filter(move |piece| match color {
            Some(color) => piece.1.color() == color,
            None => true
        })
    }

    pub fn pieces(&self, color: Option<PieceColor>) -> impl Iterator<Item = (Square, &Piece)> {
        self.pieces_after_move(color, None)
    }

    // advanced board state getters
    /// # Returns true if given square is attacked by given player after simulating move
    fn is_square_attacked_after_move(&self, square: Square, color: PieceColor, sm: Option<Move>) -> bool {
        self.pieces_after_move(Some(color), sm).any(|(start, _)| self.is_move_possible_after_move(Move::new(start, square), sm))
    }

    /// # Returns true if given square is attacked by given player
    pub fn is_square_attacked(&self, square: Square, color: PieceColor) -> bool {
        self.is_square_attacked_after_move(square, color, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_board() {
        let b = Board::new();

        assert_eq!(b.get_piece(Square::new(0, 0)).as_ref().unwrap().piece_type(), PieceType::Rook);
        assert_eq!(b.get_piece(Square::new(0, 0)).as_ref().unwrap().color(), PieceColor::WHITE);
        assert_eq!(b.get_piece(Square::new(7, 7)).as_ref().unwrap().piece_type(), PieceType::Rook);
        assert_eq!(b.get_piece(Square::new(7, 7)).as_ref().unwrap().color(), PieceColor::BLACK);

        assert_eq!(b.get_piece(Square::new(1, 0)).as_ref().unwrap().piece_type(), PieceType::Knight);
        assert_eq!(b.get_piece(Square::new(1, 0)).as_ref().unwrap().color(), PieceColor::WHITE);
        assert_eq!(b.get_piece(Square::new(6, 7)).as_ref().unwrap().piece_type(), PieceType::Knight);
        assert_eq!(b.get_piece(Square::new(6, 7)).as_ref().unwrap().color(), PieceColor::BLACK);

        assert_eq!(b.get_piece(Square::new(2, 0)).as_ref().unwrap().piece_type(), PieceType::Bishop);
        assert_eq!(b.get_piece(Square::new(2, 0)).as_ref().unwrap().color(), PieceColor::WHITE);
        assert_eq!(b.get_piece(Square::new(5, 7)).as_ref().unwrap().piece_type(), PieceType::Bishop);
        assert_eq!(b.get_piece(Square::new(5, 7)).as_ref().unwrap().color(), PieceColor::BLACK);

        assert_eq!(b.get_piece(Square::new(3, 0)).as_ref().unwrap().piece_type(), PieceType::Queen);
        assert_eq!(b.get_piece(Square::new(3, 7)).as_ref().unwrap().piece_type(), PieceType::Queen);

        assert_eq!(b.get_piece(Square::new(4, 0)).as_ref().unwrap().piece_type(), PieceType::King);
        assert_eq!(b.get_piece(Square::new(4, 7)).as_ref().unwrap().piece_type(), PieceType::King);

        assert!(b.get_piece(Square::new(4, 4)).is_none());
    }

    #[test]
    fn basic_board_movement() {
        let mut b = Board::new();

        // 1.c4 e5 2.Nc3 Nc6 3.g3 g6
        // c4 e5
        assert_eq!(b.perform_move(Move::new(Square::new(2, 1), Square::new(2, 3))), true);
        //assert_eq!(b.get_piece(Square::new(2, 3)).as_ref().unwrap().get_character(), 'P');
        //assert_eq!(b.get_piece(Square::new(2, 1)), None);
   
        assert_eq!(b.perform_move(Move::new(Square::new(4, 6), Square::new(4, 4))), true);
        //assert_eq!(b.get_piece(Square::new(4, 4)).as_ref().unwrap().get_character(), 'p');
        //assert_eq!(b.get_piece(Square::new(4, 6)), None);
        
        // Nc3 Nc6
        assert_eq!(b.perform_move(Move::new(Square::new(1, 0), Square::new(2, 2))), true);
        //assert_eq!(b.get_piece(Square::new(2, 2)).as_ref().unwrap().get_character(), 'N');
        //assert_eq!(b.get_piece(Square::new(1, 0)), None);
   
        assert_eq!(b.perform_move(Move::new(Square::new(1, 7), Square::new(2, 5))), true);
        //assert_eq!(b.get_piece(Square::new(2, 5)).as_ref().unwrap().get_character(), 'n');
        //assert_eq!(b.get_piece(Square::new(1, 7)), None);

        // g3 g6
        assert_eq!(b.perform_move(Move::new(Square::new(6, 1), Square::new(6, 2))), true);
        //assert_eq!(b.get_piece(Square::new(6, 2)).as_ref().unwrap().get_character(), 'P');
        //assert_eq!(b.get_piece(Square::new(6, 1)), None);
   
        assert_eq!(b.perform_move(Move::new(Square::new(6, 6), Square::new(6, 5))), true);
        //assert_eq!(b.get_piece(Square::new(2, 5)).as_ref().unwrap().get_character(), 'n');
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
