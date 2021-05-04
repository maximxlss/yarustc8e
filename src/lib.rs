#![allow(dead_code)]

mod display;
mod state;

use rand::Rng;

pub struct Chip8<'a> {
    state: state::State,
    pub display: display::Display,
    font: [[u8; 5]; 16],
    key_state_handler: &'a dyn Fn(u8) -> bool,
    key_wait_handler: &'a dyn Fn() -> u8
}

#[rustfmt::skip]
impl Chip8<'_> {
    pub fn new<'a, F, G>(start_from: usize, big: bool, font: Option<[[u8; 5]; 16]>, key_state_handler: &'a F, key_wait_handler: &'a G) -> Chip8<'a>
    where F: Fn(u8) -> bool, G: Fn() -> u8
    {
        Chip8 {
            state: state::State::new(start_from, match font {
                Some(font) => font,
                None => display::DEFAULT_FONT,
            }),
            display: display::Display::new(big),
            font: match font {
                Some(font) => font,
                None => display::DEFAULT_FONT,
            },
            key_state_handler,
            key_wait_handler
        }
    }

    pub fn load(&mut self, program: Vec<u8>, to: Option<usize>) {
        let to = match to {
            None => self.state.pc,
            Some(v) => v
        };
        for (i, b) in program.iter().enumerate() {
            self.state.ram[to+i] = *b;
        }
    }

    pub fn internal_state(&mut self) -> &mut state::State {
        &mut self.state
    }

    pub fn timer_step(&mut self) {
        if self.state.dt > 0 {
            self.state.dt -= 1
        }
        if self.state.st > 0 {
            self.state.st -= 1
        }
    }

    pub fn stack_push(&mut self, n: usize) {
        self.state.sp += 1;
        self.state.stack[self.state.sp] = n;
    }

    pub fn stack_pop(&mut self) -> usize {
        self.state.sp -= 1;
        return self.state.stack[self.state.sp + 1];
    }

    pub fn evolve(&mut self) -> Result<(), &'static str> {
        let instruction = (self.state.ram[self.state.pc] as usize) << 8 | self.state.ram[self.state.pc+1] as usize;

        println!("{:04x?}${:04x?}", &self.state.pc, &instruction);

        let nnn = ||  instruction & 0x0FFF;
        let kk  = || (instruction & 0x00FF) as u8;
        let n   = ||  instruction & 0x000F;
        let x   = || (instruction & 0x0F00) >> 8;
        let y   = || (instruction & 0x00F0) >> 4;

        match instruction & 0xF000 {
            0x0000 => {
                match instruction {
                    /*CLS*/ 0x00E0 => self.display.clear(),
                    /*RET*/ 0x00EE => self.state.pc = self.stack_pop() - 2,
                    /*SYS*/ _ => {}
                }
            },
            0x1000 => /*JP*/ self.state.pc = nnn() as usize - 2,
            0x2000 => /*CALL*/ {
                self.stack_push(self.state.pc);
                self.state.pc = nnn() as usize - 2;
            },
            0x3000 => /*SE*/ {
                if self.state.reg[x() as usize] == kk() {
                    self.state.pc += 2
                }
            },
            0x4000 => /*SNE*/ {
                if self.state.reg[x() as usize] != kk() as u8 {
                    self.state.pc += 2
                }
            },
            0x5000 => {
                match instruction & 0xF00F {
                    0x5000 => { /*SE*/
                        if self.state.reg[x() as usize] == self.state.reg[y() as usize] {
                            self.state.pc += 2
                        }
                    }
                    _ => return Err("Invalid instruction")
                }
            },
            0x6000 => /*LD*/ self.state.reg[x() as usize] = kk(),
            0x7000 => /*ADD*/ self.state.reg[x() as usize] += kk(),
            0x8000 => {
                match instruction & 0x000F {
                    0 => /*LD*/ self.state.reg[x() as usize] = self.state.reg[y() as usize],
                    1 => /*OR*/ self.state.reg[x() as usize] |= self.state.reg[y() as usize],
                    2 => /*AND*/ self.state.reg[x() as usize] &= self.state.reg[y() as usize],
                    3 => /*XOR*/ self.state.reg[x() as usize] ^= self.state.reg[y() as usize],
                    4 => /*ADD*/ {
                        let (r, carry) = self.state.reg[x() as usize].overflowing_add(self.state.reg[y() as usize]);
                        self.state.reg[0xF] = carry as u8;
                        self.state.reg[x() as usize] = r;
                    },
                    5 => /*SUB*/ {
                        let (r, borrow) = self.state.reg[x() as usize].overflowing_sub(self.state.reg[y() as usize]);
                        self.state.reg[0xF] = !borrow as u8;
                        self.state.reg[x() as usize] = r;
                    },
                    6 => /*SHR*/ {
                        let (r, carry) = self.state.reg[x() as usize].overflowing_shr(self.state.reg[y() as usize] as u32);
                        self.state.reg[0xF] = carry as u8;
                        self.state.reg[x() as usize] = r;
                    },
                    7 => /*SUBN*/ {
                        let (r, borrow) = self.state.reg[y() as usize].overflowing_sub(self.state.reg[x() as usize]);
                        self.state.reg[0xF] = !borrow as u8;
                        self.state.reg[x() as usize] = r;
                    },
                    0xE => /*SHL*/ {
                        let (r, carry) = self.state.reg[y() as usize].overflowing_shl(self.state.reg[x() as usize] as u32);
                        self.state.reg[0xF] = !carry as u8;
                        self.state.reg[x() as usize] = r;
                    },
                    _ => return Err("Invalid instruction")
                }
            },
            0x9000 => {
                match instruction & 0x000F {
                    0x0000 => { /*SNE*/
                        if self.state.reg[x() as usize] != self.state.reg[y() as usize] {
                            self.state.pc += 2
                        }
                    }
                    _ => return Err("Invalid instruction")
                }
            },
            0xA000 => { /*LD*/
                self.state.i = nnn() as usize
            },
            0xB000 => { /*JP*/
                self.state.pc = self.state.reg[0] as usize + nnn() - 2 as usize
            },
            0xC000 => { /*RND*/
                let mut rng = rand::thread_rng();
                self.state.reg[x() as usize] = rng.gen::<u8>() & kk();
            },
            0xD000 => { /*DRW*/
                let x = self.state.reg[x() as usize] as usize;
                let y = self.state.reg[y() as usize] as usize;

                for (j, e) in self.state.ram[self.state.i as usize .. self.state.i as usize + n() as usize].iter().enumerate() {
                    self.display.write(*e, x, y+j);
                }
            },
            0xE000 => {
                match instruction & 0xF0FF {
                    0xE09E => { /*SKP*/
                        if (self.key_state_handler)(x() as u8) {
                            self.state.pc += 2
                        }
                    },
                    0xE0A1 => { /*SKNP*/
                        if !(self.key_state_handler)(x() as u8) {
                            self.state.pc += 2
                        }
                    },
                    _ => return Err("Invalid instruction")
                }
            },
            0xF000 => {
                match instruction & 0xF0FF {
                    0xF007 => /*Timer*/ self.state.reg[x() as usize] = self.state.dt,
                    0xF00A => /*Key*/ self.state.reg[x() as usize] = (self.key_wait_handler)(),
                    0xF015 => /*Timer*/ self.state.dt = self.state.reg[x() as usize],
                    0xF018 => /*Sound*/ self.state.st = self.state.reg[x() as usize],
                    0xF01E => /*ADD*/ self.state.i += self.state.reg[x() as usize] as usize,
                    0xF029 => /*CHR*/ self.state.i = self.state.reg[x() as usize] as usize * 5,
                    0xF033 => { /*BCD*/ 
                        let vx = self.state.reg[x() as usize];
                        self.state.ram[self.state.i] = vx / 100;
                        self.state.ram[self.state.i] = vx %100 /10;
                        self.state.ram[self.state.i] = vx %100 %10;
                    },
                    0xF055 => { /*SAVE*/
                        for j in 0..x() as usize {
                            self.state.ram[self.state.i + j] = self.state.reg[j]
                        }
                    },
                    0xF065 => { /*LOAD*/
                        for j in 0..x() as usize {
                            self.state.reg[j] = self.state.ram[self.state.i + j]
                        }
                    },
                    _ => return Err("Invalid instruction")
                }
            },
            _ => unreachable!()
        }
        self.state.pc += 2;
        Ok(())
    }
}
