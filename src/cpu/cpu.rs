use std::{time, thread};

use crate::memory::*;
use rle_vec::RleVec;
use super::*;

pub struct CPU {
    pub program: Memory,
    pub memory: Memory,
    pub(super) stack_ptr: usize,
    pub(super) program_counter: usize,
    pub(super) registers: [u32; 40]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            program: Memory::new(0),
            memory: Memory::new(2048),
            stack_ptr: 0,
            program_counter: 0,
            registers: [0; 40]
        }
    }

    pub fn run(&mut self) {
        println!("Booting up...");
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        let insn = self.program.get_16(self.program_counter as u32);
//        println!("counter: {}, 0x{:02x}", self.program_counter, insn);
        self.program_counter += 2;
        match insn {
            // instructions
            0x0000 => self.insn_nop(),

            // load: 0x010x
            0x0100 => self.insn_move::<u8>(),
            0x0101 => self.insn_move::<u16>(),
            0x0102 => self.insn_move::<u32>(),

            // math: 0x02xx
            0x0200 => self.insn_calc::<u32>(),
            0x0201 => self.insn_calc::<i32>(),

            // branching: 0x03xx
            0x0300 => self.insn_jump(),
            0x0301 => self.insn_jump_cmpzero::<u32>(),
            0x0302 => self.insn_jump_cmpzero::<i32>(),
            0x0303 => self.insn_jump_calc::<u32>(),
            0x0304 => self.insn_jump_calc::<i32>(),
            0x0305 => self.insn_jump_cmp::<u32>(),
            0x0306 => self.insn_jump_cmp::<i32>(),

            // debug
            0x0f00 => self.insn_debug(),
            0x0f01 => self.insn_debug_value::<u32>(),

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
}
