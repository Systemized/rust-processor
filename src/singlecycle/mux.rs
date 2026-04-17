// Vincent Nguyen

// ALU input-A mux: selects between rs and ~rs (1-bit invert) based on invert flag
pub fn invert_mux(rs: u32, invert: bool) -> u32 {
    if invert { !rs & 1 } else { rs }
}
