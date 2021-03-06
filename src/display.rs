pub struct Display {
    size: (usize, usize),
    d: [[bool; 128]; 64],
    dirty: bool
}

impl Display {
    pub fn new(big: bool) -> Display {
        Display{
            d: [[false; 128]; 64],
            size: {
                if big {(128, 64)} else {(64, 32)}
            },
            dirty: false
        }
    }

    pub fn clear(&mut self) {
        self.d = [[false; 128]; 64]
    }
    
    pub fn write(&mut self, b: u8, x: usize, y: usize) -> bool {
        let y = y % self.size.1;

        let mut erased = false;

        self.dirty = true;

        let mut l = [false; 8];
        for i in 0..7 {
            l[i] = if (b & (0b10000000 >> i)) >> 7-i == 1 {true} else {false}
        };

        for (i, e) in l.iter().enumerate() {
            let dx = (x+i)%self.size.1;
            if self.d[y][dx] & e {
                erased = true
            }
            self.d[y][dx] ^= *e
        };
        erased
    }

    pub fn read(&mut self) -> &[[bool; 128]; 64] {
        self.dirty = false;
        return &self.d
    }

    pub fn size(&self) -> (usize, usize) {
        return self.size
    }
    
    pub fn dirty(&self) -> bool {
        return self.dirty
    }
}

pub const DEFAULT_FONT: [[u8; 5]; 16] = [
    [0b11110000, 0b10010000, 0b10010000, 0b10010000, 0b11110000], // 0
    [0b00100000, 0b01100000, 0b00100000, 0b00100000, 0b01110000], // 1
    [0b11110000, 0b00010000, 0b11110000, 0b10000000, 0b11110000], // 2
    [0b11110000, 0b00010000, 0b11110000, 0b00010000, 0b11110000], // 3
    [0b10010000, 0b10010000, 0b11110000, 0b00010000, 0b00010000], // 4
    [0b11110000, 0b10000000, 0b11110000, 0b00010000, 0b11110000], // 5
    [0b11110000, 0b10000000, 0b11110000, 0b10010000, 0b11110000], // 6
    [0b11110000, 0b00010000, 0b00100000, 0b01000000, 0b01000000], // 7
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b11110000], // 8
    [0b11110000, 0b10010000, 0b11110000, 0b00010000, 0b11110000], // 9
    [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b10010000], // A
    [0b11100000, 0b10010000, 0b11100000, 0b10010000, 0b11100000], // B
    [0b11110000, 0b10000000, 0b10000000, 0b10000000, 0b11110000], // C
    [0b11100000, 0b10010000, 0b10010000, 0b10010000, 0b11100000], // D
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b11110000], // E
    [0b11110000, 0b10000000, 0b11110000, 0b10000000, 0b10000000], // F
];
