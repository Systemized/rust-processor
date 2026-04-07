// Vincent Nguyen

use std::collections::VecDeque;
use std::io::{self, Write};

// ── Student-defined latencies (cycles to read OUT of each level) ──────────────
//   SSD=100, DRAM=50, L3=10, L2=5, L1=1
const LATENCY: [u32; 5] = [100, 50, 10, 5, 1];
const NAMES:   [&str; 5] = ["SSD", "DRAM", "L3", "L2", "L1"];

// ── Memory level ──────────────────────────────────────────────────────────────

struct MemLevel {
    capacity: usize,
    data:     VecDeque<u32>, // FIFO
}

impl MemLevel {
    fn new(capacity: usize) -> Self {
        MemLevel { capacity, data: VecDeque::new() }
    }

    fn contains(&self, val: u32) -> bool {
        self.data.contains(&val)
    }

    // Insert val. If full, evict oldest and return it.
    fn insert(&mut self, val: u32) -> Option<u32> {
        let evicted = if self.data.len() >= self.capacity {
            self.data.pop_front()
        } else {
            None
        };
        self.data.push_back(val);
        evicted
    }

    fn remove(&mut self, val: u32) {
        if let Some(pos) = self.data.iter().position(|&x| x == val) {
            self.data.remove(pos);
        }
    }
}

// ── Memory system ─────────────────────────────────────────────────────────────

struct MemSystem {
    levels: [MemLevel; 5], // 0=SSD, 1=DRAM, 2=L3, 3=L2, 4=L1
    clock:  u32,
    hits:   u32,
    misses: u32,
}

impl MemSystem {
    fn new(sizes: [usize; 5]) -> Self {
        MemSystem {
            levels: [
                MemLevel::new(sizes[0]),
                MemLevel::new(sizes[1]),
                MemLevel::new(sizes[2]),
                MemLevel::new(sizes[3]),
                MemLevel::new(sizes[4]),
            ],
            clock:  0,
            hits:   0,
            misses: 0,
        }
    }

    // Output 1: configuration
    fn print_config(&self) {
        println!("\n=== Memory Hierarchy Configuration ===");
        for i in 0..5 {
            println!(
                "  {:<5}  capacity: {:>4} instructions  latency: {} cycles",
                NAMES[i], self.levels[i].capacity, LATENCY[i]
            );
        }
        println!("  Data flow enforced: SSD → DRAM → L3 → L2 → L1 → CPU");
    }

    // Output 5: final state
    fn print_state(&self) {
        println!("\n=== Memory State (clock: {}) ===", self.clock);
        for i in 0..5 {
            let lvl = &self.levels[i];
            println!(
                "  {:<5} [{}/{}]: {:?}",
                NAMES[i], lvl.data.len(), lvl.capacity,
                lvl.data.iter().map(|v| format!("{:#010x}", v)).collect::<Vec<_>>()
            );
        }
        println!("  Cache hits: {}  misses: {}", self.hits, self.misses);
    }

    // Section 4 / 6: READ — enforces SSD→DRAM→L3→L2→L1, no bypassing
    // Output 2 (access trace) + Output 3 (movement)
    fn read(&mut self, val: u32) {
        print!("\n[CLK {:>4}] READ {:#010x}  →  ", self.clock, val);

        // Section 5: cache hit check (L1 only)
        if self.levels[4].contains(val) {
            println!("L1 HIT");
            self.hits += 1;
            return;
        }

        // Cache miss — find which level holds it
        let found = (0..4).rev().find(|&i| self.levels[i].contains(val));

        let src = match found {
            Some(i) => {
                println!("L1 MISS — found in {}", NAMES[i]);
                self.misses += 1;
                i
            }
            None => {
                println!("L1 MISS — not in hierarchy");
                self.misses += 1;
                return;
            }
        };

        // Section 6: move level-by-level from src up to L1 (no skipping)
        for lvl_i in src..4 {
            let next = lvl_i + 1;
            self.levels[lvl_i].remove(val);
            let evicted = self.levels[next].insert(val);
            self.clock += LATENCY[lvl_i];

            print!("  [CLK {:>4}] {} → {}", self.clock, NAMES[lvl_i], NAMES[next]);

            // Section 5: eviction — FIFO, evicted value written back one level down
            if let Some(ev) = evicted {
                println!("  (evicted {:#010x} back to {})", ev, NAMES[lvl_i]);
                self.levels[lvl_i].data.push_back(ev);
            } else {
                println!();
            }
        }
    }

    // Section 4 / 6: WRITE — write to L1, cascade evictions downward
    // Output 2 (access trace) + Output 3 (movement)
    fn write(&mut self, val: u32) {
        println!("\n[CLK {:>4}] WRITE {:#010x} → L1", self.clock, val);

        // Insert into L1, cascade evictions from L1 (4) down to SSD (0)
        let mut pending = val;
        for lvl_i in (0..5).rev() {
            let evicted = self.levels[lvl_i].insert(pending);
            self.clock += LATENCY[lvl_i];
            println!("  [CLK {:>4}] {:#010x} written to {}", self.clock, pending, NAMES[lvl_i]);

            match evicted {
                Some(ev) if lvl_i > 0 => {
                    println!(
                        "  [CLK {:>4}] {} full — evicting {:#010x} to {}",
                        self.clock, NAMES[lvl_i], ev, NAMES[lvl_i - 1]
                    );
                    pending = ev;
                }
                _ => break,
            }
        }
    }

    // Preload SSD only — enforces hierarchy (data starts at bottom)
    fn preload(&mut self, instructions: &[u32]) {
        for &instr in instructions {
            if self.levels[0].data.len() < self.levels[0].capacity {
                self.levels[0].data.push_back(instr);
            }
        }
        println!("Preloaded {} instruction(s) into SSD.", self.levels[0].data.len());
    }
}

// ── I/O helpers ───────────────────────────────────────────────────────────────

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    buf.trim().to_string()
}

fn parse_hex(s: &str) -> Option<u32> {
    u32::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16).ok()
}

// ── Entry point ───────────────────────────────────────────────────────────────

pub fn run() {
    println!("=== Memory Hierarchy Simulation ===");
    println!("Sizes in number of 32-bit instructions.");
    println!("Constraint enforced: SSD > DRAM > L3 > L2 > L1 >= 1\n");

    // Section 2: configurable sizes with hierarchy constraint
    let sizes: [usize; 5] = loop {
        let ssd:  usize = prompt("SSD  capacity: ").parse().unwrap_or(0);
        let dram: usize = prompt("DRAM capacity: ").parse().unwrap_or(0);
        let l3:   usize = prompt("L3   capacity: ").parse().unwrap_or(0);
        let l2:   usize = prompt("L2   capacity: ").parse().unwrap_or(0);
        let l1:   usize = prompt("L1   capacity: ").parse().unwrap_or(0);

        if ssd > dram && dram > l3 && l3 > l2 && l2 > l1 && l1 >= 1 {
            break [ssd, dram, l3, l2, l1];
        }
        println!("  Error: must satisfy SSD > DRAM > L3 > L2 > L1 >= 1. Retry.\n");
    };

    let mut sys = MemSystem::new(sizes);

    // Preload SSD with sequential 32-bit instruction words
    let count: usize = prompt("Instructions to preload into SSD (0 to skip): ")
        .parse()
        .unwrap_or(0);

    if count > 0 {
        let instructions: Vec<u32> = (0..count).map(|i| (i as u32) * 4).collect();
        sys.preload(&instructions);
    }

    // Output 1
    sys.print_config();

    // Section 3/4: interactive clock-driven command loop
    println!("\nCommands:");
    println!("  READ  <hex>   — fetch instruction (SSD→DRAM→L3→L2→L1)");
    println!("  WRITE <hex>   — write instruction to L1 (evictions cascade down)");
    println!("  STATE         — print current memory state");
    println!("  QUIT          — exit and print final state");
    println!("  Example: READ 0x00000000\n");

    loop {
        let line = prompt("> ");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0].to_uppercase().as_str() {
            "READ" => {
                if parts.len() < 2 {
                    println!("Usage: READ <hex>");
                    continue;
                }
                match parse_hex(parts[1]) {
                    Some(v) => sys.read(v),
                    None    => println!("Invalid hex."),
                }
            }
            "WRITE" => {
                if parts.len() < 2 {
                    println!("Usage: WRITE <hex>");
                    continue;
                }
                match parse_hex(parts[1]) {
                    Some(v) => sys.write(v),
                    None    => println!("Invalid hex."),
                }
            }
            "STATE" => sys.print_state(),
            "QUIT"  => break,
            _       => println!("Unknown command."),
        }
    }

    sys.print_state();
}