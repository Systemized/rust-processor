// Vincent Nguyen

use std::{io::{self}, process::exit};

mod processor;
mod kmap;
mod memory;
fn main() {

    let mut user_input = String::new();
    println!("Select a number to run:\n  [0] Processor\n  [1] K-Map\n  [2] Memory");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let user_input: u8 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error. Invalid Format");
            exit(1);
        }
    };
    
    match user_input {
        0 => processor::run(),
        1 => kmap::run(),
        2 => memory::run(),
        _ => {
            println!("Error. Choose from options.");
            exit(1);
        }
    };

}