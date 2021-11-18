use super::pieces::*;
use super::Square;

pub struct Board {
    pieces: [Option<Box<dyn Piece>>; 64]
}

impl Board {
    pub fn new() -> Board {
        const INIT: Option<Box<dyn Piece>> = None;

        let board = Board {
            pieces: [INIT; 64]
        };

        board
    }

    pub fn get_piece(&self, square: Square) -> &Option<Box<dyn Piece>> {
        &self.pieces[square.to_index()]
    }

    pub fn set(&mut self, square: Square, piece: Option<Box<dyn Piece>>) {
        self.pieces[square.to_index()] = piece;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_uinitialized_board() {
        let mut b = Board::new();
    
        let p = b.get_piece(Square::new(0, 0));

        if let Some(_) = p {
            panic!("should be empty!");
        }

        if let Some(_) = p {
            panic!("should be empty!");
        }

        b.set(Square::new(0, 0), None);

        //if let Some(_) = p {
        //    panic!("should be empty!");
        //}

        let p = b.get_piece(Square::new(0, 0));

        if let Some(_) = p {
            panic!("should be empty!");
        }
    }
}
