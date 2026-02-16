// Vincent Nguyen

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

        if value > MAX_INT32 {
            value = MAX_INT32;
            overflow = 1;
            saturated = 1
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