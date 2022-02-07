use tui::{
    widgets::Widget,
    buffer::Buffer,
    layout::{ Alignment, Rect },
    style::{ Color, Style, Modifier},
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
                        PieceColor::BLACK => 2,
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
        let white = Style::default()
            .fg(Color::White)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD);

        let black = Style::default()
            .fg(Color::Red)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD);

        let dots = Style::default()
            .fg(Color::Gray)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD);

        for (n, c) in (0..64).zip(self.display.chars()) {
            if n % 8 == 0 {
                buf.set_string(area.x, area.y + n as u16/8, (8 - n/8).to_string(), dots);
            }

            let mut style = dots;
            if self.colors[n] == 1 {
                style = white;
            }
            else if self.colors[n] == 2 {
                style = black;
            }
            buf.set_string(2*(n as u16)%16 + area.x + 2, (n as u16)/8 + area.y, c.to_string(), style);
        }
        buf.set_string(area.x + 2, area.y + 8, String::from("a b c d e f g h"), dots);
    }
}
