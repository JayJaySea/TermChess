use chess_api::board::*;
use chess_api::movement::*;
use chess_api::piece::*;

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
        assert_eq!(b.perform_move(Move::new(Square::new(2, 1), Square::new(2, 3))).is_ok(), true);
        assert_eq!(b.perform_move(Move::new(Square::new(4, 6), Square::new(4, 4))).is_ok(), true);
        
        // Nc3 Nc6
        assert_eq!(b.perform_move(Move::new(Square::new(1, 0), Square::new(2, 2))).is_ok(), true);
        assert_eq!(b.perform_move(Move::new(Square::new(1, 7), Square::new(2, 5))).is_ok(), true);

        // g3 g6
        assert_eq!(b.perform_move(Move::new(Square::new(6, 1), Square::new(6, 2))).is_ok(), true);
        assert_eq!(b.perform_move(Move::new(Square::new(6, 6), Square::new(6, 5))).is_ok(), true);
    }

    #[test]
    fn piece_iterator() {
        let board = Board::new();

        assert_eq!(board.squares().count(), 64);
        assert_eq!(board.pieces(None).count(), 32);
        assert_eq!(board.pieces(Some(PieceColor::WHITE)).count(), 16);
        assert_eq!(board.pieces(Some(PieceColor::BLACK)).count(), 16);
    }

    #[test]
    fn white_leaving_king_checked_after_move() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::King, PieceColor::WHITE)));
        board.set(Square::new(1, 2), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(1, 6), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 2), Square::new(6, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 2), Square::new(1, 3))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 2), Square::new(1, 6))).is_ok(), true);

        board.set(Square::new(1, 2), None);
        
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(2, 2))).is_ok(), true);

        board.set(Square::new(1, 6), None);
        board.set(Square::new(1, 2), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 2))).is_ok(), true);
    }

    #[test]
    fn black_leaving_king_checked_after_move() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::King, PieceColor::BLACK)));
        board.set(Square::new(1, 2), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));
        board.set(Square::new(1, 6), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 2), Square::new(6, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 2), Square::new(1, 3))).is_ok(), true);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 2), Square::new(1, 6))).is_ok(), true);

        board.set(Square::new(1, 2), None);
        
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 2))).is_ok(), false);
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(2, 2))).is_ok(), true);

        board.set(Square::new(1, 6), None);
        board.set(Square::new(1, 2), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 2))).is_ok(), true);
    }

    #[test]
    fn err_no_source_piece() {
        let board = Board::new_clear();
    
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 2))).err().unwrap(), MoveFailReason::NoSourcePiece);
    }

    #[test]
    fn err_friendly_fire() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(6, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
    
        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(6, 1))).err().unwrap(), MoveFailReason::FriendlyFire);
    }

    #[test]
    fn err_illegal_move() {
        let board = Board::new();

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(1, 6))).err().unwrap(), MoveFailReason::IllegalMove);
    }

    #[test]
    fn err_king_attacked() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::King, PieceColor::WHITE)));
        board.set(Square::new(2, 1), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.check_move_possibility(Move::new(Square::new(1, 1), Square::new(0, 1))).err().unwrap(), MoveFailReason::KingAttacked);
    }

    #[test]
    fn post_move_normal() {
        let mut board = Board::new();

        assert_eq!(board.perform_move(Move::new(Square::new(2, 1), Square::new(2, 3))).ok().unwrap(), PostMoveState::Normal);
    }

    #[test]
    fn post_move_check() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::King, PieceColor::WHITE)));
        board.set(Square::new(5, 5), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));

        assert_eq!(board.perform_move(Move::new(Square::new(5, 5), Square::new(5, 1))).ok().unwrap(), PostMoveState::Check);
    }

    #[test]
    fn post_move_stelemate() {
        let mut board = Board::new_clear();

        board.set(Square::new(0, 1), Some(Piece::new(PieceType::King, PieceColor::WHITE)));
        board.set(Square::new(2, 1), Some(Piece::new(PieceType::Knight, PieceColor::BLACK)));
        board.set(Square::new(1, 7), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));

        assert_eq!(board.perform_move(Move::new(Square::new(1, 7), Square::new(1, 6))).ok().unwrap(), PostMoveState::Stelemate);
    }

    #[test]
    fn post_move_checkmate() {
        let mut board = Board::new_clear();

        board.set(Square::new(0, 0), Some(Piece::new(PieceType::King, PieceColor::BLACK)));
        board.set(Square::new(7, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(0, 2), Some(Piece::new(PieceType::King, PieceColor::WHITE)));

        assert_eq!(board.perform_move(Move::new(Square::new(7, 1), Square::new(7, 0))).ok().unwrap(), PostMoveState::Checkmate);
    }
