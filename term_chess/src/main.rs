use {std::io, std::io::*};

fn main() {
    main_menu();
}

fn main_menu() {
    let mut option = String::new();
    let mut choice: u32 = 100;

    loop {
        clean_screen();
        print_main_menu();

        std::io::stdin().read_line(&mut option).expect("Error: Failed to take standard input!"); 
        match option.trim().parse::<u32>() {
            Ok(x) if x < 6 && x > 0 => { choice = x; break; },
            Ok(_) | Err(_) => (),
        }

        option.clear();
    }

    match choice {
        1 => quick_pairing(),
        2 => friend_game(),
        3 => computer_game(),
        _ => ()
    }
}

fn print_main_menu() {
    println!("Welcome, choose an option:");
    println!("  1. Quick pairing");
    println!("  2. Play with a friend");
    println!("  3. Play with the computer");
    println!("  4. Exit");
}



fn clean_screen() {
    print!("\x1B[2J\x1B[1;1H");   
}



fn quick_pairing() {
    clean_screen();
    print_quick_pairing();
}

fn print_quick_pairing() {
    println!("Quick pairing:");
}



fn friend_game() {
    clean_screen();
    print_friend_game();
}

fn print_friend_game () {
    println!("Friend game");
}



fn computer_game() {
    let mut option = String::new();
    let mut strength: u32 = 0;
    let mut color: u32 = 0;

    loop {
        clean_screen();
        print_strength_choice();

        std::io::stdin().read_line(&mut option).expect("Error: Failed to take standard input!"); 
        match option.trim().parse::<u32>() {
            Ok(x) if x > 0 && x < 10 => { strength = x; },
            Ok(_) | Err(_) => (),
        }

        option.clear();

        print_color_choice();
        std::io::stdin().read_line(&mut option).expect("Error: Failed to take standard input!"); 
        match option.trim().parse::<u32>() {
            Ok(x) if x > 0 && x < 4 => { color = x; break; },
            Ok(_) | Err(_) => (),
        }
    }

    //Tutaj jakiś match z komunikacją z api
    println!("Strength set to {}", strength);
    println!("Color set to {}", color);
}

fn print_strength_choice() {
    println!("Game with computer:");
    println!("  Variant: Standard");
    println!("  Time control: Unlimited");
    print!("  Choose strength (1 - 9): ");

    io::stdout().flush().unwrap();
}

fn print_color_choice() {
    println!("Choose color:");
    println!("  1. White");
    println!("  2. Black");
    println!("  3. Random");
}
