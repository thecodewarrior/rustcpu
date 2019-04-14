use std::{time, thread};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use crate::memory::*;
use std::num::Wrapping;
use rle_vec::RleVec;

pub struct CPU {
    pub program: Vec<u8>,
    pub memory: RleVec<u8>,
    program_counter: usize,
    registers: [u32; 40]
}

impl CPU {
    pub fn new() -> CPU {
        let mut mem: RleVec<u8> = RleVec::new();
        mem.push_n(2048, 0);
        CPU {
            program: Vec::new(),
            memory: mem,
            program_counter: 0,
            registers: [0; 40]
        }
    }

    pub fn run(&mut self) {
        println!("Booting up...");
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        let insn = to_u16_be(array_ref![self.program, self.program_counter, 2]);
//        println!("counter: {}, 0x{:02x}", self.program_counter, insn);
        self.program_counter += 2;
        match insn {
            // instructions
            0x0000 => self.insn_nop(),

            // load: 0x010x
            0x0100 => self.insn_load_const_u8(),
            0x0101 => self.insn_load_const_u16(),
            0x0102 => self.insn_load_const_u32(),
            0x0103 => self.insn_load_const_mem_u8(),
            0x0104 => self.insn_load_const_mem_u16(),
            0x0105 => self.insn_load_const_mem_u32(),
            0x0106 => self.insn_load_const_mem_blob(),
            0x0107 => self.insn_load_const_mem_str(),

            // math: 0x02xx
            0x0200 => self.insn_calc(),
            0x0201 => self.insn_calc_signed(),

            // branching: 0x03xx
            0x0300 => self.insn_jump(),
            0x0301 => self.insn_jump_cmpzero(),
            0x0302 => self.insn_jump_cmpzero_signed(),
            0x0303 => self.insn_jump_calc(),
            0x0304 => self.insn_jump_calc_signed(),
            0x0305 => self.insn_jump_cmp(),
            0x0306 => self.insn_jump_cmp_signed(),

            // debug
            0x0f00 => self.insn_debug(),
            0x0f01 => self.insn_debug_reg(),

            // shutdown conditions
            0xffff => {
                println!("Halting...");
                return false
            }
            _ => {
                self.program_counter -= 2; // return the PC to its original location
                panic!("Unknown instruction 0x{:02x} at 0x{:04x}", insn, self.program_counter)
            }
        }
        thread::sleep(time::Duration::from_millis(250));
        true
    }

    fn insn_nop(&mut self) {

    }

    // load

    fn insn_load_const_u8(&mut self) {
        let reg = self.program[self.program_counter];
        self.program_counter += 1;
        assert!((reg as usize) < self.registers.len(), "Invalid register {}", reg);

        let value = self.program[self.program_counter];
        self.program_counter += 1;

        self.registers[reg as usize] = value as u32;
    }

    fn insn_load_const_u16(&mut self) {
        let reg = self.program[self.program_counter];
        self.program_counter += 1;
        assert!((reg as usize) < self.registers.len(), "Invalid register {}", reg);

        let value = to_u16_be(array_ref![self.program, self.program_counter, 2]);
        self.program_counter += 2;

        self.registers[reg as usize] = value as u32;
    }

    fn insn_load_const_u32(&mut self) {
        let reg = self.program[self.program_counter];
        self.program_counter += 1;
        assert!((reg as usize) < self.registers.len(), "Invalid register {}", reg);

        let value = to_u32_be(array_ref![self.program, self.program_counter, 4]);
        self.program_counter += 4;

        self.registers[reg as usize] = value as u32;
    }

    fn insn_load_const_mem_u8(&mut self) {
        let addr = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;

        let value = self.program[self.program_counter];
        self.program_counter += 1;

        self.memory.set(addr, value);
    }

    fn insn_load_const_mem_u16(&mut self) {
        let addr = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;

        self.memory.set(addr, self.program[self.program_counter]);
        self.memory.set(addr+1, self.program[self.program_counter+1]);

        self.program_counter += 2;
    }

    fn insn_load_const_mem_u32(&mut self) {
        let addr = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;

        self.memory.set(addr, self.program[self.program_counter]);
        self.memory.set(addr+1, self.program[self.program_counter+1]);
        self.memory.set(addr+2, self.program[self.program_counter+2]);
        self.memory.set(addr+3, self.program[self.program_counter+3]);

        self.program_counter += 4;
    }

    fn insn_load_const_mem_blob(&mut self) {
        let addr = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;
        let loc = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;
        let len = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;

        for i in 0..len {
            self.memory.set(addr + i, self.program[loc + i]);
        }
    }

    fn insn_load_const_mem_str(&mut self) {
        let addr = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;

        for i in 0.. {
            let value = self.program[self.program_counter + i];
            self.memory.set(addr + i, value);
            if value == 0 {
                break;
            }
        }
    }

    // math

    fn insn_calc(&mut self) {
        let out_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((out_reg as usize) < self.registers.len(), "Invalid register {}", out_reg);

        self.registers[out_reg] = self.calc_unsigned();
    }

    fn calc_unsigned(&mut self) -> u32 {
        let op = self.program[self.program_counter];
        self.program_counter += 1;
        let lhs_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((lhs_reg as usize) < self.registers.len(), "Invalid register {}", lhs_reg);

        let lhs = self.registers[lhs_reg];

        match op {
            22 => -(lhs as i32) as u32,
            23 => !lhs,
            24 => bool_to_u32(!u32_to_bool(lhs)),
            _ => {
                let rhs_reg = self.program[self.program_counter] as usize;
                self.program_counter += 1;
                assert!((rhs_reg as usize) < self.registers.len(), "Invalid register {}", rhs_reg);

                let rhs = self.registers[rhs_reg];

                match op {
                    0 => (Wrapping(lhs) + Wrapping(rhs)).0,
                    1 => (Wrapping(lhs) - Wrapping(rhs)).0,
                    2 => (Wrapping(lhs) * Wrapping(rhs)).0,
                    3 => lhs / rhs,
                    4 => lhs % rhs,
                    6 => lhs << rhs,
                    7 => lhs >> rhs,
                    8 => lhs.rotate_left(rhs),
                    9 => lhs.rotate_right(rhs),
                    10 => lhs & rhs,
                    11 => lhs | rhs,
                    12 => lhs ^ rhs,
                    13 => bool_to_u32(u32_to_bool(lhs) && u32_to_bool(rhs)),
                    14 => bool_to_u32(u32_to_bool(lhs) || u32_to_bool(rhs)),
                    15 => bool_to_u32(u32_to_bool(lhs) != u32_to_bool(rhs)),
                    16 => bool_to_u32(lhs == rhs),
                    17 => bool_to_u32(lhs != rhs),
                    18 => bool_to_u32(lhs > rhs),
                    19 => bool_to_u32(lhs < rhs),
                    20 => bool_to_u32(lhs >= rhs),
                    21 => bool_to_u32(lhs <= rhs),
                    _ => panic!("Unknown arithmetic operator {}", op)
                }
            }
        }
    }

    fn insn_calc_signed(&mut self) {
        let out_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((out_reg as usize) < self.registers.len(), "Invalid register {}", out_reg);

        self.registers[out_reg] = self.calc_signed() as u32;
    }

    fn calc_signed(&mut self) -> i32 {
        let op = self.program[self.program_counter];
        self.program_counter += 1;
        let lhs_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((lhs_reg as usize) < self.registers.len(), "Invalid register {}", lhs_reg);

        let lhs = self.registers[lhs_reg] as i32;

        match op {
            22 => -lhs,
            23 => !lhs,
            24 => bool_to_i32(!i32_to_bool(lhs)),
            _ => {
                let rhs_reg = self.program[self.program_counter] as usize;
                self.program_counter += 1;
                assert!((rhs_reg as usize) < self.registers.len(), "Invalid register {}", rhs_reg);

                let rhs = self.registers[rhs_reg] as i32;

                match op {
                    0 => (Wrapping(lhs) + Wrapping(rhs)).0,
                    1 => (Wrapping(lhs) - Wrapping(rhs)).0,
                    2 => (Wrapping(lhs) * Wrapping(rhs)).0,
                    3 => lhs / rhs,
                    4 => lhs % rhs,
                    6 => lhs << rhs,
                    7 => lhs >> rhs,
                    8 => if rhs < 0 { lhs.rotate_right(-rhs as u32) } else { lhs.rotate_left(rhs as u32) },
                    9 => if rhs < 0 { lhs.rotate_left(-rhs as u32) } else { lhs.rotate_right(rhs as u32) },
                    10 => lhs & rhs,
                    11 => lhs | rhs,
                    12 => lhs ^ rhs,
                    13 => bool_to_i32(i32_to_bool(lhs) && i32_to_bool(rhs)),
                    14 => bool_to_i32(i32_to_bool(lhs) || i32_to_bool(rhs)),
                    15 => bool_to_i32(i32_to_bool(lhs) != i32_to_bool(rhs)),
                    16 => bool_to_i32(lhs == rhs),
                    17 => bool_to_i32(lhs != rhs),
                    18 => bool_to_i32(lhs > rhs),
                    19 => bool_to_i32(lhs < rhs),
                    20 => bool_to_i32(lhs >= rhs),
                    21 => bool_to_i32(lhs <= rhs),
                    _ => panic!("Unknown arithmetic operator {}", op)
                }
            }
        }
    }

    // jumps

    fn insn_jump(&mut self) {
        let dest = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        if dest > self.program.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        self.program_counter = dest;
    }

    fn insn_jump_cmpzero(&mut self) {
        let dest = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;
        if dest > self.program.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        let cmp_op = self.program[self.program_counter];
        self.program_counter += 1;

        let lhs_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((lhs_reg as usize) < self.registers.len(), "Invalid register {}", lhs_reg);

        let lhs = self.registers[lhs_reg];

        let should_branch = match cmp_op {
            0 => false,    //  < 0
            1 => lhs == 0, // <= 0
            2 => lhs == 0, // == 0
            3 => true,     // >= 0
            4 => lhs > 0,  //  > 0
            5 => lhs != 0, // != 0
            _ => {
                panic!("Unknown comparison {}", cmp_op)
            }
        };
        if should_branch {
            self.program_counter = dest;
        }
    }

    fn insn_jump_cmpzero_signed(&mut self) {
        let dest = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;
        if dest > self.program.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        let cmp_op = self.program[self.program_counter];
        self.program_counter += 1;

        let lhs_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((lhs_reg as usize) < self.registers.len(), "Invalid register {}", lhs_reg);

        let lhs = self.registers[lhs_reg] as i32;

        let should_branch = match cmp_op {
            0 => lhs < 0,  //  < 0
            1 => lhs <= 0, // <= 0
            2 => lhs == 0, // == 0
            3 => lhs >= 0, // >= 0
            4 => lhs > 0,  //  > 0
            5 => lhs != 0, // != 0
            _ => {
                panic!("Unknown comparison {}", cmp_op)
            }
        };
        if should_branch {
            self.program_counter = dest;
        }
    }

    fn insn_jump_calc(&mut self) {
        let dest = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;
        if dest > self.program.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        let cmp_op = self.program[self.program_counter];
        self.program_counter += 1;

        let lhs = self.calc_unsigned();

        let should_branch = match cmp_op {
            0 => false,    //  < 0
            1 => lhs == 0, // <= 0
            2 => lhs == 0, // == 0
            3 => true,     // >= 0
            4 => lhs > 0,  //  > 0
            5 => lhs != 0, // != 0
            _ => {
                panic!("Unknown comparison {}", cmp_op)
            }
        };
        if should_branch {
            self.program_counter = dest;
        }
    }

    fn insn_jump_calc_signed(&mut self) {
        let dest = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;
        if dest > self.program.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        let cmp_op = self.program[self.program_counter];
        self.program_counter += 1;

        let lhs = self.calc_signed();

        let should_branch = match cmp_op {
            0 => lhs < 0,  //  < 0
            1 => lhs <= 0, // <= 0
            2 => lhs == 0, // == 0
            3 => lhs >= 0, // >= 0
            4 => lhs > 0,  //  > 0
            5 => lhs != 0, // != 0
            _ => {
                panic!("Unknown comparison {}", cmp_op)
            }
        };
        if should_branch {
            self.program_counter = dest;
        }
    }

    fn insn_jump_cmp(&mut self) {
        let dest = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;
        if dest > self.program.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        let cmp_op = self.program[self.program_counter];
        self.program_counter += 1;

        let lhs_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((lhs_reg as usize) < self.registers.len(), "Invalid register {}", lhs_reg);
        let rhs_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((rhs_reg as usize) < self.registers.len(), "Invalid register {}", rhs_reg);

        let lhs = self.registers[lhs_reg];
        let rhs = self.registers[rhs_reg];

        let should_branch = match cmp_op {
            0 => lhs < rhs,
            1 => lhs <= rhs,
            2 => lhs == rhs,
            3 => lhs >= rhs,
            4 => lhs > rhs,
            5 => lhs != rhs,
            _ => {
                panic!("Unknown comparison {}", cmp_op)
            }
        };
        if should_branch {
            self.program_counter = dest;
        }
    }

    fn insn_jump_cmp_signed(&mut self) {
        let dest = to_u32_be(array_ref![self.program, self.program_counter, 4]) as usize;
        self.program_counter += 4;
        if dest > self.program.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        let cmp_op = self.program[self.program_counter];
        self.program_counter += 1;

        let lhs_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((lhs_reg as usize) < self.registers.len(), "Invalid register {}", lhs_reg);
        let rhs_reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        assert!((rhs_reg as usize) < self.registers.len(), "Invalid register {}", rhs_reg);

        let lhs = self.registers[lhs_reg] as i32;
        let rhs = self.registers[rhs_reg] as i32;

        let should_branch = match cmp_op {
            0 => lhs < rhs,
            1 => lhs <= rhs,
            2 => lhs == rhs,
            3 => lhs >= rhs,
            4 => lhs > rhs,
            5 => lhs != rhs,
            _ => {
                panic!("Unknown comparison {}", cmp_op)
            }
        };
        if should_branch {
            self.program_counter = dest;
        }
    }

    // debug

    fn insn_debug(&mut self) {
        println!("Program Counter: {}", self.program_counter);
        println!("Registers: {:?}", self.registers.to_vec());
        println!("Dumping {} bytes of ram", self.memory.len());
        let mut file = File::create("ram_dump");
        match file {
            Err(why) => { println!("couldn't open {}: {}", "ram_dump", why.description()); },
            Ok(mut file) => { file.write_all(self.memory.to_vec().as_slice()); },
        };
    }

    fn insn_debug_reg(&mut self) {
        let reg = self.program[self.program_counter] as usize;
        self.program_counter += 1;
        let value = self.registers[reg];

        println!("R{} = {}, 0x{:x}, {:b}", reg, value, value, value);
    }
}

fn u32_to_bool(i: u32) -> bool {
    i != 0
}

fn bool_to_u32(b: bool) -> u32 {
    if b { 1 } else { 0 }
}

fn i32_to_bool(i: i32) -> bool {
    i != 0
}

fn bool_to_i32(b: bool) -> i32 {
    if b { 1 } else { 0 }
}
