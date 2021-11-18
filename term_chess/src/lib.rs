pub mod menu {
    pub mod main_menu;
    mod quick_pairing_menu;
    mod friend_menu;
    mod computer_menu;
}

pub use crate::menu::main_menu;

//Utils for now here, will be moved somewhere else later (maybe)
pub trait Menu {
    fn display(&mut self) -> Option<Box<dyn Menu>>;
}

struct Input {}

impl Input {
    fn one_to_(n: u32) -> u32 {
        let mut option = String::new();
        let mut input:u32 = 0;

        std::io::stdin().read_line(&mut option).expect("Error: Failed to take standard input!"); 
        match option.trim().parse::<u32>() {
            Ok(x) if  x > 0 && x <= n => { input = x; },
            Ok(_) | Err(_) => (),
        }

        input
    }

    fn proceed() { }
}

fn clean_screen() {
    print!("\x1B[2J\x1B[1;1H");   
}
