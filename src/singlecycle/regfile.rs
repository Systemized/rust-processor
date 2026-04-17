// Vincent Nguyen

pub struct Regs(pub [u32; 8]);

impl Regs {
    pub fn read(&self, i: usize) -> u32 { self.0[i] }
    pub fn write(&mut self, i: usize, v: u32, en: bool) { if en { self.0[i] = v; } }
}
