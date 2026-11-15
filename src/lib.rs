//! # STMicroelectronics LSM303DLHC E-Compass Registers
//!
//! This crate provides a typed map of the LSM303DLHC's I²C registers.

#![deny(missing_docs)]
#![no_std]
#![forbid(unsafe_code)]
// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Exports commonly used traits.
pub mod prelude {
    pub use crate::{Register, WritableRegister};
    pub use hardware_registers::i2c::*;
    pub use hardware_registers::sizes::R1;
    pub use hardware_registers::{
        FromBits, HardwareRegister, ToBits, TryFromBits, WritableHardwareRegister,
    };
}

macro_rules! readable_register {
    // Registers in which every bit pattern is valid.
    ($type:ident, $addr:expr) => {
        register_common!($type, $addr);

        impl $crate::prelude::FromBits<u8> for $type {
            #[inline]
            fn from_bits(value: u8) -> Self {
                value.into()
            }

            #[inline]
            fn from_bits_ref(value: &u8) -> Self {
                (*value).into()
            }
        }
    };

    // Registers with reserved bit patterns; `$valid` rejects them.
    ($type:ident, $addr:expr, valid = $valid:expr) => {
        register_common!($type, $addr);

        impl $crate::prelude::TryFromBits<u8> for $type {
            type Error = $crate::ReservedBitPattern;

            #[inline]
            fn try_from_bits_ref(value: &u8) -> Result<Self, Self::Error> {
                let valid: fn(u8) -> bool = $valid;
                if valid(*value) {
                    Ok((*value).into())
                } else {
                    Err($crate::ReservedBitPattern(*value))
                }
            }
        }
    };
}

macro_rules! register_common {
    ($type:ident, $addr:expr) => {
        impl $crate::Register for $type {}
        impl $crate::prelude::HardwareRegister<$crate::prelude::R1> for $type {}

        impl
            $crate::prelude::I2CRegister<
                $crate::prelude::DeviceAddress7,
                $crate::prelude::RegisterAddress8,
                $crate::prelude::R1,
            > for $type
        {
            type Backing = u8;

            const DEFAULT_DEVICE_ADDRESS: $crate::prelude::DeviceAddress7 =
                $crate::prelude::DeviceAddress7::new(DEFAULT_DEVICE_ADDRESS);
            const REGISTER_ADDRESS: $crate::prelude::RegisterAddress8 =
                $crate::prelude::RegisterAddress8::new(($addr).addr());
        }

        impl $crate::prelude::ToBits for $type {
            type Target = u8;

            #[inline]
            fn to_bits(&self) -> Self::Target {
                (*self).into()
            }
        }
    };
}

macro_rules! writable_register {
    ($type:ident, $addr:expr $(, valid = $valid:expr)?) => {
        readable_register!($type, $addr $(, valid = $valid)?);
        impl $crate::prelude::WritableHardwareRegister<$crate::prelude::R1> for $type {}
        impl $crate::WritableRegister for $type {}
    };
}

/// Implements `Add` in both orders on a low/high byte register pair, yielding the `i16` value.
macro_rules! byte_pair_add {
    ($($low:ident + $high:ident),+ $(,)?) => {
        $(
            impl core::ops::Add<$high> for $low {
                type Output = i16;

                fn add(self, hi: $high) -> Self::Output {
                    i16::from_be_bytes([hi.bits(), self.bits()])
                }
            }

            impl core::ops::Add<$low> for $high {
                type Output = i16;

                fn add(self, lo: $low) -> Self::Output {
                    lo + self
                }
            }
        )+
    };
}

pub mod accel;
pub mod mag;

/// A readable sensor register.
///
/// Most registers decode every bit pattern and implement [`FromBits<u8>`](prelude::FromBits).
/// [`ControlRegister1A`](accel::ControlRegister1A) and
/// [`ConfigurationBRegisterM`](mag::ConfigurationBRegisterM) have reserved bit patterns and
/// implement [`TryFromBits<u8>`](prelude::TryFromBits) with [`ReservedBitPattern`] instead.
/// Use [`TryFromBits`](prelude::TryFromBits) to decode any register generically.
pub trait Register:
    prelude::ReadableI2CRegister8<prelude::DeviceAddress7> + From<u8> + Into<u8>
{
}

/// A writable sensor register.
pub trait WritableRegister:
    prelude::WritableI2CRegister8<prelude::DeviceAddress7> + Register
{
}

/// A register value holds a bit pattern that the datasheet marks as reserved.
///
/// Contains the raw register value.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReservedBitPattern(pub u8);

impl core::fmt::Display for ReservedBitPattern {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "reserved register bit pattern {:#010b}", self.0)
    }
}

impl core::error::Error for ReservedBitPattern {}
