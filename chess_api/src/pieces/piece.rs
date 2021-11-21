use crate::board::Board;
use crate::movement::Move;
use std::ops::Not;


#[derive(PartialEq, Copy, Clone)]
pub enum PieceColor {
    WHITE,
    BLACK
}

impl Not for PieceColor {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PieceColor::WHITE => PieceColor::BLACK,
            PieceColor::BLACK => PieceColor::WHITE
        }
    }
}

pub trait Piece {
    /// # Basic move validation
    ///
    /// only checks if move is not blocked or piece can move specific way
    /// destination piece color may or may not be checked
    /// it will not check if king is attacked after this move
    ///
    /// for complete move validation check Board::is_move_possible()
    ///
    fn can_move_to(&self, board: &Board, m: Move) -> (bool, bool);
    
    /// # Returns character representing piece
    ///
    /// should be used as display character unless needed otherwise
    ///
    fn get_character(&self) -> char;

    fn color(&self) -> PieceColor;
}
