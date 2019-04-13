#![allow(dead_code)]

pub fn to_u16_be(bytes: &[u8; 2]) -> u16 {
    0 |
        ((bytes[0] as u16) << 8) |
        ((bytes[1] as u16) << 0)
}

pub fn to_u16_le(bytes: &[u8; 2]) -> u16 {
    0 |
        ((bytes[1] as u16) << 8) |
        ((bytes[0] as u16) << 0)
}

pub fn to_u24_be(bytes: &[u8; 3]) -> u32 {
    0 |
        ((bytes[0] as u32) << 16) |
        ((bytes[1] as u32) <<  8) |
        ((bytes[2] as u32) <<  0)
}

pub fn to_u24_le(bytes: &[u8; 3]) -> u32 {
    0 |
        ((bytes[2] as u32) << 16) |
        ((bytes[1] as u32) <<  8) |
        ((bytes[0] as u32) <<  0)
}

pub fn to_u32_be(bytes: &[u8; 4]) -> u32 {
    0 |
        ((bytes[0] as u32) << 24) |
        ((bytes[1] as u32) << 16) |
        ((bytes[2] as u32) <<  8) |
        ((bytes[3] as u32) <<  0)
}

pub fn to_u32_le(bytes: &[u8; 4]) -> u32 {
    0 |
        ((bytes[3] as u32) << 24) |
        ((bytes[2] as u32) << 16) |
        ((bytes[1] as u32) <<  8) |
        ((bytes[0] as u32) <<  0)
}

pub fn to_u40_be(bytes: &[u8; 5]) -> u64 {
    0 |
        ((bytes[0] as u64) << 32) |
        ((bytes[1] as u64) << 24) |
        ((bytes[2] as u64) << 16) |
        ((bytes[3] as u64) <<  8) |
        ((bytes[4] as u64) <<  0)
}

pub fn to_u40_le(bytes: &[u8; 5]) -> u64 {
    0 |
        ((bytes[4] as u64) << 32) |
        ((bytes[3] as u64) << 24) |
        ((bytes[2] as u64) << 16) |
        ((bytes[1] as u64) <<  8) |
        ((bytes[0] as u64) <<  0)
}

pub fn to_u48_be(bytes: &[u8; 6]) -> u64 {
    0 |
        ((bytes[0] as u64) << 40) |
        ((bytes[1] as u64) << 32) |
        ((bytes[2] as u64) << 24) |
        ((bytes[3] as u64) << 16) |
        ((bytes[4] as u64) <<  8) |
        ((bytes[5] as u64) <<  0)
}

pub fn to_u48_le(bytes: &[u8; 6]) -> u64 {
    0 |
        ((bytes[5] as u64) << 40) |
        ((bytes[4] as u64) << 32) |
        ((bytes[3] as u64) << 24) |
        ((bytes[2] as u64) << 16) |
        ((bytes[1] as u64) <<  8) |
        ((bytes[0] as u64) <<  0)
}

pub fn to_u56_be(bytes: &[u8; 7]) -> u64 {
    0 |
        ((bytes[0] as u64) << 48) |
        ((bytes[1] as u64) << 40) |
        ((bytes[2] as u64) << 32) |
        ((bytes[3] as u64) << 24) |
        ((bytes[4] as u64) << 16) |
        ((bytes[5] as u64) <<  8) |
        ((bytes[6] as u64) <<  0)
}

pub fn to_u56_le(bytes: &[u8; 7]) -> u64 {
    0 |
        ((bytes[6] as u64) << 48) |
        ((bytes[5] as u64) << 40) |
        ((bytes[4] as u64) << 32) |
        ((bytes[3] as u64) << 24) |
        ((bytes[2] as u64) << 16) |
        ((bytes[1] as u64) <<  8) |
        ((bytes[0] as u64) <<  0)
}

pub fn to_u64_be(bytes: &[u8; 8]) -> u64 {
    0 |
        ((bytes[0] as u64) << 56) |
        ((bytes[1] as u64) << 48) |
        ((bytes[2] as u64) << 40) |
        ((bytes[3] as u64) << 32) |
        ((bytes[4] as u64) << 24) |
        ((bytes[5] as u64) << 16) |
        ((bytes[6] as u64) <<  8) |
        ((bytes[7] as u64) <<  0)
}

pub fn to_u64_le(bytes: &[u8; 8]) -> u64 {
    0 |
        ((bytes[7] as u64) << 56) |
        ((bytes[6] as u64) << 48) |
        ((bytes[5] as u64) << 40) |
        ((bytes[4] as u64) << 32) |
        ((bytes[3] as u64) << 24) |
        ((bytes[2] as u64) << 16) |
        ((bytes[1] as u64) <<  8) |
        ((bytes[0] as u64) <<  0)
}
