extern crate core;

use std::io::File;
use std::io;
use self::brainfuck::tape;
use self::brainfuck::program;

mod brainfuck;

fn main() {
    let args = std::os::args().clone();

    if args.len() != 2 {
        fail!("Usage: brainfuck <file>");
    }

    let path_string = std::os::args()[1].clone();
    let path = Path::new(path_string);

    let mut program = program::Program::new(&path);
    let mut tape = tape::Tape::new();

    let mut reader = io::stdin();

    while program.pointer != program.len() as uint {
        match program.command() {
            '+' => tape.inc(),
            '-' => tape.dec(),
            '>' => tape.next(),
            '<' => tape.prev(),
            '.' => print!("{}", tape.char_value()),
            ',' => {
                tape.set_value(reader.read_char().ok().unwrap() as u8);
            },
            '[' => {
                if tape.value() == 0 {
                    program.fast_forward(1);
                }
            },
            ']' => {
                if tape.value() != 0 {
                    program.rewind(1);
                }
            },
            _ => ()
        }

        program.forward();
    }
}
