use std::{io, thread, time::Duration};
use term_chess::board_manager::chess_board::ChessBoard;
use chess_api::board::Board;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Terminal,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = Rect { x: 10, y: 10, width: 18, height: 9 };
        let board = ChessBoard::new(&Board::new());
        f.render_widget(board, size);
    })?;

    thread::sleep(Duration::from_millis(2500));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
