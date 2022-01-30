//use term_chess::main_menu::MainMenu;
//use term_chess::Menu;
use cursive::{Cursive, CursiveExt};
use cursive::views::TextView;
use term_chess::board_ui::board_ui::BoardUI;

fn main() {
    let mut siv = Cursive::new();

    let mut b = BoardUI::new();
    siv.add_layer(b.get_view());
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}
