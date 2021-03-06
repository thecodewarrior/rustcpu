use std::num::Wrapping;

use super::*;
use crate::memory::*;
use crate::number_type::NumberType;
use num_traits::PrimInt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::u32;
use failure::Error;

/// - `'_____` = Constant
/// - `>_____` = RAM
/// - `^_____` = Frame relative RAM
/// - `%_____` = Register
/// - `@_____` = ROM (program memory)
/// - `::____` = Special
/// - `0b0000` = '
/// - `0b0001` = >
/// - `0b0010` = ^
/// - `0b0011` = %
/// - `0b0100` = >%
/// - `0b0101` = ^%
/// - `0b0111` = @
/// - `0b1000` = @>
/// - `0b1001` = @^
/// - `0b1010` = @%
/// - `0b1011` = @>%
/// - `0b1100` = @^%
/// - `0b1101` = ::
#[derive(PartialEq, Eq)]
pub enum Location {
    Constant(u32),
    Ram(u32),
    Rom(u32),
    Register(u32),
    SpecialRegister(u32),
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Location::Constant(value) => write!(f, "'{}", value),
            Location::Ram(addr) => write!(f, ">{}", addr),
            Location::Rom(addr) => write!(f, "@{}", addr),
            Location::Register(reg) => write!(f, "%{}", reg),
            Location::SpecialRegister(reg) => write!(f, "::{}", reg),
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
            Location::SpecialRegister(reg) => N::from_u32(cpu.special_registers[*reg as usize]),
        }
    }

    pub fn set_n<N: NumberType>(&self, cpu: &mut CPU, value: N) -> Result<(), Error> {
        match self {
            Location::Constant(_) => bail!("Can't set a constant to value"),
            Location::Ram(addr) => Ok(cpu.memory.set_n(*addr, value)),
            Location::Rom(addr) => Ok(cpu.program.set_n(*addr, value)),
            Location::Register(reg) => Ok(cpu.registers[*reg as usize] = value.as_u32()),
            Location::SpecialRegister(reg) => Ok(cpu.special_registers[*reg as usize] = value.as_u32()),
        }
    }

    pub fn get_8(&self, cpu: &CPU) -> u8 {
        match self {
            Location::Constant(value) => *value as u8,
            Location::Ram(addr) => cpu.memory.get_8(*addr),
            Location::Rom(addr) => cpu.program.get_8(*addr),
            Location::Register(reg) => cpu.registers[*reg as usize] as u8,
            Location::SpecialRegister(reg) => cpu.special_registers[*reg as usize] as u8,
        }
    }

    pub fn set_8(&self, cpu: &mut CPU, value: u8) -> Result<(), Error> {
        match self {
            Location::Constant(_) => bail!("Can't set a constant to value"),
            Location::Ram(addr) => Ok(cpu.memory.set_8(*addr, value)),
            Location::Rom(addr) => Ok(cpu.program.set_8(*addr, value)),
            Location::Register(reg) => Ok(cpu.registers[*reg as usize] = value as u32),
            Location::SpecialRegister(reg) => Ok(cpu.special_registers[*reg as usize] = value as u32),
        }
    }

    pub fn get_16(&self, cpu: &CPU) -> u16 {
        match self {
            Location::Constant(value) => *value as u16,
            Location::Ram(addr) => cpu.memory.get_16(*addr),
            Location::Rom(addr) => cpu.program.get_16(*addr),
            Location::Register(reg) => cpu.registers[*reg as usize] as u16,
            Location::SpecialRegister(reg) => cpu.special_registers[*reg as usize] as u16,
        }
    }

    pub fn set_16(&self, cpu: &mut CPU, value: u16) -> Result<(), Error> {
        match self {
            Location::Constant(_) => bail!("Can't set a constant to value"),
            Location::Ram(addr) => Ok(cpu.memory.set_16(*addr, value)),
            Location::Rom(addr) => Ok(cpu.program.set_16(*addr, value)),
            Location::Register(reg) => Ok(cpu.registers[*reg as usize] = value as u32),
            Location::SpecialRegister(reg) => Ok(cpu.special_registers[*reg as usize] = value as u32),
        }
    }

    pub fn get_32(&self, cpu: &CPU) -> u32 {
        match self {
            Location::Constant(value) => *value as u32,
            Location::Ram(addr) => cpu.memory.get_32(*addr),
            Location::Rom(addr) => cpu.program.get_32(*addr),
            Location::Register(reg) => cpu.registers[*reg as usize] as u32,
            Location::SpecialRegister(reg) => cpu.special_registers[*reg as usize] as u32,
        }
    }

    pub fn set_32(&self, cpu: &mut CPU, value: u32) -> Result<(), Error> {
        match self {
            Location::Constant(_) => bail!("Can't set a constant to value"),
            Location::Ram(addr) => Ok(cpu.memory.set_32(*addr, value)),
            Location::Rom(addr) => Ok(cpu.program.set_32(*addr, value)),
            Location::Register(reg) => Ok(cpu.registers[*reg as usize] = value),
            Location::SpecialRegister(reg) => Ok(cpu.special_registers[*reg as usize] = value),
        }
    }

    pub fn get_offset(&self, cpu: &CPU, offset: u32) -> Result<u8, Error> {
        match self {
            Location::Constant(value) => if offset > 3 {
                bail!("can't get a constant with an offset of {}", offset)
            } else { Ok((value >> (24 - 8*offset)) as u8) },
            Location::Ram(addr) => Ok(cpu.memory.get_8(*addr + offset)),
            Location::Rom(addr) => Ok(cpu.program.get_8(*addr + offset)),
            Location::Register(reg) => if offset > 3 {
                bail!("can't get a register with an offset of {}", offset)
            } else { Ok((cpu.registers[*reg as usize] >> (8*offset)) as u8) },
            Location::SpecialRegister(reg) => if offset > 3 {
                bail!("can't get a special register with an offset of {}", offset)
            } else { Ok((cpu.special_registers[*reg as usize] >> (8*offset)) as u8) },
        }
    }

    pub fn set_offset(&self, cpu: &mut CPU, offset: u32, value: u8) -> Result<(), Error> {
        match self {
            Location::Constant(reg) => bail!("Can't set a constant to value"),
            Location::Ram(addr) => Ok(cpu.memory.set_8(*addr + offset, value)),
            Location::Rom(addr) => Ok(cpu.program.set_8(*addr + offset, value)),
            Location::Register(reg) => if offset > 3 {
                bail!("can't set a register with an offset of {}", offset)
            } else {
                cpu.registers[*reg as usize] = cpu.registers[*reg as usize]
                    & !(0b1111_1111u32 << (8*offset)) | ((value as u32) << (8*offset));
                Ok(())
            },
            Location::SpecialRegister(reg) => if offset > 3 {
                bail!("can't set a special register with an offset of {}", offset)
            } else {
                cpu.special_registers[*reg as usize] = cpu.special_registers[*reg as usize]
                    & !(0b1111_1111u32 << (8*offset)) | ((value as u32) << (8*offset));
                Ok(())
            },
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
    fn read_register(&mut self) -> Result<usize, Error> {
        let reg = self.read_8() as usize;
        ensure!(reg < self.registers.len(), "Invalid register {}", reg);
        Ok(reg)
    }

    /// Reads a single value reference from the program
    ///
    pub fn read_location(&mut self) -> Result<Location, Error> {
        let mut loc = self.read_8();
        let wide = loc & 0b1000_000 != 0;
        loc = loc & 0b0111_1111;
        let frame = self.special_registers[SpecialRegister::FrameBase.index()];
        let stack = self.special_registers[SpecialRegister::Stack.index()];
        let mut read_local_8 = || {
            if wide {
                self.read_32() as u8
            } else {
                self.read_8()
            }
        };
        Ok(match loc {
            0b0000 => {
                let value = self.read_32();
                Location::Constant(value)
            }
            0b0001 => {
                let addr = self.read_32();
                Location::Ram(addr)
            }
            0b0010 => {
                let signed_64 = (self.read_32() as i32) as i64;
                let addr = frame as i64 + signed_64;
                if addr < 0 {
                    bail!("negative address {}. Frame is {}, offset is {}.", addr, frame, signed_64)
                }
                if addr > u32::MAX as i64 {
                    bail!("offset address {} is too large. Frame is {}, offset is {}.", addr, frame, signed_64)
                }
                Location::Ram(addr as u32)
            }
            0b0011 => {
                let reg = read_local_8() as u32;
                Location::Register(reg)
            }
            0b0100 => {
                let reg = read_local_8();
                let addr = self.registers[reg as usize];
                Location::Ram(addr)
            }
            0b0101 => {
                let reg = read_local_8();
                let signed_64 = (self.registers[reg as usize] as i32) as i64;
                let addr = (frame as i64 + signed_64);
                if addr < 0 {
                    bail!("negative address {}. Frame is {}, offset is {}.", addr, frame, signed_64)
                }
                if addr > u32::MAX as i64 {
                    bail!("offset address {} is too large. Frame is {}, offset is {}.", addr, frame, signed_64)
                }
                Location::Ram(addr as u32)
            }
            0b0111 => {
                let addr = self.read_32();
                Location::Rom(addr)
            }
            0b1000 => {
                let ram_addr = self.read_32();
                let rom_addr = self.memory.get_32(ram_addr);
                Location::Rom(rom_addr)
            }
            0b1001 => {
                let signed_64 = (self.read_32() as i32) as i64;
                let ram_addr = (frame as i64 + signed_64);
                if ram_addr < 0 {
                    bail!("negative address {}. Frame is {}, offset is {}.", ram_addr, frame, signed_64)
                }
                if ram_addr > u32::MAX as i64 {
                    bail!("offset address {} is too large. Frame is {}, offset is {}.", ram_addr, frame, signed_64)
                }
                let rom_addr = self.memory.get_32(ram_addr as u32);
                Location::Rom(rom_addr)
            }
            0b1010 => {
                let reg = read_local_8();
                let addr = self.registers[reg as usize];
                Location::Rom(addr)
            }
            0b1011 => {
                let reg = read_local_8();
                let ram_addr = self.registers[reg as usize];
                let rom_addr = self.memory.get_32(ram_addr);
                Location::Rom(rom_addr)
            }
            0b1100 => {
                let reg = read_local_8();
                let signed_64 = (self.registers[reg as usize] as i32) as i64;
                let ram_addr = (frame as i64 + signed_64);
                if ram_addr < 0 {
                    bail!("negative address {}. Frame is {}, offset is {}.", ram_addr, frame, signed_64)
                }
                if ram_addr > u32::MAX as i64 {
                    bail!("offset address {} is too large. Frame is {}, offset is {}.", ram_addr, frame, signed_64)
                }
                let rom_addr = self.memory.get_32(ram_addr as u32);
                Location::Rom(rom_addr)
            }
            0b1101 => {
                let reg = read_local_8() as u32;
                Location::SpecialRegister(reg)
            }
            0b1110 => {
                let addr = stack + self.read_32();
                Location::Ram(addr)
            }
            _ => bail!("Unknown location {}", loc),
        })
    }

    pub fn calc<N: NumberType>(&mut self) -> Result<N, Error> {
        let op = self.read_8();

        let lhs = self.read_location()?.get_n::<N>(self);

        Ok(match op {
            22 => N::zero().sub(lhs),
            23 => lhs.not(),
            24 => bool_to_num(!num_to_bool(lhs)),
            _ => {
                let rhs = self.read_location()?.get_n::<N>(self);

                match op {
                    0 => lhs.wrapping_add(&rhs),
                    1 => lhs.wrapping_sub(&rhs),
                    2 => lhs.wrapping_mul(&rhs),
                    3 => if rhs == N::zero() {
                        bail!("Divide by zero")
                    } else {
                        lhs.div(rhs)
                    },
                    4 => if rhs == N::zero() {
                        bail!("Remainder by zero")
                    } else {
                        lhs.rem(rhs)
                    },
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
                    _ => panic!("Unknown arithmetic operator {}", op),
                }
            }
        })
    }
}

pub enum SpecialRegister {
    FrameBase,
    Stack
}

impl SpecialRegister {
    pub fn index(&self) -> usize {
        match self {
            SpecialRegister::FrameBase => 0,
            SpecialRegister::Stack => 1
        }
    }
}

pub fn num_to_bool<N: PrimInt>(i: N) -> bool {
    i != N::zero()
}

pub fn bool_to_num<N: PrimInt>(b: bool) -> N {
    if b {
        N::one()
    } else {
        N::zero()
    }
}
