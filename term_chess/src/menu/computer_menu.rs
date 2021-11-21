use {std::io, std::io::*};
use crate::{Menu, *};

pub struct ComputerMenu {
    strength: u32,
    side: u32,
}

impl Menu for ComputerMenu {
    fn display(&mut self) -> Option<Box<dyn Menu>> {
        self.choose_strength();
        self.choose_side();

        clean_screen();
        self.print();
        self.format_strength_choice();
        self.format_side_choice();

        if !Input::proceed() {
            return Some(Box::new(ComputerMenu::new()));
        }
    
        None
    }
}

impl ComputerMenu {
    pub fn new() -> ComputerMenu {
        ComputerMenu {
            strength: 0,
            side: 0,
        }
    }


    fn choose_strength(&mut self) {
        while self.strength == 0 {
            clean_screen();
            self.print();
            self.print_strength_choice();

            self.strength = Input::one_to_(9);
        }
    }

    fn print(&self) {
        println!("Game with computer:");
        println!("  Variant: Standard");
        println!("  Time control: Unlimited");
    }

    fn print_strength_choice(&self) {
        print!("  Choose strength (1 - 9): ");

        io::stdout().flush().unwrap();
    }



    fn choose_side(&mut self) {
        while self.side == 0 {
            clean_screen();
            self.print();
            self.format_strength_choice();

            self.print_side();
            self.side = Input::one_to_(3);
        }
    }

    fn format_strength_choice(&self) {
        println!("  Strength: {}", self.strength);
    }

    fn print_side(&self) {
        println!("  Choose side:");
        println!("      1. White");
        println!("      2. Black");
        println!("      3. Random");
    }

    fn format_side_choice(&self) {
        match self.side {
            1 => println!("  Side: White"),
            2 => println!("  Side: Black"),
            3 => println!("  Side: Random"),
            _ => println!("  Error: Something went wrong!"),
        }
    }
}
