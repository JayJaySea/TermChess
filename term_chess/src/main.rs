use term_chess::main_menu::MainMenu;
use term_chess::Menu;

fn main() {
    let mut menu: Option<Box<dyn Menu>> = Some(Box::new(MainMenu::new()));
    while let Some(mut x) = menu.take() { 
        menu = x.display();
    }
}
