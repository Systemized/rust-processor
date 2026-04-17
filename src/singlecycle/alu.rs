// Vincent Nguyen

pub fn alu(a: u32, b: u32, op_or: bool) -> u32 {
    if op_or { a | b } else { a & b }
}
