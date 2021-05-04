pub struct State {
    pub ram: [u8; 4096],  // 4 KB RAM
    pub reg: [u8; 16],    // General-purpose 8-bit registers
    pub pc: usize,        // Program counter. Should be u16, but it's mostly an index anyway
    pub dt: u8,           // Delay timer
    pub st: u8,           // Sound timer
    pub i: usize,         // I-register. Should be u16, but it's mostly an index anyway
    pub sp: usize,        // Stack pointer. Should be u8, but it's only an index anyway
    pub stack: [usize; 16],// Stack. Should be u16, but it's mostly an index anyway
}

impl State {
    pub fn new(start_from: usize, font: [[u8; 5]; 16]) -> State {
        let mut ram = [0; 4096];

        let mut i = 0;
        for letter in &font {
            for b in letter {
                ram[i] = *b;
                i += 1;
            }
        }

        State {
            ram,
            reg: [0; 16],
            pc: start_from,
            dt: 0,
            st: 0,
            i: 0,
            sp: 0,
            stack: [0; 16],
        }
    }
}
