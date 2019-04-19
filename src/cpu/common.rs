use std::num::Wrapping;

use crate::memory::*;
use super::*;
use crate::number_type::NumberType;
use num_traits::{ PrimInt, WrappingAdd, WrappingMul, WrappingSub };
use std::fmt::Debug;
use std::fmt::Formatter;

/// - `'_____` = Constant
/// - `>_____` = RAM
/// - `>+____` = Stack relative RAM
/// - `%_____` = Register
/// - `@_____` = ROM (program memory)
/// - `0b0000` = '
/// - `0b0001` = >
/// - `0b0010` = >+
/// - `0b0011` = %
/// - `0b0100` = >%
/// - `0b0101` = >+%
/// - `0b0111` = @
/// - `0b1000` = @>
/// - `0b1001` = @>+
/// - `0b1010` = @%
/// - `0b1011` = @>%
/// - `0b1100` = @>+%
#[derive(PartialEq, Eq)]
pub enum Location {
    Constant(u32),
    Ram(u32),
    Rom(u32),
    Register(u32),
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Location::Constant(value) => write!(f, "'{}", value),
            Location::Ram(addr) => write!(f, ">{}", addr),
            Location::Rom(addr) => write!(f, "@{}", addr),
            Location::Register(reg) => write!(f, "%{}", reg),
        }
    }
}

impl Location {
    pub fn get_n<N: NumberType>(&self, cpu: &CPU) -> N {
        match self {
            Location::Constant(value) => N::from_u32(*value),
            Location::Ram(addr) => cpu.memory.get_n(*addr),
            Location::Rom(addr) => cpu.program.get_n(*addr),
            Location::Register(reg) => N::from_u32(cpu.registers[*reg as usize]),
        }
    }

    pub fn set_n<N: NumberType>(&self, cpu: &mut CPU, value: N) {
        match self {
            Location::Constant(value) => panic!("can't set the value of a constant"),
            Location::Ram(addr) => cpu.memory.set_n(*addr, value),
            Location::Rom(addr) => cpu.program.set_n(*addr, value),
            Location::Register(reg) => cpu.registers[*reg as usize] = value.as_u32(),
        }
    }

    pub fn get_8(&self, cpu: &CPU) -> u8 {
        match self {
            Location::Constant(value) => *value as u8,
            Location::Ram(addr) => cpu.memory.get_8(*addr),
            Location::Rom(addr) => cpu.program.get_8(*addr),
            Location::Register(reg) => cpu.registers[*reg as usize] as u8,
        }
    }

    pub fn set_8(&self, cpu: &mut CPU, value: u8) {
        match self {
            Location::Constant(value) => panic!("can't set the value of a constant"),
            Location::Ram(addr) => cpu.memory.set_8(*addr, value),
            Location::Rom(addr) => cpu.program.set_8(*addr, value),
            Location::Register(reg) => cpu.registers[*reg as usize] = value as u32,
        }
    }

    pub fn get_16(&self, cpu: &CPU) -> u16 {
        match self {
            Location::Constant(value) => *value as u16,
            Location::Ram(addr) => cpu.memory.get_16(*addr),
            Location::Rom(addr) => cpu.program.get_16(*addr),
            Location::Register(reg) => cpu.registers[*reg as usize] as u16,
        }
    }

    pub fn set_16(&self, cpu: &mut CPU, value: u16) {
        match self {
            Location::Constant(value) => panic!("can't set the value of a constant"),
            Location::Ram(addr) => cpu.memory.set_16(*addr, value),
            Location::Rom(addr) => cpu.program.set_16(*addr, value),
            Location::Register(reg) => cpu.registers[*reg as usize] = value as u32,
        }
    }

    pub fn get_32(&self, cpu: &CPU) -> u32 {
        match self {
            Location::Constant(value) => *value as u32,
            Location::Ram(addr) => cpu.memory.get_32(*addr),
            Location::Rom(addr) => cpu.program.get_32(*addr),
            Location::Register(reg) => cpu.registers[*reg as usize] as u32,
        }
    }

    pub fn set_32(&self, cpu: &mut CPU, value: u32) {
        match self {
            Location::Constant(value) => panic!("can't set the value of a constant"),
            Location::Ram(addr) => cpu.memory.set_32(*addr, value),
            Location::Rom(addr) => cpu.program.set_32(*addr, value),
            Location::Register(reg) => cpu.registers[*reg as usize] = value,
        }
    }

    pub fn get_offset(&self, cpu: &CPU, offset: u32) -> u8 {
        match self {
            Location::Constant(value) => panic!("Cannot get a constant with an offset"),
            Location::Ram(addr) => cpu.memory.get_8(*addr + offset),
            Location::Rom(addr) => cpu.program.get_8(*addr + offset),
            Location::Register(reg) => panic!("Cannot get a register with an offset"),
        }
    }

    pub fn set_offset(&self, cpu: &mut CPU, offset: u32, value: u8) {
        match self {
            Location::Constant(value) => panic!("can't set the value of a constant"),
            Location::Ram(addr) => cpu.memory.set_8(*addr + offset, value),
            Location::Rom(addr) => cpu.program.set_8(*addr + offset, value),
            Location::Register(reg) => panic!("Cannot set a register with an offset"),
        }
    }
}

impl CPU {
    pub fn read_8(&mut self) -> u8 {
        let value = self.program.get_8(self.program_counter as u32);
        self.program_counter += 1;
        value
    }

    pub fn read_16(&mut self) -> u16 {
        let value = self.program.get_16(self.program_counter as u32);
        self.program_counter += 2;
        value
    }

    pub fn read_32(&mut self) -> u32 {
        let value = self.program.get_32(self.program_counter as u32);
        self.program_counter += 4;
        value
    }

    /// Reads a byte and validates it as a legal register index.
    ///
    /// Increments program counter
    fn read_register(&mut self) -> usize {
        let reg = self.read_8() as usize;
        assert!(reg < self.registers.len(), "Invalid register {}", reg);
        reg
    }

    /// Reads a single value reference from the program
    ///
    pub fn read_location(&mut self) -> Location {
        let loc = self.read_8();
        let stack = self.stack_ptr as u32;
        match loc {
            0b0000 => {
                let value = self.read_32();
                Location::Constant(value)
            }
            0b0001 => {
                let addr = self.read_32();
                Location::Ram(addr)
            }
            0b0010 => {
                let addr = stack + self.read_32();
                Location::Ram(addr)
            }
            0b0011 => {
                let reg = self.read_register() as u32;
                Location::Register(reg)
            }
            0b0100 => {
                let reg = self.read_register();
                let addr = self.registers[reg];
                Location::Ram(addr)
            }
            0b0101 => {
                let reg = self.read_register();
                let addr = stack + self.registers[reg];
                Location::Ram(addr)
            }
            0b0111 => {
                let addr = self.read_32();
                Location::Rom(addr)
            }
            0b1000 => {
                let ram_addr = self.read_32();
                let rom_addr = self.memory.get_32(ram_addr);
                Location::Rom(ram_addr)
            }
            0b1001 => {
                let ram_addr = stack + self.read_32();
                let rom_addr = self.memory.get_32(ram_addr);
                Location::Rom(rom_addr)
            }
            0b1010 => {
                let reg = self.read_register();
                let addr = self.registers[reg];
                Location::Rom(addr)
            }
            0b1011 => {
                let reg = self.read_register();
                let ram_addr = self.registers[reg];
                let rom_addr = self.memory.get_32(ram_addr);
                Location::Rom(rom_addr)
            }
            0b1100 => {
                let reg = self.read_register();
                let ram_addr = stack + self.registers[reg];
                let rom_addr = self.memory.get_32(ram_addr);
                Location::Rom(rom_addr)
            }
            _ => panic!("Unknown location {}", loc)
        }
    }

    pub fn calc<N: NumberType>(&mut self) -> N {
        let op = self.read_8();

        let lhs = self.read_location().get_n::<N>(self);

        match op {
            22 => N::zero().sub(lhs),
            23 => lhs.not(),
            24 => bool_to_num(!num_to_bool(lhs)),
            _ => {

                let rhs = self.read_location().get_n::<N>(self);

                match op {
                    0 => lhs.wrapping_add(&rhs),
                    1 => lhs.wrapping_sub(&rhs),
                    2 => lhs.wrapping_mul(&rhs),
                    3 => lhs.div(rhs),
                    4 => lhs.rem(rhs),
                    6 => lhs.shl(rhs.as_usize()),
                    7 => lhs.shr(rhs.as_usize()),
                    8 => lhs.rotate_left(rhs.as_u32()),
                    9 => lhs.rotate_right(rhs.as_u32()),
                    10 => lhs.bitand(rhs),
                    11 => lhs.bitor(rhs),
                    12 => lhs.bitxor(rhs),
                    13 => bool_to_num(num_to_bool(lhs) && num_to_bool(rhs)),
                    14 => bool_to_num(num_to_bool(lhs) || num_to_bool(rhs)),
                    15 => bool_to_num(num_to_bool(lhs) != num_to_bool(rhs)),
                    16 => bool_to_num(lhs == rhs),
                    17 => bool_to_num(lhs != rhs),
                    18 => bool_to_num(lhs > rhs),
                    19 => bool_to_num(lhs < rhs),
                    20 => bool_to_num(lhs >= rhs),
                    21 => bool_to_num(lhs <= rhs),
                    _ => panic!("Unknown arithmetic operator {}", op)
                }
            }
        }
    }
}

pub fn num_to_bool<N: PrimInt>(i: N) -> bool {
    i != N::zero()
}

pub fn bool_to_num<N: PrimInt>(b: bool) -> N {
    if b { N::one() } else { N::zero() }
}
