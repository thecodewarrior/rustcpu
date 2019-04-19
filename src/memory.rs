#![allow(dead_code)]

use rle_vec::RleVec;
use crate::number_type::NumberType;


/// A sparse block of memory that provides big-endian multi-byte value access
pub struct Memory {
    pub data: RleVec<u8>,
}

impl Memory {
    pub fn new(capacity: usize) -> Memory {
        let mut data: RleVec<u8> = RleVec::new();
        data.push_n(capacity, 0);
        Memory {
            data
        }
    }

    pub fn get_n<N: NumberType>(&self, index: u32) -> N {
        match N::count_zeros(N::zero()) {
            8 => N::from_u8(self.get_8(index)),
            16 => N::from_u16(self.get_16(index)),
            32 => N::from_u32(self.get_32(index)),
            _ => panic!("Unknown bit width {}", N::count_zeros(N::zero()))
        }
    }

    pub fn set_n<N: NumberType>(&mut self, index: u32, value: N) {
        match N::count_zeros(N::zero()) {
            8 => self.set_8(index, value.as_u8()),
            16 => self.set_16(index, value.as_u16()),
            32 => self.set_32(index, value.as_u32()),
            _ => panic!("Unknown bit width {}", N::count_zeros(N::zero()))
        }
    }

    pub fn get_8(&self, index: u32) -> u8 {
        self.data[index as usize]
    }

    pub fn set_8(&mut self, index: u32, value: u8) {
        self.data.set(index as usize, value);
    }

    pub fn get_16(&self, index: u32) -> u16 {
        0 |
            ((self.data[(index+0) as usize] as u16) << 8) |
            ((self.data[(index+1) as usize] as u16) << 0)
    }

    pub fn set_16(&mut self, index: u32, value: u16) {
        self.data.set((index+0) as usize, (value >> 8) as u8);
        self.data.set((index+1) as usize, (value >> 0) as u8);
    }

    pub fn get_32(&self, index: u32) -> u32 {
        0 |
            ((self.data[(index+0) as usize] as u32) << 24) |
            ((self.data[(index+1) as usize] as u32) << 16) |
            ((self.data[(index+2) as usize] as u32) <<  8) |
            ((self.data[(index+3) as usize] as u32) <<  0)
    }

    pub fn set_32(&mut self, index: u32, value: u32) {
        self.data.set((index+0) as usize, (value >> 24) as u8);
        self.data.set((index+1) as usize, (value >> 16) as u8);
        self.data.set((index+2) as usize, (value >>  8) as u8);
        self.data.set((index+3) as usize, (value >>  0) as u8);
    }
}
