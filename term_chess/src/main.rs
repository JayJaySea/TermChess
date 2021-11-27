//use term_chess::main_menu::MainMenu;
//use term_chess::Menu;

use lichess_api::test;

fn main() {
    test();

/*
    let mut menu: Option<Box<dyn Menu>> = Some(Box::new(MainMenu::new()));
    while let Some(mut x) = menu.take() { 
        menu = x.display();
    }
*/
}
