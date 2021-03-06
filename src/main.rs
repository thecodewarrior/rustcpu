mod cpu;
mod libs;
mod memory;
mod number_type;

#[macro_use] extern crate arrayref;
#[macro_use] extern crate failure;
extern crate rle_vec;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::cpu::*;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Too few arguments!");
    }
    println!("Running script {}", args[1]);

    let mut cpu = CPU::new();
    let program = read_memory(&args[1]);
    for byte in program {
        cpu.program.data.push(byte);
    }

    cpu.run();
}

fn read_memory(path_str: &String) -> Vec<u8> {
    // Create a path to the desired file
    let path = Path::new(path_str);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    let mut data: Vec<u8> = Vec::new();
    match file.read_to_end(&mut data) {
        Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
        Ok(_) => println!("read {} bytes from {}", data.len(), display),
    }

    data
}
