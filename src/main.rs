// Vincent Nguyen

use std::{io::{self, Write}, process::exit};

mod processor;

use processor::Processor32;
use crate::processor::OutputFormat;

fn main() {

    let mut user_input = String::new();
    let mut user_format = String::new();
    
    print!("Input a number: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    
    let user_input: i64 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error. Invalid Input");
            exit(1);
        }
    };
    let cpu = Processor32::new(user_input);

    println!("Select an Output format:\n  BIN\n  DEC\n  HEX\n");

    io::stdin()
        .read_line(&mut user_format)
        .expect("Failed to read input");

    let format_result = match user_format.to_uppercase().trim() {
        "BIN" => cpu.format(OutputFormat::BIN),
        "DEC" => cpu.format(OutputFormat::DEC),
        "HEX" => cpu.format(OutputFormat::HEX),
        _ => {
            println!("Error. Invalid Format");
            exit(1);
        }
    };

    println!("\nValue Output: {}", format_result);
    println!("Saturated:    ({}/1)", cpu.overflow());
    println!("Overflow:     ({}/1)", cpu.saturated());

}