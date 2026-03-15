// Vincent Nguyen

use std::{io::{self, Write}, process::exit};

pub const MAX_INT32: i64 = (1 << 31) - 1;
pub const MIN_INT32: i64 = -(1 << 31);

pub enum OutputFormat {
    BIN,
    DEC,
    HEX
}

// I think correct approach is storing value as a 32 bit integer, an i32. 
// But I'll keep it as a string for now. May change later
pub struct Processor32 {
    value: String,
    overflow: u8,
    saturated: u8,
}

impl Processor32 {
    pub fn new(user_input: i64) -> Self {
        let mut value = user_input;
        let mut overflow = 0;
        let mut saturated = 0;

        // Clamps values to max and min if overflows. also raises flags.
        if value > MAX_INT32 {
            value = MAX_INT32;
            overflow = 1;
            saturated = 1;
        } else if value < MIN_INT32 {
            value = MIN_INT32;
            overflow = 1;
            saturated = 1;
        }

        let binary_value = format!("{:032b}", value as i32);

        Processor32 {
            value: binary_value,
            overflow,
            saturated,
        }
    }

    fn binary_to_decimal(&self) -> String {
        let val = u32::from_str_radix(&self.value, 2).unwrap() as i32;
        val.to_string()
    }

    fn binary_to_hex(&self) -> String {
        let val = u32::from_str_radix(&self.value, 2).unwrap();
        format!("0x{:08X}", val)
    }

    pub fn format(&self, format: OutputFormat) -> String {
        match format {
            OutputFormat::BIN => self.value.clone(),
            OutputFormat::DEC => self.binary_to_decimal(),
            OutputFormat::HEX => self.binary_to_hex()
        }
    }

    pub fn overflow(&self) -> u8 {
        self.overflow
    }

    pub fn saturated(&self) -> u8 {
        self.saturated
    }
}

pub fn run() {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_number() {
        let test_cpu = Processor32::new(123);
        assert_eq!(test_cpu.format(OutputFormat::BIN), "00000000000000000000000001111011");
        assert_eq!(test_cpu.format(OutputFormat::DEC), "123");
        assert_eq!(test_cpu.format(OutputFormat::HEX), "0x0000007B");
        assert_eq!(test_cpu.overflow(), 0);
        assert_eq!(test_cpu.saturated(), 0);
    }

    #[test]
    fn test_zero_value() {
        let test_cpu = Processor32::new(0);
        assert_eq!(test_cpu.format(OutputFormat::BIN), "00000000000000000000000000000000");
        assert_eq!(test_cpu.format(OutputFormat::DEC), "0");
        assert_eq!(test_cpu.format(OutputFormat::HEX), "0x00000000");
        assert_eq!(test_cpu.overflow(), 0);
        assert_eq!(test_cpu.saturated(), 0);
    }

    #[test]
    fn test_negative_number() {
        let test_cpu = Processor32::new(-123);
        assert_eq!(test_cpu.format(OutputFormat::BIN), "11111111111111111111111110000101");
        assert_eq!(test_cpu.format(OutputFormat::DEC), "-123");
        assert_eq!(test_cpu.format(OutputFormat::HEX), "0xFFFFFF85");
        assert_eq!(test_cpu.overflow(), 0);
        assert_eq!(test_cpu.saturated(), 0);
    }

    #[test]
    fn test_max_boundary_value() {
        let test_cpu = Processor32::new(MAX_INT32);
        assert_eq!(test_cpu.format(OutputFormat::BIN), "01111111111111111111111111111111");
        assert_eq!(test_cpu.format(OutputFormat::DEC), "2147483647");
        assert_eq!(test_cpu.format(OutputFormat::HEX), "0x7FFFFFFF");
        assert_eq!(test_cpu.overflow(), 0);
        assert_eq!(test_cpu.saturated(), 0);
    }

    #[test]
    fn test_min_boundary_value() {
        let test_cpu = Processor32::new(MIN_INT32);
        assert_eq!(test_cpu.format(OutputFormat::BIN), "10000000000000000000000000000000");
        assert_eq!(test_cpu.format(OutputFormat::DEC), "-2147483648");
        assert_eq!(test_cpu.format(OutputFormat::HEX), "0x80000000");
        assert_eq!(test_cpu.overflow(), 0);
        assert_eq!(test_cpu.saturated(), 0);
    }

    #[test]
    fn test_max_overflow() {
        let test_cpu = Processor32::new(MAX_INT32 + 1);
        assert_eq!(test_cpu.format(OutputFormat::BIN), "01111111111111111111111111111111");
        assert_eq!(test_cpu.format(OutputFormat::DEC), "2147483647");
        assert_eq!(test_cpu.format(OutputFormat::HEX), "0x7FFFFFFF");
        assert_eq!(test_cpu.overflow(), 1);
        assert_eq!(test_cpu.saturated(), 1);
    }

    #[test]
    fn test_min_overflow() {
        let test_cpu = Processor32::new(MIN_INT32 - 1);
        assert_eq!(test_cpu.format(OutputFormat::BIN), "10000000000000000000000000000000");
        assert_eq!(test_cpu.format(OutputFormat::DEC), "-2147483648");
        assert_eq!(test_cpu.format(OutputFormat::HEX), "0x80000000");
        assert_eq!(test_cpu.overflow(), 1);
        assert_eq!(test_cpu.saturated(), 1);
    }
}