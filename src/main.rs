mod memory;
mod cpu;

#[macro_use]
extern crate arrayref;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use cpu::*;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Too few arguments!");
    }
    println!("Running script {}", args[1]);

    let mut cpu = CPU::new();
    cpu.program = read_memory(&args[1]);

    cpu.run();
}

fn read_memory(path_str: &String) -> Vec<u8> {
    // Create a path to the desired file
    let path = Path::new(path_str);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut data: Vec<u8> = Vec::new();
    match file.read_to_end(&mut data) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => println!("read {} bytes from {}", data.len(), display),
    }

    data
}

