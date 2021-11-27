use chess_api::board::*;
use chess_api::piece::*;
use chess_api::movement::*;

#[test]
    fn basic_white_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
    
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 5))).is_ok(), true);

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 2))).is_ok(), false);

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 4))).is_ok(), false);
    }


    #[test]
    fn basic_black_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
    
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 1))).is_ok(), true);

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 4))).is_ok(), false);

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 4))).is_ok(), false);
    }

    #[test]
    fn blocked_basic_white_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(3, 5), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
    
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 5))).is_ok(), false);

        board.set(Square::new(3, 3), None);
        board.set(Square::new(3, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 5))).is_ok(), false);
    }

    #[test]
    fn blocked_basic_black_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(3, 1), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
    
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 1))).is_ok(), false);

        board.set(Square::new(3, 3), None);
        board.set(Square::new(3, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 1))).is_ok(), false);
    }

    #[test]
    fn white_pawn_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 5), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 5), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(5, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(1, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 5))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 5))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(1, 4))).is_ok(), false);
    }

    #[test]
    fn black_pawn_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 1), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 1), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(5, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(1, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 1))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 1))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(1, 2))).is_ok(), false);
    }

    #[test]
    fn white_pawn_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), false);
    }

    #[test]
    fn black_pawn_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), false);
    }

    #[test]
    fn basic_bishop_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), true);
        
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 0))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 6))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 0))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 7))).is_ok(), true);

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 3))).is_ok(), false);
        
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 6))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 6))).is_ok(), false);
    }

    #[test]
    fn bishop_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), true);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        
        board.set(Square::new(0, 0), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(0, 6), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(6, 0), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(7, 7), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 0))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 6))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 0))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 7))).is_ok(), true);
    }

    #[test]
    fn bishop_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));

        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), false);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        
        board.set(Square::new(0, 0), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(0, 6), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(6, 0), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(7, 7), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 0))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 6))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 0))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 7))).is_ok(), false);
    }

    #[test]
    fn bishop_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 0))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 6))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 0))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 7))).is_ok(), false);
    }

    #[test]
    fn basic_knight_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Knight, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 5))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 1))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(1, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 1))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(5, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(1, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 5))).is_ok(), true);

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(3, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 3))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), false);
    }

    #[test]
    fn knight_movement_near_positive_board_edges() {
        let mut board = Board::new_clear();

        board.set(Square::new(6, 6), Some(Piece::new(PieceType::Knight, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(6, 6), Square::new(4, 7))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(6, 6), Square::new(4, 5))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(6, 6), Square::new(5, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(6, 6), Square::new(7, 4))).is_ok(), true);
    }

    #[test]
    fn knight_movement_near_negative_board_edges() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Knight, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(0, 3))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(2, 3))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(3, 0))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(3, 2))).is_ok(), true);
    }

    #[test]
    fn queen_rook_basic_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 5))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(5, 1))).is_ok(), true);

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(5, 2))).is_ok(), false);

        board.set(Square::new(1, 1), None);

        board.set(Square::new(5, 5), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(5, 5), Square::new(1, 5))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(5, 5), Square::new(5, 1))).is_ok(), true);
        
    }

    #[test]
    fn queen_bishop_basic_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), true);
        
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 0))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 6))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 0))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 7))).is_ok(), true);

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 6))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 6))).is_ok(), false);
    }

    #[test]
    fn queen_rook_taking_oppnent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 5))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(5, 1))).is_ok(), true);
    }

    #[test]
    fn queen_bishop_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), true);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(0, 0), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(0, 6), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(6, 0), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(7, 7), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 0))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 6))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 0))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 7))).is_ok(), true);
    }
   
    #[test]
    fn queen_rook_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 5))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(5, 1))).is_ok(), false);
    }

    #[test]
    fn queen_bishop_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(2, 4))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(4, 4))).is_ok(), false);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(0, 0), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(0, 6), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(6, 0), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(7, 7), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 0))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 6))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 0))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 7))).is_ok(), false);
    }

    #[test]
    fn queen_rook_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
    
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 6))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(6, 1))).is_ok(), false);
    }

    #[test]
    fn queen_bishop_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 0))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(0, 6))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(6, 0))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(3, 3), Square::new(7, 7))).is_ok(), false);
    }

    #[test]
    fn basic_rook_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 5))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(5, 1))).is_ok(), true);
   
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(5, 5))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(5, 2))).is_ok(), false);

        board.set(Square::new(1, 1), None);

        board.set(Square::new(5, 5), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(5, 5), Square::new(1, 5))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(5, 5), Square::new(5, 1))).is_ok(), true);
    }

    #[test]
    fn rook_taking_oppnent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 5))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(5, 1))).is_ok(), true);
    }

    #[test]
    fn rook_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 5))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(5, 1))).is_ok(), false);
    }

    #[test]
    fn rook_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));
    
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 6))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(6, 1))).is_ok(), false);
    }
