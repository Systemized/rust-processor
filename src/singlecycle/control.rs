// Vincent Nguyen

use super::instruction::{Instr, INV};

pub struct Control { pub op_or: bool, pub invert_rs: bool, pub reg_write: bool }

pub fn decode(i: &Instr) -> Control {
    Control {
        op_or: (i.funct & 1) == 1,
        invert_rs: (i.funct & INV) != 0,
        reg_write: true,
    }
}
