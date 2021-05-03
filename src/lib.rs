#![allow(dead_code)]
mod state;
mod display;

pub struct Chip8 {
    state: state::State,
    display: display::Display,
    font: [[u8; 5]; 16],
}

impl Chip8 {
    pub fn new(start_from: u16, big: bool, font: Option<[[u8; 5]; 16]>) -> Chip8 {
        Chip8 {
            state: state::State::new(start_from),
            display: {
                if big {
                    display::Display::Big([[false; 128]; 64])
                } else {
                    display::Display::Default([[false; 64]; 32])
                }
            },
            font: match font {
                Some(font) => font,
                None => display::DEFAULT_FONT,
            },
        }
    }
}
