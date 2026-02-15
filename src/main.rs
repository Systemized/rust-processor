// // Vincent Nguyen
use std::{io::{self, Write}, process::exit};

// Max and Min range for 2^32 bits
const MAX_INT32: i64 = (1 << 31) - 1;
const MIN_INT32: i64 = -(1 << 31);

enum OutputFormat {
    BIN,
    DEC,
    HEX,
}
struct Processor32 {
    value: String,
    overflow: u8,
    saturated: u8,
}

impl Processor32 {
    fn new(input: i64) -> Self {
        let mut value = input;
        let mut overflow = 0;
        let mut saturated = 0;

        if input > MAX_INT32 {
            value = MAX_INT32;
            overflow = 1;
            saturated = 1;
        } else if input < MIN_INT32 {
            value = MIN_INT32;
            overflow = 1;
            saturated = 1;
        }

        // Convert to binary. Store/operate interally only on this 32bit.
        let binary_value = format!("{:032b}", value as i32);

        Processor32 {
            value: binary_value,
            overflow,
            saturated,
        }        
    }

    fn format(&self, user_format: OutputFormat) -> String {
        match user_format {
            OutputFormat::BIN => self.value.clone(),
            OutputFormat::DEC => self.binary_to_decimal(),
            OutputFormat::HEX => self.binary_to_hex(),
        }
    }

    fn binary_to_decimal(&self) -> String {
        let value = u32::from_str_radix(&self.value, 2).unwrap();
        let value = value as i32;
        value.to_string()
    }

    fn binary_to_hex(&self) -> String {
        let value = u32::from_str_radix(&self.value, 2).unwrap();
        format!("0x{:08X}", value)
    }
}


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
    println!("Saturated:    ({}/1)", cpu.saturated);
    println!("Overflow:     ({}/1)", cpu.overflow);

}