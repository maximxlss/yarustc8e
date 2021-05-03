pub struct State {
    pub ram: [u8; 4096],  // 4 KB RAM
    pub reg: [u8; 16],    // General-purpose 8-bit registers
    pub pc: u16,          // Program counter
    pub dt: u8,           // Delay timer
    pub st: u8,           // Sound timer
    pub i: u16,           // I-register
    pub sp: u8,           // Stack pointer
    pub stack: [u16; 16], // Stack
}

impl State {
    pub fn new(start_from: u16) -> State {
        State {
            ram: [0; 4096],
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
