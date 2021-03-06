extern crate core;

use std::io;

use self::brainfuck::tape;
use self::brainfuck::program;

mod brainfuck;

fn main() {
    let path = get_path();

    let mut program = program::Program::new(&path);
    let mut tape = tape::Tape::new();

    let mut reader = io::stdin();

    while program.pointer != program.len() as usize {
        match program.command() {
            '+' => tape.inc(),
            '-' => tape.dec(),
            '>' => tape.next(),
            '<' => tape.prev(),
            '.' => print!("{}", tape.value() as char),
            ',' => tape.read_value(&mut reader),
            '[' if tape.zero() => program.fast_forward(1),
            ']' if tape.not_zero() => program.rewind(1),
            _ => ()
        }

        program.forward();
    }
}

fn get_path() -> Path {
    let args = std::os::args().clone();

    if args.len() != 2 {
        panic!("Usage: brainfuck <file>");
    }

    let path_string = std::os::args()[1].clone();
    Path::new(path_string)
}
