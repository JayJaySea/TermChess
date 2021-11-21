use crate::{Menu, *};

pub struct QuickPairingMenu {
    time: u32,
}

impl Menu for QuickPairingMenu {
    fn display(&mut self) -> Option<Box<dyn Menu>> {
        self.choose_time();

        None
    }
}

impl QuickPairingMenu {
    pub fn new() -> QuickPairingMenu {
        QuickPairingMenu {
            time: 0,
        }
    }
    
    fn choose_time(&mut self) {
        while self.time == 0 {
            clean_screen();
            self.print();
            self.print_time_choice();

            self.time = Input::one_to_(7);
        }
    }

    fn print(&self) {
        println!("Quick pairing:");
        println!("  Variant: Standard");
    }

    fn print_time_choice(&self) {
        println!("  1. 5 + 0 Blitz");
        println!("  2. 5 + 3 Blitz");
        println!("  3. 10 + 0 Rapid");
        println!("  4. 10 + 5 Rapid");
        println!("  5. 15 + 10 Rapid");
        println!("  6. 30 + 0 Classical");
        println!("  7. 30 + 20 Classical");
    }
}
