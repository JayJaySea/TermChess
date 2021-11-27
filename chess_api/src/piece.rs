use crate::movement::Move;
use std::ops::Not;


#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    Pawn, Rook, Knight, Bishop, Queen, King
}

#[derive(Debug)]
pub struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor,
    moved: bool
}




impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Piece {
        Piece {
            piece_type,
            piece_color,
            moved: false
        }
    }

    pub fn can_move_to(&self, m: Move, dest_occupied: bool) -> (bool, bool) {
        let (dx, dy) = m.to_deltas();

        match self.piece_type {
            PieceType::Knight => ( (dx == 2 && dy == 1) || (dx == 1 && dy == 2), false ),
            PieceType::Queen => ( dx == 0 || dy == 0 || dx == dy, true),
            PieceType::King => ( dx <= 1 && dy <= 1, false ),
            PieceType::Rook => ( dx == 0 || dy == 0, true ),
            PieceType::Bishop => ( dx == dy, true ),
            PieceType::Pawn => {
                let ((sx, sy), (ex, ey)) = m.to_coords();

                let distance = match self.piece_color {
                    PieceColor::WHITE => {
                        if ey > sy { 
                            ey - sy
                        } else {
                            return (false, false);
                        } 
                    },
                    PieceColor::BLACK => {
                        if ey < sy {
                            sy - ey
                        } else {
                            return (false, false);
                        }
                    }
                };


                if sx == ex && !dest_occupied {
                    match distance {
                        1 => (true, false),
                        2 => (!self.moved, true),
                        _ => (false, false)
                    }
                } else if dest_occupied && distance == 1 {
                    if sx > ex {
                        (sx - ex == 1, false) 
                    } else if sx < ex {
                        (ex - sx == 1, false) 
                    } else { (false, false) }
                } else { (false, false) } // todo en passant 
            },
        }
    }

    pub fn color(&self) -> PieceColor {
        self.piece_color
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn move_piece(&mut self) {
        self.moved = true;
    }    
}
