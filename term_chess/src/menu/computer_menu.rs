use {std::io, std::io::*};
use crate::{Menu, *};

pub struct ComputerMenu {
    strength: u32,
    color: u32,
}

impl Menu for ComputerMenu {
    fn display(&mut self) -> Option<Box<dyn Menu>> {
        self.choose_strength();
        self.choose_color();

        clean_screen();
        self.print();
        self.format_strength_choice();
        self.format_color_choice();
        None
    }
}

impl ComputerMenu {
    pub fn new() -> ComputerMenu {
        ComputerMenu {
            strength: 0,
            color: 0,
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



    fn choose_color(&mut self) {
        while self.color == 0 {
            clean_screen();
            self.print();
            self.format_strength_choice();

            self.print_color_choice();
            self.color = Input::one_to_(3);
        }
    }

    fn format_strength_choice(&self) {
        println!("  Strength: {}", self.strength);
    }

    fn print_color_choice(&self) {
        println!("  Choose color:");
        println!("      1. White");
        println!("      2. Black");
        println!("      3. Random");
    }

    fn format_color_choice(&self) {
        match self.color {
            1 => println!("  Color: White"),
            2 => println!("  Color: Black"),
            3 => println!("  Color: Random"),
            _ => println!("  Error: Something went wrong!"),
        }
    }
}
