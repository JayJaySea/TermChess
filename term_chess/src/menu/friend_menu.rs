use {std::io, std::io::*};
use crate::{Menu, *};

pub struct FriendMenu {
    time_mode: u32,
    time: u32,
    increment: u32,
    ranked: u32,
    side: u32,
    friend: String,
}

impl Menu for FriendMenu {
    fn display(&mut self) -> Option<Box<dyn Menu>> {
        clean_screen();
        self.print();

        self.choose_time_mode();
        self.choose_time();
        self.choose_increment();
        self.choose_ranked();
        self.choose_side();
        self.choose_friend();

        if !Input::proceed() {
            return Some(Box::new(FriendMenu::new()));
        }

        None
    }
}

impl FriendMenu {
    pub fn new() -> FriendMenu {
        FriendMenu { 
            time_mode: 0,
            time: 0,
            increment: 0,
            ranked: 0,
            side: 0,
            friend: String::new(),
        }
    }

    fn choose_time_mode(&mut self) {
        while self.time_mode == 0 {
            clean_screen();
            self.print();
            self.print_time_mode();
            self.time_mode = Input::one_to_(3); 
        }
    }

    fn print(&self) {
        println!("Game with a friend:");
        println!("  Variant: Standard");
    }

    fn print_time_mode(&self) {
        println!("  Time control: ");
        println!("  1. Real time ");
        println!("  2. Correspondence ");
        println!("  3. Unlimited ");
    }

    fn choose_time(&mut self) {
        //works as if and while at the same time
        while self.time_mode != 3 && self.time == 0 {
            clean_screen();
            self.print();
            self.format_time_mode();
            self.print_time_choice();

            //Works well only if Correspondence and Real Time has the same amount of options
            self.time = Input::one_to_(4);
        }
    }

    fn format_time_mode(&self) {
        match self.time_mode {
            1 => println!("  Time control: Real Time"),
            2 => println!("  Time control: Correspondence"),
            3 => println!("  Time control: Unlimited"),
            _ => println!("Something went wrong!"),
        }
    }

    fn print_time_choice(&self) {
        if self.time_mode == 1 {
            println!("  Time to play:");
            println!("  1. 5 minutes");
            println!("  2. 10 minutes");
            println!("  3. 15 minutes");
            println!("  4. 30 minutes");
        }
        else {
            println!("  Time for turn:");
            println!("  1. 1 day");
            println!("  2. 5 days");
            println!("  3. 10 days");
            println!("  4. 14 days");
        }
    }


    fn choose_increment(&mut self) {
        while self.time_mode == 1 && self.increment == 0 {
            clean_screen();
            self.print();
            self.format_time_mode();
            self.format_time_choice();
            self.print_increment_choice();

            self.increment = Input::one_to_(4);
        }
    }

    fn format_time_choice(&self) {
        if self.time_mode == 1 {
            print!("  Time to play:");
            match self.time {
                1 => println!(" 5 minutes"),
                2 => println!(" 10 minutes"),
                3 => println!(" 15 minutes"),
                4 => println!(" 30 minutes"),
                _ => println!("Something went wrong..."),
            }
            io::stdout().flush().unwrap();
        }
        else if self.time_mode == 2 {
            print!("  Time for turn:");
            match self.time {
                1 => println!(" 1 day"),
                2 => println!(" 5 days"),
                3 => println!(" 10 days"),
                4 => println!(" 14 days"),
                _ => (),
            }
            io::stdout().flush().unwrap();
        }
    }

    fn print_increment_choice(&self) {
        println!("  With increment:");
        println!("  1. 1 second");
        println!("  2. 3 seconds");
        println!("  3. 5 seconds");
        println!("  4. 15 seconds");
    }

    fn choose_ranked(&mut self) {
        while self.ranked == 0 {
            clean_screen();
            self.print();
            self.format_time_mode();
            self.format_time_choice();
            self.format_increment_choice();
            self.print_ranked_choice();

            self.ranked = Input::one_to_(2);
        }
    }

    fn format_increment_choice(&self) {
        if self.increment != 0 {
            print!("  With increment:");
            match self.increment {
                1 => println!(" 1 second"),
                2 => println!(" 3 seconds"),
                3 => println!(" 5 seconds"),
                4 => println!(" 15 seconds"),
                _ => (),
            }
            io::stdout().flush().unwrap();

        }
    }

    fn print_ranked_choice(&self) {
        println!("  Game will be: ");
        println!("  1. Ranked");
        println!("  2. Unranked");
    }

    fn choose_side(&mut self) {
        while self.side == 0 {
            clean_screen();
            self.print();
            self.format_time_mode();
            self.format_time_choice();
            self.format_increment_choice();
            self.format_ranked_choice();
            self.print_side_choice();

            self.side = Input::one_to_(3);
        }
    }

    fn format_ranked_choice(&self) {
        match self.ranked {
            1 => println!("  Game: Ranked"),
            2 => println!("  Game: Unranked"),
            _ => (),
        }
    }

    fn print_side_choice(&self) {
        println!("  Choose side: ");
        println!("  1. White ");
        println!("  2. Black ");
        println!("  3. Random ");
    }

    fn choose_friend(&mut self) {
        clean_screen();
        self.print();
        self.format_time_mode();
        self.format_time_choice();
        self.format_increment_choice();
        self.format_ranked_choice();
        self.format_side_choice();


        print!("  Your friend's nick: ");
        io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut self.friend).expect("Error: Failed to take standard input!"); 
        self.friend = String::from(self.friend.trim());

    }

    fn format_side_choice(&self) {
        match self.side {
            1 => println!("  Side: White"),
            2 => println!("  Side: Black"),
            3 => println!("  Side: Random"),
            _ => (),
        }
    }

    fn _reset_fields(&mut self) {
        self.time_mode = 0;
        self.time = 0;
        self.increment = 0;
        self.ranked = 0;
        self.side = 0;
        self.friend = String::new();
    }
}
