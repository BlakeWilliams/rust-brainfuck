extern crate core;

use core::iter::Repeat;
use std::io::File;
use std::io;

fn main() {
    let args = std::os::args().clone();

    if args.len() != 2 {
        fail!("Usage: brainfuck <file>");
    }

    let path_string = std::os::args()[1].clone();
    let path = Path::new(path_string);

    let file = match File::open(&path) {
        Ok(mut f) => f.read_to_string().ok().unwrap(),
        Err(e) => fail!("Could not open file: {}", e)
    };

    let program: Vec<char> = file.as_slice().chars().collect();
    let mut program_pointer = 0u;

    let mut tape: Vec<u8> = Repeat::new(0u8).take(30_000u).collect();
    let mut tape_pointer = 0u;

    let mut reader = io::stdin();

    while program_pointer != program.len() as uint {
        match program[program_pointer] {
            '+' => *tape.get_mut(tape_pointer) += 1u8,
            '-' => *tape.get_mut(tape_pointer) -= 1u8,
            '>' => tape_pointer += 1u,
            '<' => tape_pointer -= 1u,
            '.' => print!("{}", *tape.get(tape_pointer) as char),
            ',' => {
                *tape.get_mut(tape_pointer) = reader.read_char().ok().unwrap() as u8
            },
            '[' => {
                if tape[tape_pointer] == 0 {
                    program_pointer = fast_forward(&program, program_pointer, 1);
                }
            },
            ']' => {
                if tape[tape_pointer] != 0 {
                    program_pointer = rewind(&program, program_pointer, 1);
                }
            },
            _ => ()
        }

        program_pointer += 1;
    }
}

fn fast_forward(program: &Vec<char>, program_pointer: uint, count: uint) -> uint {
    let pointer = program_pointer + 1;
    let command = program[pointer];

    if count == 0 {
        pointer - 1
    } else {
        match command {
            ']' => fast_forward(program, pointer, count - 1),
            '[' => fast_forward(program, pointer, count + 1),
             _  => fast_forward(program, pointer, count)
        }
    }
}

fn rewind(program: &Vec<char>, program_pointer: uint, count: uint) -> uint {
    let pointer = program_pointer - 1;
    let command = program[pointer];

    if count == 0 {
        pointer + 1
    } else {
        match command {
            ']' => rewind(program, pointer, count + 1),
            '[' => rewind(program, pointer, count - 1),
             _  => rewind(program, pointer, count)
        }
    }
}
