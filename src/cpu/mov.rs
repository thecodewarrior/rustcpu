use super::*;
use crate::memory::*;
use crate::number_type::NumberType;
use failure::Error;

impl CPU {
    pub fn insn_move<N: NumberType>(&mut self) -> Result<(), Error> {
        let value = self.read_location()?.get_n::<N>(self);
        self.read_location()?.set_n::<N>(self, value)?;
        Ok(())
    }

    pub fn insn_move_block(&mut self) -> Result<(), Error> {
        let source = self.read_location()?;
        let destination = self.read_location()?;
        let len = self.read_location()?.get_32(self);

        for i in 0..len {
            destination.set_offset(self, i, source.get_offset(self, i)?)?;
        }

        if source == Location::Rom(self.program_counter as u32) {
            self.program_counter += len as usize;
        }
        Ok(())
    }

    pub fn insn_move_str(&mut self) -> Result<(), Error> {
        let source = self.read_location()?;
        let destination = self.read_location()?;

        let mut length = 0;
        for i in 0.. {
            let value = source.get_offset(self, i)?;
            destination.set_offset(self, i, value)?;
            length += 1;
            if value == 0 {
                break;
            }
        }

        if source == Location::Rom(self.program_counter as u32) {
            self.program_counter += length;
        }
        Ok(())
    }
}
