// Vincent Nguyen

use std::collections::HashSet;
use std::io::{self, Write};

// ── Implicant (Quine-McCluskey term) ─────────────────────────────────────────

#[derive(Clone, PartialEq, Eq, Hash)]
struct Implicant {
    bits: Vec<Option<u8>>, // Some(0), Some(1), None = don't-care
    terms: Vec<usize>,     // covered minterms or maxterms
}

impl Implicant {
    fn new(index: usize, n: usize) -> Self {
        let bits = (0..n)
            .map(|b| Some(((index >> (n - 1 - b)) & 1) as u8))
            .collect();
        Implicant { bits, terms: vec![index] }
    }

    fn combine(&self, other: &Self) -> Option<Self> {
        let mut diff: Option<usize> = None;
        let mut result = Vec::with_capacity(self.bits.len());

        for (i, (a, b)) in self.bits.iter().zip(&other.bits).enumerate() {
            match (a, b) {
                (None, None) => result.push(None),
                (Some(x), Some(y)) if x == y => result.push(Some(*x)),
                (Some(_), Some(_)) => {
                    if diff.is_some() { return None; } // more than one differing bit
                    diff = Some(i);
                    result.push(None);
                }
                _ => return None, // mismatched don't-care positions
            }
        }

        diff?; // return None if implicants are identical

        let mut terms = self.terms.clone();
        terms.extend_from_slice(&other.terms);
        terms.sort_unstable();
        terms.dedup();
        Some(Implicant { bits: result, terms })
    }

    fn matches(&self, index: usize, n: usize) -> bool {
        self.bits.iter().enumerate().all(|(i, b)| match b {
            None => true,
            Some(v) => ((index >> (n - 1 - i)) & 1) as u8 == *v,
        })
    }

    // SOP: Some(1) → A, Some(0) → A'
    fn to_sop_term(&self) -> String {
        const VARS: [char; 4] = ['A', 'B', 'C', 'D'];
        let mut s = String::new();
        for (i, b) in self.bits.iter().enumerate() {
            match b {
                Some(1) => s.push(VARS[i]),
                Some(0) => { s.push(VARS[i]); s.push('\''); }
                _ => {}
            }
        }
        if s.is_empty() { "1".to_string() } else { s }
    }

    // POS: Some(0) → A, Some(1) → A' (maxterm convention)
    fn to_pos_term(&self) -> String {
        const VARS: [char; 4] = ['A', 'B', 'C', 'D'];
        let mut lits: Vec<String> = Vec::new();
        for (i, b) in self.bits.iter().enumerate() {
            match b {
                Some(0) => lits.push(VARS[i].to_string()),
                Some(1) => lits.push(format!("{}'", VARS[i])),
                _ => {}
            }
        }
        if lits.is_empty() { "0".to_string() } else { format!("({})", lits.join("+")) }
    }
}

// ── Quine-McCluskey ───────────────────────────────────────────────────────────

fn find_prime_implicants(n: usize, terms: &[usize]) -> Vec<Implicant> {
    if terms.is_empty() {
        return vec![];
    }

    let mut current: Vec<Implicant> = terms.iter().map(|&m| Implicant::new(m, n)).collect();
    let mut primes: Vec<Implicant> = vec![];

    loop {
        let mut used: HashSet<usize> = HashSet::new();
        let mut next: Vec<Implicant> = vec![];
        let mut seen: HashSet<Vec<Option<u8>>> = HashSet::new();

        for i in 0..current.len() {
            for j in (i + 1)..current.len() {
                if let Some(c) = current[i].combine(&current[j]) {
                    used.insert(i);
                    used.insert(j);
                    if seen.insert(c.bits.clone()) {
                        next.push(c);
                    }
                }
            }
        }

        for (i, imp) in current.iter().enumerate() {
            if !used.contains(&i) {
                primes.push(imp.clone());
            }
        }

        if next.is_empty() { break; }
        current = next;
    }

    primes
}

fn select_cover(terms: &[usize], primes: &[Implicant]) -> Vec<Implicant> {
    let mut uncovered: HashSet<usize> = terms.iter().cloned().collect();
    let mut selected_bits: HashSet<Vec<Option<u8>>> = HashSet::new();
    let mut selected: Vec<Implicant> = vec![];

    // Essential prime implicants: terms covered by exactly one PI
    for &t in terms {
        let covering: Vec<&Implicant> = primes.iter()
            .filter(|pi| pi.terms.contains(&t))
            .collect();
        if covering.len() == 1 {
            let pi = covering[0];
            if selected_bits.insert(pi.bits.clone()) {
                for &m in &pi.terms { uncovered.remove(&m); }
                selected.push(pi.clone());
            }
        }
    }

    // Greedy cover for remaining uncovered terms
    while !uncovered.is_empty() {
        let best = primes.iter()
            .filter(|pi| !selected_bits.contains(&pi.bits))
            .max_by_key(|pi| pi.terms.iter().filter(|m| uncovered.contains(m)).count())
            .cloned();

        match best {
            Some(pi) => {
                for &m in &pi.terms { uncovered.remove(&m); }
                selected_bits.insert(pi.bits.clone());
                selected.push(pi);
            }
            None => break,
        }
    }

    selected
}

// ── K-Map display (Gray code ordering) ───────────────────────────────────────

fn print_kmap(n: usize, outputs: &[u8]) {
    println!("\nK-Map:");
    match n {
        2 => {
            println!("    | B=0  B=1");
            println!("----|----------");
            for a in 0..2usize {
                print!("A={}|", a);
                for b in 0..2usize {
                    print!("  {}   ", outputs[(a << 1) | b]);
                }
                println!();
            }
        }
        3 => {
            // Columns in Gray code: BC = 00, 01, 11, 10
            let cols: [usize; 4] = [0b00, 0b01, 0b11, 0b10];
            println!("    | BC=00  BC=01  BC=11  BC=10");
            println!("----|------------------------------");
            for a in 0..2usize {
                print!("A={}|", a);
                for &bc in &cols {
                    print!("   {}     ", outputs[(a << 2) | bc]);
                }
                println!();
            }
        }
        4 => {
            // Both axes in Gray code: 00, 01, 11, 10
            let order: [usize; 4] = [0b00, 0b01, 0b11, 0b10];
            println!("      | CD=00  CD=01  CD=11  CD=10");
            println!("------|------------------------------");
            for &ab in &order {
                print!("AB={:02b}|", ab);
                for &cd in &order {
                    print!("   {}     ", outputs[(ab << 2) | cd]);
                }
                println!();
            }
        }
        _ => {}
    }
}

// ── I/O helper ────────────────────────────────────────────────────────────────

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    buf.trim().to_string()
}

// ── Public entry point ────────────────────────────────────────────────────────

pub fn run() {
    const VAR_NAMES: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    // Section 1: Input System

    let n: usize = loop {
        match prompt("Number of input variables (n >= 2): ").parse::<usize>() {
            Ok(v) if v >= 2 => break v,
            _ => println!("Enter an integer >= 2."),
        }
    };
    let num_rows = 1usize << n;

    // Validate: 2^n rows, each combination once, outputs in {0, 1}
    println!(
        "\nEnter output column ({} values, 0 or 1), space-separated.",
        num_rows
    );

    // // Commented out for now
    // println!(
    //     "Row order: {}",
    //     (0..num_rows)
    //         .map(|i| format!("{:0>width$b}", i, width = n))
    //         .collect::<Vec<_>>()
    //         .join("  ")
    // );
    let outputs: Vec<u8> = loop {
        let vals: Vec<u8> = prompt("Outputs: ")
            .split_whitespace()
            .filter_map(|s| s.parse::<u8>().ok())
            .filter(|&v| v <= 1)
            .collect();
        if vals.len() == num_rows {
            break vals;
        }
        println!("Expected {} binary values. Got {}.", num_rows, vals.len());
    };

    let use_sop: bool = loop {
        match prompt("Select form (SOP / POS): ").to_uppercase().as_str() {
            "SOP" => break true,
            "POS" => break false,
            _ => println!("Enter SOP or POS."),
        }
    };

    // Section 1 output: Truth table
    println!("\nTruth Table:");
    let header: String = (0..n).map(|i| format!(" {}", VAR_NAMES[i])).collect();
    println!("{} | F", header);
    println!("{}", "-".repeat(header.len() + 4));
    for i in 0..num_rows {
        let row: String = (0..n)
            .map(|b| format!(" {}", (i >> (n - 1 - b)) & 1))
            .collect();
        println!("{} | {}", row, outputs[i]);
    }

    // Section 2: Boolean Expression & Simplification

    let minterms: Vec<usize> = (0..num_rows).filter(|&i| outputs[i] == 1).collect();
    let maxterms: Vec<usize> = (0..num_rows).filter(|&i| outputs[i] == 0).collect();
    let terms: &[usize] = if use_sop { &minterms } else { &maxterms };

    // Canonical equation (SOP or POS)
    let canonical = if use_sop {
        if minterms.is_empty() {
            "0".to_string()
        } else {
            minterms.iter().map(|&m| {
                (0..n).map(|b| {
                    if (m >> (n - 1 - b)) & 1 == 1 { VAR_NAMES[b].to_string() }
                    else { format!("{}'", VAR_NAMES[b]) }
                }).collect::<String>()
            }).collect::<Vec<_>>().join(" + ")
        }
    } else {
        if maxterms.is_empty() {
            "1".to_string()
        } else {
            maxterms.iter().map(|&m| {
                let lits: Vec<String> = (0..n).map(|b| {
                    if (m >> (n - 1 - b)) & 1 == 0 { VAR_NAMES[b].to_string() }
                    else { format!("{}'", VAR_NAMES[b]) }
                }).collect();
                format!("({})", lits.join("+"))
            }).collect::<Vec<_>>().join(" · ")
        }
    };

    println!("\nCanonical {}: {}", if use_sop { "SOP" } else { "POS" }, canonical);
    println!("{}s: {:?}", if use_sop { "Minterm" } else { "Maxterm" }, terms);

    // K-Map + Quine-McCluskey simplification (2-4 variables)
    if n <= 4 {
        print_kmap(n, &outputs);

        let primes = find_prime_implicants(n, terms);
        let cover = select_cover(terms, &primes);

        // K-Map groupings
        println!("\nK-Map Groups:");
        if cover.is_empty() {
            println!("  (none)");
        } else {
            for (i, pi) in cover.iter().enumerate() {
                let expr = if use_sop { pi.to_sop_term() } else { pi.to_pos_term() };
                println!("  Group {}: {:?} → {}", i + 1, pi.terms, expr);
            }
        }

        // Simplified Boolean expression
        let simplified = if cover.is_empty() {
            if use_sop { "0".to_string() } else { "1".to_string() }
        } else if use_sop {
            cover.iter().map(|pi| pi.to_sop_term()).collect::<Vec<_>>().join(" + ")
        } else {
            cover.iter().map(|pi| pi.to_pos_term()).collect::<Vec<_>>().join(" · ")
        };
        println!("\nSimplified {}: {}", if use_sop { "SOP" } else { "POS" }, simplified);

        // Section 3: Validation — evaluate simplified expression against original table
        let pass = (0..num_rows).all(|i| {
            let covered = cover.iter().any(|pi| pi.matches(i, n));
            let predicted = if use_sop {
                if covered { 1u8 } else { 0u8 }
            } else {
                if covered { 0u8 } else { 1u8 } // POS: covered → this is a maxterm → output 0
            };
            outputs[i] == predicted
        });
        println!("\nValidation: {}", if pass { "PASS" } else { "FAIL" });
    } else {
        println!("\nNote: K-Map simplification is supported for 2-4 variables. Canonical form shown above.");
    }
}