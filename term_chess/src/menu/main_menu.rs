use crate::{Menu, *};
use super::quick_pairing_menu::QuickPairingMenu;
use super::friend_menu::FriendMenu;
use super::computer_menu::ComputerMenu;

pub struct MainMenu {
    choice: u32,
}

impl Menu for MainMenu {
    fn display(&mut self) -> Option<Box<dyn Menu>> {
        while self.choice == 0 {
            clean_screen();
            self.print();

            self.choice = Input::one_to_(4);
        }

        match self.choice {
            1 => { Some(Box::new(QuickPairingMenu::new())) },
            2 => { Some(Box::new(FriendMenu::new())) },
            3 => { Some(Box::new(ComputerMenu::new())) },
            _ => None
        }
    }
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            choice: 0,
        }
    }

    fn print(&self) {
        println!("Welcome, choose an option:");
        println!("  1. Quick pairing");
        println!("  2. Play with a friend");
        println!("  3. Play with the computer");
        println!("  4. Exit");
    }
}



#[cfg(test)]
mod test {
    use super::*;
}
