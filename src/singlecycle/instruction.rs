// Vincent Nguyen

// funct field: bit 0 selects AND(0)/OR(1), bit 1 = invert rs
pub const F_AND: u8 = 0b00;
pub const F_OR:  u8 = 0b01;
pub const INV:   u8 = 0b10;

pub struct Instr { pub rd: usize, pub rs: usize, pub rt: usize, pub funct: u8 }
