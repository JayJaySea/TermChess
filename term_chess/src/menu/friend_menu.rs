use crate::{Menu, *};

pub struct FriendMenu;

impl Menu for FriendMenu {
    fn display(&mut self) -> Option<Box<dyn Menu>> {
        clean_screen();
        self.print();

        None
    }
}

impl FriendMenu {
    pub fn new() -> FriendMenu {
        let mut menu = FriendMenu { };
        menu.display();

        menu
    }

    fn print(&self) {
        println!("Game with a friend:");
        println!("  Variant: Standard");
        println!("  TimeControl: ");
        println!("  1. Real time ");
        println!("  2. Correspondence ");
        println!("  3. Unlimited ");
    }
}
