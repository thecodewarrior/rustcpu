use std::{thread, time};

use super::*;
use crate::memory::*;
use rle_vec::RleVec;
use failure::Error;
use failure::Fail;

pub struct CPU {
    pub program: Memory,
    pub memory: Memory,
    pub clock: u64,
    pub(super) special_registers: [u32; 16],
    pub(super) program_counter: usize,
    pub(super) registers: [u32; 40],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            program: Memory::new(0),
            memory: Memory::new(2048),
            clock: 10,
            special_registers: [0; 16],
            program_counter: 0,
            registers: [0; 40],
        }
    }

    pub fn run(&mut self) {
        println!("Booting up...");
        loop {
            match self.step() {
                Ok(true) => break,
                Ok(false) => {},
                Err(err) => {
                    println!("Error!\n{:?}", err);
                    break
                }
            }
        }
    }

    fn step(&mut self) -> Result<bool, Error> {
        let insn_loc = self.program_counter;
        let insn = self.program.get_16(self.program_counter as u32);
        //        println!("counter: {}, 0x{:02x}", self.program_counter, insn);
        self.program_counter += 2;

        let insn_result = match insn {
            // instructions
            0x0000 => self.insn_nop(),

            // load: 0x010x
            0x0100 => self.insn_move::<u8>(),
            0x0101 => self.insn_move::<u16>(),
            0x0102 => self.insn_move::<u32>(),
            0x0103 => self.insn_move_block(),
            0x0104 => self.insn_move_str(),

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

            0x0400 => self.insn_print_value::<u32>(),
            0x0401 => self.insn_print_value::<i32>(),
            0x0402 => self.insn_print_block(),
            0x0403 => self.insn_print_str(),
            0x0404 => self.insn_print_nl(),

            0x0500 => self.insn_call(),
            0x0501 => self.insn_return_void(),
            0x0502 => self.insn_return_value(),
            0x0503 => self.insn_push(),
            0x0504 => self.insn_drop(),
            0x0505 => self.insn_pop(),

            // debug
            0x0f00 => self.insn_debug(),
            0x0f01 => self.insn_debug_value::<u32>(),

            // shutdown conditions
            0xffff => {
                println!("Halting...");
                return Ok(true);
            }
            _ => {
                self.program_counter -= 2; // return the PC to its original location
                Err(format_err!("Unknown opcode"))
            }
        };
        match insn_result {
            Ok(_) => {}
            Err(err) => return Err(err.context(format!("Error occured on instruction 0x{:04x} at 0x{:x}", insn, insn_loc)).into())
        }
        thread::sleep(time::Duration::from_millis(1000 / self.clock));
        Ok(false)
    }

    fn insn_nop(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
