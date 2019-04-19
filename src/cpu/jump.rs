use super::*;
use crate::memory::*;
use crate::number_type::NumberType;
use num_traits::PrimInt;

impl CPU {
    pub fn insn_jump(&mut self) {
        let dest = self.read_location().get_32(self) as usize;
        if dest > self.program.data.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        self.program_counter = dest;
    }

    pub fn insn_jump_cmpzero<N: NumberType>(&mut self) {
        let dest = self.read_location().get_32(self) as usize;
        if dest > self.program.data.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        let cmp_op = self.read_8();

        let lhs = self.read_location().get_n::<N>(self);

        let should_branch = match cmp_op {
            0 => lhs < N::zero(),  //  < 0
            1 => lhs <= N::zero(), // <= 0
            2 => lhs == N::zero(), // == 0
            3 => lhs >= N::zero(), // >= 0
            4 => lhs > N::zero(),  //  > 0
            5 => lhs != N::zero(), // != 0
            _ => panic!("Unknown comparison {}", cmp_op),
        };
        if should_branch {
            self.program_counter = dest;
        }
    }

    pub fn insn_jump_calc<N: NumberType>(&mut self) {
        let dest = self.read_location().get_32(self) as usize;
        if dest > self.program.data.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        let cmp_op = self.read_8();

        let lhs = self.calc::<N>();

        let should_branch = match cmp_op {
            0 => lhs < N::zero(),  //  < 0
            1 => lhs <= N::zero(), // <= 0
            2 => lhs == N::zero(), // == 0
            3 => lhs >= N::zero(), // >= 0
            4 => lhs > N::zero(),  //  > 0
            5 => lhs != N::zero(), // != 0
            _ => panic!("Unknown comparison {}", cmp_op),
        };
        if should_branch {
            self.program_counter = dest;
        }
    }

    pub fn insn_jump_cmp<N: NumberType>(&mut self) {
        let dest = self.read_location().get_32(self) as usize;
        if dest > self.program.data.len() {
            panic!("Invalid jump target 0x{:04x}", dest)
        }
        let cmp_op = self.read_8();

        let lhs = self.read_location().get_n::<N>(self);
        let rhs = self.read_location().get_n::<N>(self);

        let should_branch = match cmp_op {
            0 => lhs < rhs,  //  < 0
            1 => lhs <= rhs, // <= 0
            2 => lhs == rhs, // == 0
            3 => lhs >= rhs, // >= 0
            4 => lhs > rhs,  //  > 0
            5 => lhs != rhs, // != 0
            _ => panic!("Unknown comparison {}", cmp_op),
        };
        if should_branch {
            self.program_counter = dest;
        }
    }
}
