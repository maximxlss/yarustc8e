pub struct State {
    ram: [u8; 4096],  // 4 KB RAM
    reg: [u8; 16],    // General-purpose 8-bit registers
    pc: u16,          // Program counter
    dt: u8,           // Delay timer
    st: u8,           // Sound timer
    i: u16,           // I-register
    sp: u8,           // Stack pointer
    stack: [u16; 16], // Stack
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
