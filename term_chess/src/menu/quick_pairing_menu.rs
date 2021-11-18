use crate::{Menu, *};

pub struct QuickPairingMenu { }

impl Menu for QuickPairingMenu {
    fn display(&mut self) -> Option<Box<dyn Menu>> {
        clean_screen();
        self.print();
        None
    }
}

impl QuickPairingMenu {
    pub fn new() -> QuickPairingMenu {
        let mut menu = QuickPairingMenu { };
        menu.display();

        menu
    }
    
    fn print(&self) {
        println!("Quick pairing:");
    }
}
