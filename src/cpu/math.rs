use super::*;
use crate::memory::*;
use crate::number_type::NumberType;
use failure::Error;

impl CPU {
    pub fn insn_calc<N: NumberType>(&mut self) -> Result<(), Error> {
        let value = self.calc::<N>()?;
        self.read_location()?.set_n::<N>(self, value)?;
        Ok(())
    }
}
