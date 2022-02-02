use chess_api::board::Board;

pub struct BoardManager {
    board: Board,
    contents: String,
}

impl BoardManager {
    pub fn new() -> BoardManager {
        BoardManager { 
            board: Board::new(), 
            contents: String::new(),
        }
    }

    pub fn init_board_ui(&mut self) {
        self.draw_board();
    }

    fn draw_board(&mut self) {

    }

    pub fn display(&mut self) {
    }

}
