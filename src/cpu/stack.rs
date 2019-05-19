use std::fs::File;
use std::io::prelude::*;

use super::*;
use crate::memory::*;
use crate::number_type::NumberType;
use num_traits::AsPrimitive;
use num_traits::PrimInt;
use failure::Error;

// stack frame when calling:
// |   arguments    |
// |     . . .      |
// |  return insn   | <- old stack, new frame
// |       |        |
// |       |        |
// |       |        |
// | previous frame |
// |       |        |
// |       |        |
// |       |        |
// |                | <- new stack
// stack frame after returning:
// |   arguments    |
// |     . . .      |
// |  return value  |
// |     . . .      |
// |                | <- new stack

impl CPU {
    fn push_stack(&mut self, size: usize) -> Location {
        let loc = Location::Ram(self.special_registers[SpecialRegister::Stack.index()]);
        self.special_registers[SpecialRegister::Stack.index()] += size as u32;
        loc
    }

    fn pop_stack(&mut self, size: usize) -> Location {
        self.special_registers[SpecialRegister::Stack.index()] -= size as u32;
        let loc = Location::Ram(self.special_registers[SpecialRegister::Stack.index()]);
        loc
    }

    pub fn insn_call(&mut self) -> Result<(), Error> {
        let dest = self.read_location()?.get_32(self) as usize;
        if dest > self.program.data.len() {
            bail!("Invalid jump target 0x{:04x}", dest)
        }

        self.push_stack(4).set_32(self, self.program_counter as u32)?;
        self.push_stack(4).set_32(self, self.special_registers[SpecialRegister::FrameBase.index()])?;
        self.special_registers[SpecialRegister::FrameBase.index()]
            = self.special_registers[SpecialRegister::Stack.index()];

        self.program_counter = dest;
        Ok(())
    }

    pub fn insn_return_void(&mut self) -> Result<(), Error> {
        self.special_registers[SpecialRegister::Stack.index()]
            = self.special_registers[SpecialRegister::FrameBase.index()];
        self.special_registers[SpecialRegister::FrameBase.index()] = self.pop_stack(4).get_32(self);
        self.program_counter = self.pop_stack(4).get_32(self) as usize;

        Ok(())
    }

    pub fn insn_return_value(&mut self) -> Result<(), Error> {
        let return_value = self.read_location()?;
        let return_length = self.read_location()?.get_32(self);
        self.special_registers[SpecialRegister::Stack.index()]
            = self.special_registers[SpecialRegister::FrameBase.index()];
        self.special_registers[SpecialRegister::FrameBase.index()] = self.pop_stack(4).get_32(self);
        self.program_counter = self.pop_stack(4).get_32(self) as usize;

        let stack_top = self.push_stack(return_length as usize);
        for i in 0 .. return_length {
            stack_top.set_offset(self, i, return_value.get_offset(self, i)?)?
        }

        Ok(())
    }

    pub fn insn_push(&mut self) -> Result<(), Error> {
        let value = self.read_location()?;
        let length = self.read_location()?.get_32(self);

        let stack_top = self.push_stack(length as usize);
        for i in 0 .. length {
            stack_top.set_offset(self, i, value.get_offset(self, i)?)?
        }

        Ok(())
    }

    pub fn insn_drop(&mut self) -> Result<(), Error> {
        let length = self.read_location()?.get_32(self);

        self.pop_stack(length as usize);

        Ok(())
    }

    pub fn insn_pop(&mut self) -> Result<(), Error> {
        let value = self.read_location()?;
        let length = self.read_location()?.get_32(self);

        let stack_top = self.pop_stack(length as usize);
        for i in 0 .. length {
            value.set_offset(self, i, stack_top.get_offset(self, i)?)?
        }

        Ok(())
    }
}
