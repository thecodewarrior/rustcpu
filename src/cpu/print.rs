use std::fs::File;
use std::str;
use std::io;
use std::io::prelude::*;

use super::*;
use crate::memory::*;
use crate::number_type::NumberType;
use num_traits::AsPrimitive;
use num_traits::PrimInt;
use failure::Error;

impl CPU {
    pub fn insn_print_block(&mut self) -> Result<(), Error> {
        let source = self.read_location()?;
        let len = self.read_location()?.get_32(self);

        let mut chars: Vec<u8> = Vec::new();
        for i in 0..len {
            chars.push(source.get_offset(self, i)?);
        }
        if chars.last() == Some(&0) {
            chars.pop();
        }

        print!("{}", str::from_utf8(chars.as_slice())?);
        io::stdout().flush()?;

        if source == Location::Rom(self.program_counter as u32) {
            self.program_counter += len as usize;
        }
        Ok(())
    }

    pub fn insn_print_str(&mut self) -> Result<(), Error> {
        let source = self.read_location()?;

        let mut chars: Vec<u8> = Vec::new();
        for i in 0.. {
            let value = source.get_offset(self, i)?;
            if value == 0 {
                break;
            }
            chars.push(value);
        }

        print!("{}", str::from_utf8(chars.as_slice())?);
        io::stdout().flush()?;

        if source == Location::Rom(self.program_counter as u32) {
            self.program_counter += chars.len() + 1;
        }
        Ok(())
    }

    pub fn insn_print_value<N: NumberType>(&mut self) -> Result<(), Error> {
        let loc = self.read_location()?;
        let value = loc.get_n::<N>(self);

        print!("{}", value);
        io::stdout().flush()?;
        Ok(())
    }

    pub fn insn_print_nl(&mut self) -> Result<(), Error> {
        println!();
        Ok(())
    }
}
