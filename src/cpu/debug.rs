use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use super::*;
use crate::memory::*;
use crate::number_type::NumberType;
use num_traits::AsPrimitive;
use num_traits::PrimInt;

impl CPU {
    pub fn insn_debug(&mut self) {
        println!("Program Counter: {}", self.program_counter);
        println!("Registers: {:?}", self.registers.to_vec());
        println!("Dumping {} bytes of ram", self.memory.data.len());
        let mut file = File::create("ram_dump");
        match file {
            Err(why) => {
                println!("couldn't open {}: {}", "ram_dump", why.description());
            }
            Ok(mut file) => {
                file.write_all(self.memory.data.to_vec().as_slice());
            }
        };
    }

    pub fn insn_debug_value<N: NumberType>(&mut self) {
        let loc = self.read_location();
        let value = loc.get_n::<N>(self);

        println!("{:?} = {}, 0x{:x}, {:b}", loc, value, value, value);
    }
}
