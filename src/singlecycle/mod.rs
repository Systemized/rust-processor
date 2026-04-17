// Vincent Nguyen

use std::io::{self, Write};

mod instruction;
mod control;
mod alu;
mod mux;
mod regfile;

use instruction::{Instr, F_AND, F_OR, INV};
use control::decode;
use alu::alu;
use mux::invert_mux;
use regfile::Regs;

fn step(rf: &mut Regs, i: &Instr) {
    // Fetch is implicit (program array). Decode -> Execute -> Write-back.
    let c = decode(i);
    let rs = rf.read(i.rs);
    let rt = rf.read(i.rt);
    let a = invert_mux(rs, c.invert_rs);
    let y = alu(a, rt, c.op_or);
    rf.write(i.rd, y, c.reg_write);

    let mn = if c.op_or { "or" } else { "and" };
    let rs_s = if c.invert_rs { format!("~t{}", i.rs) } else { format!("t{}", i.rs) };
    println!("  {} t{}, {}, t{}", mn, i.rd, rs_s, i.rt);
    println!("    ctrl:  op_or={} invert_rs={} reg_write={}", c.op_or, c.invert_rs, c.reg_write);
    println!("    regs:  t{}={} t{}={} -> t{}={}", i.rs, rs, i.rt, rt, i.rd, y);
}

fn read_bit(name: &str) -> u32 {
    loop {
        print!("{} (0/1): ", name);
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("read failed");
        match s.trim() {
            "0" => return 0,
            "1" => return 1,
            _ => println!("enter 0 or 1"),
        }
    }
}

pub fn run() {
    println!("Single-Cycle Processor - Y = A*B + C'*D\n");

    let a = read_bit("A");
    let b = read_bit("B");
    let c = read_bit("C");
    let d = read_bit("D");

    // t0=A, t1=B, t2=C, t3=D
    let mut rf = Regs([a, b, c, d, 0, 0, 0, 0]);

    // Note: PDF listing shows "and t6, t5, t3" but t5=0 would make t6 always 0;
    // corrected to t2 (C) so the program actually computes Y = A*B + C'*D.
    let program = [
        Instr { rd: 4, rs: 0, rt: 1, funct: F_AND       }, // t4 = A & B
        Instr { rd: 6, rs: 2, rt: 3, funct: F_AND | INV }, // t6 = ~C & D
        Instr { rd: 0, rs: 4, rt: 6, funct: F_OR        }, // t0 = t4 | t6
    ];

    println!("\nInitial: t0={} t1={} t2={} t3={}", a, b, c, d);
    for (n, i) in program.iter().enumerate() {
        println!("\n[cycle {}]", n + 1);
        step(&mut rf, i);
    }

    let y = rf.read(0);
    let expected = (a & b) | ((!c & 1) & d);
    println!("\nt4 = {}   t6 = {}   t0 (Y) = {}", rf.read(4), rf.read(6), y);
    println!("expected = {}   {}", expected, if y == expected { "PASS" } else { "FAIL" });
}
