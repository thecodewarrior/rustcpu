
use crate::memory::*;
use super::*;
use crate::number_type::NumberType;

impl CPU {
    pub fn insn_move<N: NumberType>(&mut self) {
        let value = self.read_location().get_n::<N>(self);
        self.read_location().set_n::<N>(self, value);
    }

    pub fn insn_move_block(&mut self) {
        let destination = self.read_location();
        let source = self.read_location();
        let len = self.read_location().get_32(self);

        for i in 0..len {
            destination.set_offset(self, i, source.get_offset(self, i));
        }
    }

    pub fn insn_move_str(&mut self) {
        let destination = self.read_location();
        let source = self.read_location();

        for i in 0.. {
            let value = source.get_offset(self, i);
            destination.set_offset(self, i, value);
            if value == 0 {
                break;
            }
        }
    }
}
