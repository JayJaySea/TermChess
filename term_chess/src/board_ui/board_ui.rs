use std::collections::HashMap;
use cursive::theme::Theme;
use cursive::views::Canvas;
use cursive::theme::ColorStyle;
use chess_api::board::{Board, PostMoveState, MoveFailReason};
use chess_api::piece::PieceType;

pub struct BoardUI {
    board: Board,
    offset: (u32, u32),
}

impl<'a> BoardUI {
    pub fn new() -> BoardUI {
        BoardUI { 
            board: Board::new(), 
            offset: (0, 0),
        }
    }

    pub fn get_view(&'a mut self) -> Canvas<String>  {

    let state = String::new();
    let mut display = String::new();
    let mut color: [u8; 64];

    for (square, piece) in self.board.squares() {
        match piece {
            Some(p) => {
                display.push(self.get_piece_char(p.piece_type()));
            },
            None => {
                display.push('.');
            },
        }
    }



    let canvas = Canvas::new(state)
        .with_draw(move |text: &String, printer| {
            printer.with_color(ColorStyle::primary(), |printer| {
                printer.print((1, 1), &display);
            });

            printer.with_color(ColorStyle::tertiary(), |printer| {
                printer.print((1, 1), "Hello");
            });
        })
        .with_required_size(|_, _| (28, 28).into());

        canvas
    }

    fn get_piece_char(&self, piece: PieceType) -> char {
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
            //for y in 0..9 {
            //    for x in -1..8 {
            //        printer.with_color(ColorStyle::primary(), |printer| {
            //            if x > -1 && y < 8 {
            //                printer.print((2*x+5,y+2), ".");
            //            }
            //            if y == 8 && x == 0 {
            //                printer.print((x+5, y+2), "a b c d e f g h");
            //            }
            //            if x == -1 && y < 8 {
            //                printer.print((x+4, y+2), &i32::abs(8-y).to_string());
            //            }
            //        });
            //    }
            //}
