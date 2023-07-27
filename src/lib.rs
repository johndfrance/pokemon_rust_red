/*
Title: lib.rs
Project: Pokemon Rust Red

Desc: Holds a variety of useful functions that don't relate to any sort of game logic. Things like
user input validation, some math operations, text presentation etc.
 */
use std::{io, thread};
use std::fmt::format;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use colored::{Color, Colorize};
use colored::Color::TrueColor;

// Custom colours to correspond to the cities of Kanto.
pub const VIRIDIAN: Color = TrueColor{r:64, g:130, b:109};
pub const PEWTER: Color = TrueColor {r:137, g:148, b: 153};
pub const CERULEAN: Color = TrueColor {r:0, g:123, b:167};
pub const VERMILION: Color = TrueColor {r:227, g:66, b:52};
pub const LAVENDER: Color = TrueColor {r:150, g:123, b:182};
pub const CELADON: Color = TrueColor {r:172, g:225, b:175};
pub const FUCHSIA: Color = TrueColor {r:255, g:0, b: 255};
pub const SAFFRON: Color = TrueColor {r:244, g:196, b:48};
pub const CINNABAR: Color = TrueColor {r:170, g:56, b:30};


fn type_text(text: &str) {
    let delay = 25;
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(delay));
    }
}
pub fn travelling(destination: &str){
    let ellipses = ". . .";
    let delay = 250;
    println!();
    for c in ellipses.chars(){
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay))
    }

    type_text(format!("Going to {}", destination).as_str());
    for c in ellipses.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay))
    }
    println!();
}

//Validates that a user's input is between 1 and a given max value.
pub fn get_user_input(max: u8) -> u8 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        // Parse the user input into a u8
        match input.trim().parse::<u8>() {
            Ok(num) => {
                // Check if the number is between 1 and max
                if num >= 1 && num <= max {
                    return num;
                } else {
                    println!("Invalid input. Please enter a number between 1 and 4.");
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}


fn main() {}