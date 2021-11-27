use std::io;
use std::io::{ Write };

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
        let mut line = String::new();
        let mut input:u32 = 0;

        std::io::stdin().read_line(&mut line).expect("Error: Failed to take standard input!"); 

        match line.trim().parse::<u32>() {
            Ok(x) 
                if x > 0 && x <= n 
                => { input = x; },

            Ok(_) | Err(_) => (),
        }

        input
    }


    fn proceed() -> bool {
        let mut line = String::new();

        print!("\nProceed? [Y/n] ");
        io::stdout().flush().unwrap();

        loop {
            line.clear();
            std::io::stdin().read_line(&mut line).expect("Error: Failed to take standard input!"); 
            if line.trim() == "Y" {
                return true;
            }
            else if line.trim() == "n" {
                return false;
            }
        }
    }
}

fn clean_screen() {
    print!("\x1B[2J\x1B[1;1H");   
}
