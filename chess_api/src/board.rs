use super::piece::*;
use super::movement::*;


#[derive(Debug, PartialEq)]
pub enum MoveFailReason {
    NoSourcePiece, FriendlyFire, IllegalMove, KingAttacked
}

#[derive(Debug, PartialEq)]
pub enum PostMoveState {
    Normal, Check, Checkmate, Stelemate
}

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
    /// # use chess_api::board::Board;
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
    /// # use chess_api::piece::PieceColor;
    /// # use chess_api::board::Board;
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
    fn check_move_possibility_after_move(&self, m: Move, sm: Option<Move>) -> Result<(), MoveFailReason> {
        let (src, dst) = m.to_squares();

        let source_piece = self.get_piece_after_move(src, sm);
        let destination_piece = self.get_piece_after_move(dst, sm);

        if let Some(source_piece) = source_piece {
            let dest_ocuppied = if let Some(destination_piece) = destination_piece {
                if source_piece.color() == destination_piece.color() {
                    return Err(MoveFailReason::FriendlyFire);
                }
                true
            } else { false };

            let (can_move, validate_block) = source_piece.can_move_to(m, dest_ocuppied);

            let move_possible = if validate_block && can_move {
                LineMovement::from(m).all(|pos| self.get_piece_after_move(pos, sm).is_none())
            } else { can_move };

            if move_possible {
                if sm.is_some() {
                    Ok(())
                } else if self.is_king_attacked_after_move(source_piece.color(), Some(m)) {
                    Err(MoveFailReason::KingAttacked)
                } else { Ok(()) }
            } else { Err(MoveFailReason::IllegalMove) }
        } else { Err(MoveFailReason::NoSourcePiece) }
    }

    /// # 
    ///
    /// ```
    /// # use chess_api::movement::{Move, Square};
    /// # use chess_api::board::Board;
    ///
    /// # let board = Board::new();
    ///
    /// assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 3))).is_ok(), true);
    /// assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 0), Square::new(1, 3))).is_ok(), false);
    /// ```
    pub fn check_move_possibility(&self, m: Move) -> Result<(), MoveFailReason> {
        self.check_move_possibility_after_move(m, None) 
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
    /// # use chess_api::movement::{Move, Square};
    /// # use chess_api::piece::{PieceType, PieceColor};
    /// # use chess_api::board::Board;
    ///
    /// let mut board = Board::new();
    ///
    /// board.perform_move(Move::new(Square::new(1, 1), Square::new(1, 3)));
    ///
    /// assert_eq!(board.get_piece(Square::new(1, 3)).unwrap().piece_type(), PieceType::Pawn);
    /// assert_eq!(board.get_piece(Square::new(1, 3)).unwrap().color(), PieceColor::WHITE);
    /// assert!(board.get_piece(Square::new(1, 1)).is_none());
    /// ```
    pub fn perform_move(&mut self, m: Move) -> Result<PostMoveState, MoveFailReason> {
        match self.check_move_possibility(m) {
            Ok(_) => {
                let src = m.start().to_index();
                let dst = m.end().to_index();

                self.pieces[dst] = self.pieces[src].take();
                self.pieces[dst].as_mut().unwrap().move_piece();

                let next_color = !self.pieces[dst].as_ref().unwrap().color();
                let king_attacked = self.is_king_attacked(next_color);
                let has_moves = self.all_possible_moves(Some(next_color)).next().is_some();

                if king_attacked && has_moves {
                    Ok(PostMoveState::Check)
                } else if king_attacked {
                    Ok(PostMoveState::Checkmate)
                } else if has_moves {
                    Ok(PostMoveState::Normal)
                } else {
                    Ok(PostMoveState::Stelemate)
                }
            },
            Err(e) => Err(e)
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
    /// # use chess_api::board::Board;
    /// 
    /// # let board = Board::new();
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
        self.pieces_after_move(Some(color), sm).filter(|(start, _)| *start != square).any(|(start, _)| self.check_move_possibility_after_move(Move::new(start, square), sm).is_ok())
    }

    /// # Returns true if given square is attacked by given player
    ///
    /// will return false if attacked only by allied piece
    ///
    /// ```
    /// # use chess_api::board::Board;
    /// # use chess_api::movement::Square;
    /// # use chess_api::piece::PieceColor;
    ///
    /// let mut board = Board::new();
    ///
    /// board.set(Square::new(3, 1), None); // d2 ( in front of white's queen )
    /// 
    /// assert_eq!(board.is_square_attacked(Square::new(3, 6), PieceColor::WHITE), true); // d7
    /// assert_eq!(board.is_square_attacked(Square::new(3, 6), PieceColor::BLACK), false);
    /// assert_eq!(board.is_square_attacked(Square::new(3, 1), PieceColor::WHITE), true);
    /// assert_eq!(board.is_square_attacked(Square::new(3, 1), PieceColor::BLACK), false);
    /// ```
    pub fn is_square_attacked(&self, square: Square, color: PieceColor) -> bool {
        self.is_square_attacked_after_move(square, color, None)
    }

    /// # If king is attacked returns true
    fn is_king_attacked_after_move(&self, color: PieceColor, sm: Option<Move>) -> bool {
        if let Some((square, _)) = self.pieces_after_move(Some(color), sm).find(|(_, piece)| piece.piece_type() == PieceType::King) {
            self.is_square_attacked_after_move(square, !color, sm)
        } else { false }
    }

    /// # If king is attacked returns true
    ///
    /// will return false if attacked only by allied piece
    ///
    /// ```
    /// # use chess_api::board::Board;
    /// # use chess_api::movement::{Move, Square};
    /// # use chess_api::piece::PieceColor;
    ///
    /// let mut board = Board::new();
    ///
    /// assert!(board.perform_move(Move::new(Square::new(4, 1), Square::new(4, 3))).is_ok()); // e4
    /// assert!(board.perform_move(Move::new(Square::new(5, 6), Square::new(5, 4))).is_ok()); // f6
    /// assert!(board.perform_move(Move::new(Square::new(3, 0), Square::new(7, 4))).is_ok()); // Qh5
    ///
    /// assert_eq!(board.is_king_attacked(PieceColor::WHITE), false);
    /// assert_eq!(board.is_king_attacked(PieceColor::BLACK), true);
    /// ```
    pub fn is_king_attacked(&self, color: PieceColor) -> bool {
        self.is_king_attacked_after_move(color, None)
    }

    /// # Returns iterator for every possible move from given square
    ///
    /// move order is not defined and may change in future
    ///
    /// ```
    /// # use chess_api::board::Board;
    /// # use chess_api::movement::{Move, Square};
    /// # use chess_api::piece::PieceColor;
    ///
    /// let mut board = Board::new();
    ///
    /// assert!(board.perform_move(Move::new(Square::new(4, 1), Square::new(4, 3))).is_ok()); // e4
    /// 
    /// assert_eq!(board.all_possible_moves_from_square(Square::new(3, 0)).count(), 4); // queen
    /// assert_eq!(board.all_possible_moves_from_square(Square::new(4, 3)).count(), 1);
    /// assert_eq!(board.all_possible_moves_from_square(Square::new(3, 1)).count(), 2);
    /// ```

    pub fn all_possible_moves_from_square<'a>(&'a self, start: Square) -> impl Iterator<Item = Move> + 'a {
        // todo optimize, piece should give subset of board's squares to test for move possibility
        // fx: for pawn we are testing all 64 destination squares but we should only test 4
        self.squares()
            .filter_map(move |(end, _)| if end == start { None } else { Some(end) })
            .map(move |end| Move::new(start, end))
            .filter(|m| self.check_move_possibility(*m).is_ok())
    }

    /// # Returns iterator for every possoble move by given color
    ///
    /// move order is not defined and may change in future
    ///
    /// ```
    /// # use chess_api::board::Board;
    /// # use chess_api::piece::PieceColor;
    ///
    /// # let board = Board::new();
    ///
    /// assert_eq!(board.all_possible_moves(None).count(), 40);
    /// ```
    pub fn all_possible_moves<'a>(&'a self, color: Option<PieceColor>) -> impl Iterator<Item = Move> +'a {
        self.pieces(color).flat_map(|(square, _)| self.all_possible_moves_from_square(square))
    }
}
