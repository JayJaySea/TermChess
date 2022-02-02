use tui::{
    widgets::Widget,
    buffer::Buffer,
    layout::{ Alignment, Rect },
    style::{ Color, Style },
};

use chess_api::board::Board;
use chess_api::piece::PieceType;
use chess_api::piece::PieceColor;

#[derive(Debug, Clone, PartialEq)]
pub struct ChessBoard {
    display: String,
    colors: [u8; 64],
}

impl ChessBoard {
    pub fn new(board: &Board) -> ChessBoard {
        let mut display = String::new();
        let mut colors = [0; 64];

        for (color, (_, piece)) in colors.iter_mut().zip(board.squares()) {
            match piece {
                Some(p) => {
                    display.push(ChessBoard::get_piece_char(p.piece_type()));
                    *color = match p.color() {
                        PieceColor::WHITE => 1,
                        PieceColor::BLACK => 0,
                    }
                },
                None => {
                    display.push('.');
                    *color = 0;
                },
            }
        }

        ChessBoard { display, colors}
    }

    fn get_piece_char(piece: PieceType) -> char {
        match piece {
            PieceType::Pawn => '♟',
            PieceType::Knight => '♞',
            PieceType::Bishop => '♝',
            PieceType::Rook => '♜',
            PieceType::Queen => '♛',
            PieceType::King => '♚',
        }
    }
}



impl Widget for ChessBoard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (n, c) in (0..64).zip(self.display.chars()) {
            if n % 8 == 0 {
                buf.set_string(area.x, area.y + n/8, (8 - n/8).to_string(), Style::default());
            }
            buf.set_string(2*n%16 + area.x + 2, n/8 + area.y, c.to_string(), Style::default());
        }
        buf.set_string(area.x + 2, area.y + 8, String::from("a b c d e f g h"), Style::default());
    }
}
