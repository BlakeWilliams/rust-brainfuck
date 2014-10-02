use core::iter::Repeat;
use std::io::BufferedReader;
use std::io::stdio::StdReader;

pub struct Tape {
    content: Vec<u8>,
    pointer: uint
}

impl Tape {
    pub fn new() -> Tape {
        Tape {
            content: Repeat::new(0u8).take(30_000u).collect(),
            pointer: 0,
        }
    }

    pub fn inc(&mut self) {
        *self.content.get_mut(self.pointer) += 1u8;
    }

    pub fn dec(&mut self) {
        *self.content.get_mut(self.pointer) -= 1u8;
    }

    pub fn next(&mut self) {
        self.pointer += 1;
    }

    pub fn prev(&mut self) {
        self.pointer -= 1;
    }

    pub fn char_value(&mut self) -> char {
        self.content[self.pointer] as char
    }

    pub fn value(&mut self) -> u8 {
        self.content[self.pointer]
    }


    pub fn read_value(&mut self, reader: &mut BufferedReader<StdReader>) {
        let value = reader.read_char().ok().unwrap() as u8;
        self.set_value(value)
    }

    fn set_value(&mut self, value: u8) {
        *self.content.get_mut(self.pointer) = value;
    }
}
