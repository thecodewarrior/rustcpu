use num_traits::{PrimInt, WrappingAdd, WrappingMul, WrappingSub};
use std::fmt::{Binary, Debug, Display, LowerHex, Octal, UpperHex};

pub trait NumberType:
    PrimInt
    + WrappingAdd
    + WrappingMul
    + WrappingSub
    + Debug
    + Display
    + LowerHex
    + UpperHex
    + Binary
    + Octal
{
    fn from_isize(value: isize) -> Self;
    fn from_i8(value: i8) -> Self;
    fn from_i16(value: i16) -> Self;
    fn from_i32(value: i32) -> Self;
    fn from_i64(value: i64) -> Self;

    fn from_usize(value: usize) -> Self;
    fn from_u8(value: u8) -> Self;
    fn from_u16(value: u16) -> Self;
    fn from_u32(value: u32) -> Self;
    fn from_u64(value: u64) -> Self;

    fn as_isize(self) -> isize;
    fn as_i8(self) -> i8;
    fn as_i16(self) -> i16;
    fn as_i32(self) -> i32;
    fn as_i64(self) -> i64;

    fn as_usize(self) -> usize;
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
    fn as_u32(self) -> u32;
    fn as_u64(self) -> u64;
}

macro_rules! impl_number_type {
    ($T:ty) => {
        #[allow(deprecated)]
        impl NumberType for $T {
            #[inline]
            fn from_isize(n: isize) -> $T {
                n as $T
            }
            #[inline]
            fn from_i8(n: i8) -> $T {
                n as $T
            }
            #[inline]
            fn from_i16(n: i16) -> $T {
                n as $T
            }
            #[inline]
            fn from_i32(n: i32) -> $T {
                n as $T
            }
            #[inline]
            fn from_i64(n: i64) -> $T {
                n as $T
            }

            #[inline]
            fn from_usize(n: usize) -> $T {
                n as $T
            }
            #[inline]
            fn from_u8(n: u8) -> $T {
                n as $T
            }
            #[inline]
            fn from_u16(n: u16) -> $T {
                n as $T
            }
            #[inline]
            fn from_u32(n: u32) -> $T {
                n as $T
            }
            #[inline]
            fn from_u64(n: u64) -> $T {
                n as $T
            }

            #[inline]
            fn as_isize(self) -> isize {
                self as isize
            }
            #[inline]
            fn as_i8(self) -> i8 {
                self as i8
            }
            #[inline]
            fn as_i16(self) -> i16 {
                self as i16
            }
            #[inline]
            fn as_i32(self) -> i32 {
                self as i32
            }
            #[inline]
            fn as_i64(self) -> i64 {
                self as i64
            }

            #[inline]
            fn as_usize(self) -> usize {
                self as usize
            }
            #[inline]
            fn as_u8(self) -> u8 {
                self as u8
            }
            #[inline]
            fn as_u16(self) -> u16 {
                self as u16
            }
            #[inline]
            fn as_u32(self) -> u32 {
                self as u32
            }
            #[inline]
            fn as_u64(self) -> u64 {
                self as u64
            }
        }
    };
}

impl_number_type!(isize);
impl_number_type!(i8);
impl_number_type!(i16);
impl_number_type!(i32);
impl_number_type!(i64);

impl_number_type!(usize);
impl_number_type!(u8);
impl_number_type!(u16);
impl_number_type!(u32);
impl_number_type!(u64);
