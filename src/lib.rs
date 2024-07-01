//! # STMicroelectronics LSM303DLHC E-Compass Registers
//!
//! This crate provides a typed map of the LSM303DLHC's I²C registers.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![forbid(unsafe_code)]
// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

use hardware_registers::i2c::DeviceAddress7;

/// Exports commonly used traits.
pub mod prelude {
    pub use crate::{Register, WritableRegister};
    pub use hardware_registers::i2c::*;
    pub use hardware_registers::sizes::R1;
    pub use hardware_registers::{HardwareRegister, WritableHardwareRegister};
}

macro_rules! readable_register {
    ($type:ident) => {
        impl $crate::prelude::HardwareRegister<$crate::prelude::R1> for $type {}

        impl
            $crate::prelude::I2CRegister<
                $crate::prelude::DeviceAddress7,
                $crate::prelude::RegisterAddress8,
                $crate::prelude::R1,
            > for $type
        {
            const DEFAULT_DEVICE_ADDRESS: $crate::prelude::DeviceAddress7 =
                $crate::prelude::DeviceAddress7::new($type::DEV_ADDRESS);
            const REGISTER_ADDRESS: $crate::prelude::RegisterAddress8 =
                $crate::prelude::RegisterAddress8::new($type::REG_ADDRESS);
        }
    };
}

macro_rules! writable_register {
    ($type:ident) => {
        readable_register!($type);
        impl $crate::prelude::WritableHardwareRegister<$crate::prelude::R1> for $type {}
    };
}

pub mod accel;
pub mod mag;

/// A sensor register.
pub trait Register: From<u8> + Into<u8> {
    /// The slave device address.
    const DEV_ADDRESS: u8;

    /// Gets the address of the register.
    const REG_ADDRESS: u8;

    /// Creates a new register instance from bit values.
    fn from_bits(bits: u8) -> Self;

    /// Converts the register instance into bit values.
    fn to_bits(&self) -> u8;
}

/// A writable sensor register.
pub trait WritableRegister: Register {}

impl<R> WritableRegister for R where
    R: hardware_registers::i2c::WritableI2CRegister8<DeviceAddress7> + Register
{
}
