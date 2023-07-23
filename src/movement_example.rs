use std::io::{self, Write};

enum Location {
    Home,
    Forest,
    River,
    Town,
}

fn main() {
    let mut current_location = Location::Home;

    loop {
        // Display current location and options to the player
        match current_location {
            Location::Home => {
                println!("You are at home.");
                println!("1. Go to the forest");
                println!("2. Go to the town");
            }
            Location::Forest => {
                println!("You are in the forest.");
                println!("1. Go back home");
                println!("2. Go to the river");
            }
            Location::River => {
                println!("You are at the river.");
                println!("1. Go back to the forest");
                println!("2. Go to the town");
            }
            Location::Town => {
                println!("You are in the town.");
                println!("1. Go back home");
                println!("2. Go to the river");
            }
        }

        // Read player input
        let mut input = String::new();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<usize>().unwrap();

        // Handle player's choice based on the current location
        match current_location {
            Location::Home => {
                match choice {
                    1 => current_location = Location::Forest,
                    2 => current_location = Location::Town,
                    _ => println!("Invalid choice."),
                }
            }
            Location::Forest => {
                match choice {
                    1 => current_location = Location::Home,
                    2 => current_location = Location::River,
                    _ => println!("Invalid choice."),
                }
            }
            Location::River => {
                match choice {
                    1 => current_location = Location::Forest,
                    2 => current_location = Location::Town,
                    _ => println!("Invalid choice."),
                }
            }
            Location::Town => {
                match choice {
                    1 => current_location = Location::Home,
                    2 => current_location = Location::River,
                    _ => println!("Invalid choice."),
                }
            }
        }
    }
}
